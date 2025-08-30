//! The core concepts of the SahaniR framework, inspired by cosmology.

use std::any::Any;

/// Represents a `PocketUniverse`, an isolated module within the application.
///
/// In SahaniR, an application is composed of one or more "universes" that are
/// isolated from each other, ensuring that faults or security breaches in one
/// do not affect others. This trait is the foundation for creating such modules.
pub trait PocketUniverse {
    /// Returns the unique name of this universe.
    fn name(&self) -> &'static str;

    /// Configures the services, controllers, and other components of this universe.
    /// This method is called by the Orchestrator during application startup.
    fn configure(&self) -> UniverseConfiguration;
}

/// Configuration for a `PocketUniverse`.
///
/// This struct is returned by the `configure` method of a `PocketUniverse` and
/// is used by the `Orchestrator` to set up the universe.
#[derive(Default)]
pub struct UniverseConfiguration {
    /// A list of controllers to be registered for this universe.
    pub controllers: Vec<Box<dyn Any>>,
}

/// The `Spacetime` orchestrator, responsible for managing the application's lifecycle.
///
/// Spacetime is the fabric of your application. It discovers and manages all the
/// `PocketUniverse`s, handles incoming requests, and routes them to the appropriate
// universe.
pub struct Orchestrator {
    universes: Vec<Box<dyn PocketUniverse>>,
}

impl Orchestrator {
    /// Creates a new `Orchestrator`.
    pub fn new() -> Self {
        Self {
            universes: Vec::new(),
        }
    }

    /// Registers a `PocketUniverse` with the orchestrator.
    pub fn aether(mut self, universe: impl PocketUniverse + 'static) -> Self {
        self.universes.push(Box::new(universe));
        self
    }

    /// Starts the application, listening for incoming events (e.g., web requests).
    pub async fn ignite(self) {
        println!("The Orchestrator is igniting...");
        for universe in &self.universes {
            println!("  - Configuring universe: '{}'", universe.name());
            let config = universe.configure();
            if !config.controllers.is_empty() {
                println!("    - Registered {} controller(s)", config.controllers.len());
            }
        }
        println!("Spacetime is now stable. Application is running.");
        // In a real scenario, a web server would be started here.
    }
}
