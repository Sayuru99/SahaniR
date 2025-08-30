use sahanir::prelude::*;

/// The controller for the root path of the application.
pub struct HomeController;

#[sahanir_controller("/")]
impl HomeController {
    #[get("/")]
    async fn index() -> &'static str {
        "Welcome to the SahaniR Universe!"
    }
}
