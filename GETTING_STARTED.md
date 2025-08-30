# Getting Started with SahaniR

This guide provides instructions on how to set up your development environment, build the framework's tools, and create your first SahaniR application.

## Prerequisites

Before you begin, ensure you have the Rust programming language and its package manager, Cargo, installed on your system. You can install them by following the official instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

## Building the Framework

This repository is a Cargo workspace containing the entire SahaniR framework. To build the project, including the CLI tool, you can run the following command from the root of the repository:

```bash
# This will build all crates in the workspace
cargo build
```

## Creating Your First Application

The easiest way to start a new project is by using the SahaniR Command-Line Interface (CLI).

1.  **Build the CLI:**
    If you haven't already, build the CLI tool specifically. This ensures you have the latest version ready to use.
    ```bash
    cargo build --package sahanir-cli
    ```

2.  **Run the CLI to Generate a New Project:**
    Execute the CLI binary from the workspace's target directory to scaffold a new "Hello, Universe!" application.
    ```bash
    ./target/debug/sahanir-cli new my-first-universe
    ```
    This command will create a new directory named `my-first-universe` in your current location.

3.  **Explore the New Project:**
    The generated directory contains a complete, runnable SahaniR application.
    ```
    my-first-universe/
    ├── Cargo.toml
    └── src/
        ├── main.rs
        └── controllers.rs
    ```
    You can explore its structure to see how the `PocketUniverse` trait is implemented and how the `HomeController` is defined.

### Defining Your First Controller

The core of a SahaniR web application is its controllers. Open `src/controllers.rs` in your new project to see an example:

```rust
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
```

Key concepts here are:
-   **`#[sahanir_controller("/")]`**: This macro is placed on an `impl` block and defines a new controller. The `"/` argument is the base path for all routes within this controller.
-   **`#[get("/")]`**: This macro defines a route that responds to HTTP GET requests. The path specified is relative to the controller's base path. You can also use `#[post(...)]`, `#[put(...)]`, and `#[delete(...)]`.
-   The function (`index`) is the handler for this route. It's a standard `async` function that returns a type that can be converted into a response.

4.  **Running Your Application:**
    Navigate into your new project's directory and run it using Cargo.
    ```bash
    cd my-first-universe
    cargo run
    ```
    You will see log output from the SahaniR Orchestrator as it configures and ignites your application's "universe".
