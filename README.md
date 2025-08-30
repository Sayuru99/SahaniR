# SahaniR Framework

*Version: 1.0.0 (MVP)*

---

## Introduction

**SahaniR** is a next-generation backend framework built in Rust, designed to empower developers in creating secure, scalable, and performant applications. Drawing inspiration from cosmological principles—such as pocket universes from inflationary theory and spacetime from general relativity—SahaniR reimagines backend architecture as a dynamic, interconnected cosmos. It combines the modular elegance of NestJS with the robust, type-safe ecosystem of Rust, while surpassing frameworks like NestJS (Node.js) and ASP.NET (C#) in speed, memory efficiency, and built-in quantum-resistant security.

Named after Sahani—celebrating a 9-year milestone on March 22, 2026—and "R" for Rust, SahaniR embodies enduring strength and efficiency. Our vision: To create a framework that is not just a tool, but a paradigm shift.

## Workspace Structure

This repository contains the core components of the SahaniR framework in a single Cargo workspace:

-   `sahanir/`: The core framework library. This crate contains the foundational concepts like the `Orchestrator`, the `PocketUniverse` trait, and other core components.
-   `sahanir-cli/`: The command-line interface for SahaniR. This tool helps you create and manage your SahaniR applications.
-   `sahanir-macros/`: A crate containing the procedural macros for the framework, such as `#[sahanir_controller]`.

## Getting Started (MVP)

This is the Minimum Viable Product (MVP) of the SahaniR framework. The core architectural stubs and the project scaffolding CLI are in place.

**To test the CLI and generate a new project:**

1.  **Build the CLI:**
    Navigate to the root of this workspace and run the following command to build the CLI tool. (Note: You will need Rust and Cargo installed.)
    ```bash
    cargo build --package sahanir-cli
    ```

2.  **Run the CLI:**
    Once built, you can run the CLI to scaffold a new "Hello, Universe!" application.
    ```bash
    ./target/debug/sahanir-cli new my-first-universe
    ```

3.  **Explore the New Project:**
    This will create a new directory named `my-first-universe` with a runnable SahaniR application. You can explore its structure and see how the `PocketUniverse` trait is implemented.

## Development Roadmap

This MVP represents the completion of the foundational phase. The next phases of development will focus on:

-   **Phase 2 (Q4 2025):** Implementing the advanced concepts from the design document (e.g., `TearingHoles`, `Portals`), adding asynchronous ML integration, and defining security surfaces.
-   **Phase 3 (Q1 2026):** Full microservices support, comprehensive benchmarks, and the first community release.

---

*This project is dedicated to Sahani, marking 9 years together on March 22, 2026.*
