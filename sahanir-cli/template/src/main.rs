use sahanir::prelude::*;

mod controllers;
use controllers::HomeController;

/// The root universe of our application.
/// Every SahaniR application has at least one "universe" module.
struct RootUniverse;

impl PocketUniverse for RootUniverse {
    fn name(&self) -> &'static str {
        "root"
    }

    fn configure(&self) -> UniverseConfiguration {
        UniverseConfiguration {
            controllers: vec![HomeController::get_router()],
        }
    }
}


#[tokio::main]
async fn main() {
    println!("Booting the {{ project_name }} universe...");

    // The Orchestrator manages the application's lifecycle.
    // We register our "universes" (modules) with it.
    Orchestrator::new()
        .aether(RootUniverse)
        .ignite()
        .await;
}
