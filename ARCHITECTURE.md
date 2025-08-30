# Architecture

This document outlines the architecture of the improvie application, which is
built using Tauri. It consists of a Rust backend for core logic and a SvelteKit
frontend for the user interface. The project adopts a modern, modular approach
in both its backend and frontend to ensure maintainability, scalability, and
type safety.

## Backend Architecture (Rust)

The backend is structured as a multi-crate Rust workspace. It employs a layered
architecture that separates concerns and promotes modularity, a design that is
heavily influenced by Clean Architecture principles.

### Layered Structure

The architecture is divided into the following main layers:

1.  **Domain + Logic Layer (`improvie-domain`, `improvie-logic`)**: This is the
    core of the application. It defines business rules, data structures (entities),
    and pure logic that is independent of any specific framework or technology.
2.  **Infrastructure Layer (`improvie-infra`)**: This layer implements technical
    details like database access and external API communication. It provides
    concrete implementations for the interfaces (traits) defined in the Domain layer.
3.  **Application Layer (`improvie-app`)**: This layer contains application-specific
    use cases. It orchestrates the Domain and Infrastructure layers to realize
    concrete features, such as creating a folder.
4.  **Command Layer (`improvie-command`)**: This acts as the interface layer,
    exposing the Application layer's functionality to the frontend through
    Tauri commands.

### Crate Responsibilities

The `src-tauri` directory is a Cargo workspace with the following key crates:

-   **`improvie` (Root Crate)**: The main entry point for the Tauri application.
    It initializes Tauri, sets up plugins (logging, dialogs), manages the
    application state (`AppState`), and registers all commands from `improvie-command`.
-   **`improvie-command`**: Defines all `#[tauri::command]` functions invoked by the
    frontend. It receives requests, calls the appropriate use cases in
    `improvie-app`, and returns the results.
-   **`improvie-app`**: Implements the application's use cases (e.g., `ItemsUseCase`).
    It contains the primary application logic, coordinating data flow between the
    domain and infrastructure layers.
-   **`improvie-logic`**: Defines application-wide, framework-agnostic data models
    (`Content`, `Folder`), constants, and shared error types.
-   **`improvie-domain`**: Defines repository interfaces (traits) like `ItemsRepository`
    using the Repository Pattern. This decouples application logic from the
    database implementation.
-   **`improvie-infra`**: Provides concrete implementations for the repository traits
    from `improvie-domain`. It uses SQLite as the database and SeaORM as the
    asynchronous ORM.
-   **`addtional/*`**: A collection of utility crates:
    -   **`uid`**: A wrapper for handling ULIDs, used as primary keys.
    -   **`youtube`**: Handles downloading and merging YouTube video/audio via
        `reqwest` and `ez-ffmpeg`.
    -   **`bind`**: A procedural macro to simplify using `ts-rs` for auto-generating
        TypeScript types from Rust structs.

### Data Flow Example (Creating a Folder)

1.  **UI**: A user clicks the "Create Folder" button.
2.  **Frontend (JS/TS)**: A TypeScript function calls Tauri's `invoke('create_folder', { ... })`.
3.  **`improvie-command`**: The `create_folder` command function is executed.
4.  **`improvie-app`**: The command calls the `ItemsUseCase::create_folder` method.
5.  **`improvie-infra`**: The use case calls `ItemsRepository::create_folder`. The
    implementation in this crate then uses SeaORM to issue `INSERT` statements
    to SQLite within a transaction.
6.  **Database**: A new folder record is written to the SQLite database.
7.  **Return**: The result is passed back through the layers to the frontend,
    which updates the UI.

### Key Backend Technologies

-   **Tauri**: Core application framework.
-   **Tokio**: Asynchronous runtime.
-   **SeaORM**: Asynchronous ORM for SQLite.
-   **Serde**: Data serialization/deserialization (Rust <-> JSON).
-   **ts-rs**: Generates TypeScript definitions from Rust types for type-safe
    communication.

---

## Frontend Architecture (SvelteKit)

The frontend is a modern, well-structured application built with SvelteKit and
TypeScript. The UI is crafted with shadcn-svelte and Tailwind CSS, achieving a
balance between aesthetics and customizability.

### Key Frontend Technologies

-   **Framework**: SvelteKit provides file-system based routing, lifecycle
    management, and a powerful development experience.
-   **UI Components**: shadcn-svelte, which uses bits-ui (a headless component
    library) and is styled with Tailwind CSS.
-   **Language**: TypeScript is used throughout for type safety.
-   **Styling**: Tailwind CSS for a utility-first CSS workflow.
-   **Form Handling**: Libraries like `svelte-formify` are used for declarative
    form management and validation.

### Directory Structure

-   `src/routes`: Defines the application's pages and layouts based on file and
    directory names.
    -   `+page.svelte`: The main component for a specific route.
    -   `+layout.svelte`: A component that defines a shared layout for a set of routes.
    -   `+layout.ts`: A module to load data for a layout and its child routes
        before they are rendered.
-   `src/lib`: Contains shared, reusable code.
    -   `lib/components/ui`: Basic, reusable UI components managed by `shadcn-svelte`.
    -   `lib/features`: More complex components composed from basic UI elements to
        provide specific features (e.g., the sidebar, media player).
    -   `lib/stores`: Global state management using Svelte's built-in stores
        (`writable`, `readable`).
    -   `lib/action`: A dedicated layer that encapsulates calls to the Rust backend,
        providing a clean API for UI components.
    -   `src/bindings`: **Crucially**, this directory contains TypeScript type
        definitions automatically generated from the Rust backend.

### Backend (Rust) Integration

The frontend and backend are tightly integrated, with a strong emphasis on type safety.

-   **Command Invocation**: The frontend uses the `invoke` function from
    `@tauri-apps/api/core` to call commands exposed by the Rust backend. These
    calls are abstracted away in the `src/lib/action` modules.
-   **Type-Safe Bridge**: The `src/bindings` directory is the cornerstone of
    frontend/backend robustness. Because these TypeScript types are generated
    from the Rust source, any breaking change on the backend will immediately
    cause a TypeScript compilation error on the frontend. This eliminates a
    whole class of runtime errors.
-   **Event Listening**: The frontend uses Tauri's `listen` function to subscribe
    to events from the backend. This enables real-time UI updates for
    long-running processes like YouTube video downloads.