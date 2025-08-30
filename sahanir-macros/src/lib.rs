use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ItemImpl, Lit, Meta};

#[proc_macro_attribute]
pub fn sahanir_controller(args: TokenStream, input: TokenStream) -> TokenStream {
    // The `args` TokenStream is parsed into a `syn::Meta` structure.
    let args_meta = parse_macro_input!(args as Meta);
    let mut impl_block = parse_macro_input!(input as ItemImpl);

    // Get the base path from the controller attribute, e.g., #[sahanir_controller("/users")]
    let base_path = match args_meta {
        Meta::List(list) => match list.nested.first() {
            Some(syn::NestedMeta::Lit(Lit::Str(lit))) => lit.value(),
            _ => "".to_string(),
        },
        _ => "".to_string(),
    };

    let mut route_handlers = Vec::new();

    // Iterate over the items in the impl block (methods, etc.)
    for item in &mut impl_block.items {
        if let ImplItem::Fn(method) = item {
            // Find the first SahaniR route attribute on the method
            if let Some(route_attr_index) = method.attrs.iter().position(|attr| {
                let path = attr.path();
                path.is_ident("get")
                    || path.is_ident("post")
                    || path.is_ident("put")
                    || path.is_ident("delete")
            }) {
                let route_attr = &method.attrs[route_attr_index];
                let http_method = route_attr.path().get_ident().unwrap().clone();

                // Parse the route path from the attribute's metadata
                let route_path = match &route_attr.meta {
                    Meta::List(meta_list) => match meta_list.nested.first() {
                        Some(syn::NestedMeta::Lit(Lit::Str(lit))) => lit.value(),
                        _ => panic!("Route attribute must have a path string, e.g., #[get(\"/\")]"),
                    },
                    _ => panic!("Invalid route attribute format."),
                };

                let method_name = &method.sig.ident;
                route_handlers.push((http_method, route_path, method_name.clone()));

                // Remove the attribute so it doesn't cause a compile error
                method.attrs.remove(route_attr_index);
            }
        }
    }

    // Generate the routing code for each handler
    let routes = route_handlers.iter().map(|(http_method, path, method_name)| {
        quote! {
            .route(#path, axum::routing::#http_method(Self::#method_name))
        }
    });

    let router_code = quote! {
        axum::Router::new()
            #(#routes)*
    };

    // Generate the new `__get_router__` method
    let get_router_method = quote! {
        pub fn __get_router__() -> axum::Router {
            axum::Router::new().nest(#base_path, #router_code)
        }
    };

    // Add the new method to the impl block
    let new_method: ImplItem = syn::parse2(get_router_method).unwrap();
    impl_block.items.push(new_method);

    let struct_name = &impl_block.self_ty;

    // Generate the `impl Controller` block that calls our new method
    let impl_controller_trait = quote! {
        impl sahanir::web::controller::Controller for #struct_name {
            fn get_router() -> axum::Router {
                Self::__get_router__()
            }
        }
    };

    // Combine the original (modified) impl block with the new trait impl
    let output = quote! {
        #impl_block
        #impl_controller_trait
    };

    output.into()
}

// These are now just marker attributes. The main controller macro does all the work.
#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    input
}
#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    input
}
#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    input
}
#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    input
}
