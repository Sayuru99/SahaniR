//! The core concepts of the SahaniR framework, inspired by cosmology.

use std::net::SocketAddr;
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Represents a `PocketUniverse`, an isolated module within the application.
pub trait PocketUniverse {
    /// Returns the unique name of this universe.
    fn name(&self) -> &'static str;

    /// Configures the services, controllers, and other components of this universe.
    fn configure(&self) -> UniverseConfiguration;
}

/// Configuration for a `PocketUniverse`.
#[derive(Default)]
pub struct UniverseConfiguration {
    /// A list of Axum routers from the controllers in this universe.
    pub controllers: Vec<Router>,
}

/// The `Spacetime` orchestrator, responsible for managing the application's lifecycle.
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

    /// Starts the application, collecting routes and running the web server.
    pub async fn ignite(self) {
        // Initialize tracing for logging.
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "sahanir=debug,tower_http=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();

        tracing::debug!("The Orchestrator is igniting...");
        let mut app_router = Router::new();

        // Collect routers from all registered universes.
        for universe in &self.universes {
            tracing::debug!("  - Configuring universe: '{}'", universe.name());
            let config = universe.configure();
            if !config.controllers.is_empty() {
                tracing::debug!("    - Merging {} router(s)", config.controllers.len());
                for controller_router in config.controllers {
                    app_router = app_router.merge(controller_router);
                }
            }
        }

        // Add final middleware layers.
        let app = app_router.layer(TraceLayer::new_for_http());

        // Define the server address.
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::debug!("Spacetime is stable. Server listening on {}", addr);

        // Start the server.
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
