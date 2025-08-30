use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Item};

/// A procedural macro to define a controller in the SahaniR framework.
///
/// A controller is a struct responsible for handling incoming web requests
/// for a specific path.
///
/// # Example
///
/// ```ignore
/// #[sahanir_controller("/users")]
/// struct UserController;
/// ```
#[proc_macro_attribute]
pub fn sahanir_controller(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the attribute arguments, e.g., the "/users" path.
    let _args = parse_macro_input!(args as AttributeArgs);

    // Parse the input item the attribute is attached to (e.g., the struct).
    let input_item = parse_macro_input!(input as Item);

    // For now, the macro doesn't modify the input item.
    // It just returns the original item's token stream.
    // In the future, this will be expanded to generate routing logic.
    let output = quote! {
        #input_item
    };

    output.into()
}
