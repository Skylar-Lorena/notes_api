# Rust Note-Taking API

A minimal REST API built using **Rust** and **Actix Web** to store and retrieve text notes.

## Features
- Create and view notes
- Simple JSON-based API
- In-memory storage using `Mutex`
- Built with Rust’s safety and performance guarantees

## Requirements
- **Rust (1.80+)** — Install via [rustup](https://www.rust-lang.org/tools/install)
- **Cargo** (comes with Rust)
- Editor: VS Code + Rust Analyzer

## Setup Instructions

```bash
# Clone this repository
git clone https://github.com/Skylar-Lorena/notes_api
cd notes_api

# Run the app
cargo run
```

Server runs at: `http://127.0.0.1:8080`

## API Endpoints

### ➕ POST /notes
Create a new note.

**Request:**
```json
{
  "id": 1,
  "title": "My first note",
  "content": "Learning Rust is fun!"
}
```

**Response:** `201 Created`

### 📄 GET /notes
Retrieve all notes.

**Response:**
```json
[
  { "id": 1, "title": "My first note", "content": "Learning Rust is fun!" }
]
```

## Common Issues

| Error | Solution |
|-------|----------|
| `Address already in use` | Stop previous instance or use another port |
| `serde not found` | Add serde to Cargo.toml with derive feature |
| `thread '<main>' panicked at 'unwrap'` | Replace with `expect()` or proper error handling |
| `error: linker 'cc' not found` | Install a C compiler (e.g., `gcc`) and ensure it's in your system PATH |

## References
- [Rust Docs](https://doc.rust-lang.org/)
- [Actix Web Docs](https://actix.rs/docs/)
- [Serde Docs](https://serde.rs/)
- [Tokio Docs](https://tokio.rs/)

## Author
Built by **Lorenah M.** for the “New Stack Learning” mini-project using GenAI for setup, debugging, and documentation.
