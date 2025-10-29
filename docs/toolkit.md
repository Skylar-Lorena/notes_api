# Rust Note-Taking API — Toolkit Document

## 1) Title & Objective
**Tech:** Rust + Actix Web  
**Objective:** Build a minimal REST API for notes to learn Rust’s backend ecosystem and document learning with GenAI.

## 2) Quick Summary of the Technology
**Rust:** Systems language emphasizing performance, safety (no GC), and fearless concurrency.  
**Actix Web:** High-performance async web framework for Rust.  
**Real-world usage:** Discord, Cloudflare, AWS teams use Rust for perf-critical services.

## 3) System Requirements
- OS: Windows/macOS/Linux
- Rust: 1.90+ (via rustup)
- Cargo: ships with Rust
- Editor: VS Code + Rust Analyzer
- Deps: actix-web, serde, serde_json, tokio

## 4) Installation & Setup
```bash
curl https://sh.rustup.rs -sSf | sh
rustc --version && cargo --version

cargo new note_api
cd note_api
# Replace Cargo.toml and src/main.rs with repo versions
cargo run
# Open http://127.0.0.1:8080
```

## 5) Minimal Working Example
Endpoints:
- `GET /notes` → list all notes
- `POST /notes` → create a note `{ id, title, content }`

Expected output:
- Start: `[]`  
- After POST: returns `201`, then GET shows your note in the array.

## 6) AI Prompt Journal (with refined prompts)
See `docs/ai-prompt-journal.md` for full prompts, response summaries, and reflections. A quick snapshot:

| Step | Prompt Theme | Key Takeaways |
|-----:|--------------|---------------|
| 1 | Conceptual understanding (JS/React → Rust) | Ownership/borrowing vs GC, zero-cost abstractions, compile-time guarantees |
| 2 | Deep-dive concept (Concurrency & async) | `async/await`, executors, `Send/Sync`, comparing to JS event loop |
| 3 | Guided implementation (CRUD route) | Handlers, extractors, shared state with `Mutex`, JSON (serde) |
| 4 | Verification | Replace `unwrap` with error handling, validate payloads, consider IDs server-generated |

## 7) Common Issues & Fixes
| Issue | Cause | Fix |
|------|-------|-----|
| Address in use | Previous instance bound to 8080 | Stop it or change port |
| Borrow checker errors | Moving values unexpectedly | Clone or borrow immutably (`&T`), rethink lifetimes |
| Panics from `unwrap()` | Unhandled errors | Use `?`, `map_err`, or `expect()` with context |
| Serde derive missing | Cargo.toml feature not set | `serde = { version="1.0", features=["derive"] }` |

## 8) References
- Rust Book, Actix Web docs, Serde, Tokio (links in README)
