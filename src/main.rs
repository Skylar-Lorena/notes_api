// Import key modules from the Actix Web framework.
// - `web` for request handlers and shared data.
// - `App` for defining the web app and routes.
// - `HttpResponse` for returning responses.
// - `HttpServer` for running the server instance.
// - `Responder` trait allows handler functions to return valid HTTP responses.
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// Import serialization/deserialization macros from Serde.
// These let us convert structs <-> JSON automatically.
use serde::{Deserialize, Serialize};

// Import `Mutex` from the standard library to manage shared, thread-safe mutable state.
use std::sync::Mutex;

// Define the Note struct, representing a single note entity.
// - `Serialize` / `Deserialize` let us convert to/from JSON.
// - `Clone` allows duplication when needed (e.g., pushing to a vector).
#[derive(Serialize, Deserialize, Clone)]
struct Note {
    id: usize,
    title: String,
    content: String,
}

// Define a struct for global application state.
// This holds a vector of notes wrapped in a `Mutex` to allow
// concurrent access across async request handlers safely.
struct AppState {
    notes: Mutex<Vec<Note>>,
}

