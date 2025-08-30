use sahanir::prelude::*;

/// The controller for the root path of the application.
#[sahanir_controller("/")]
pub struct HomeController;

// In the future, this controller would have methods for handling
// GET, POST, etc. requests. For example:
//
// impl HomeController {
//     #[get("/")]
//     pub fn index() -> &'static str {
//         "Welcome to the SahaniR Universe!"
//     }
// }
