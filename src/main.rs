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

// Handler for GET /notes
// - `web::Data<AppState>` gives shared access to app state.
// - Locks the mutex to read the notes safely.
// - Returns JSON response containing all notes.
async fn get_notes(data: web::Data<AppState>) -> impl Responder {
    let notes = data.notes.lock().unwrap(); // acquire lock
    HttpResponse::Ok().json(&*notes)        // return JSON array of notes
}

// Handler for POST /notes
// - Accepts a JSON payload representing a new note.
// - Locks the mutex to modify shared notes vector.
// - Pushes the new note into the vector.
// - Returns HTTP 201 Created status.
async fn add_note(note: web::Json<Note>, data: web::Data<AppState>) -> impl Responder {
    let mut notes = data.notes.lock().unwrap();
    notes.push(note.into_inner());
    HttpResponse::Created().finish()
}

