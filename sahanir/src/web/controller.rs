//! Defines the core `Controller` trait.

use axum::Router;

/// A trait that all controllers must implement.
///
/// This trait is used by the framework to retrieve the router
/// associated with a controller. The `#[sahanir_controller]` macro
/// automatically implements this trait for you.
pub trait Controller {
    /// Returns the Axum router for this controller.
    fn get_router() -> Router;
}
