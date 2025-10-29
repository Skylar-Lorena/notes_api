# AI Prompt Journal — Rust Note-Taking API

> **Source → Target:** JavaScript/React → Rust/Actix Web

This journal logs prompts, summarized responses, and reflections following the 4-step learning flow.

---

## Step 1: Conceptual Understanding

**Prompt (use as-is):**
```
I'm currently proficient in JavaScript/React and want to learn Rust for backend APIs with Actix Web.
Before diving into code:
1) What are the key philosophical differences between JavaScript and Rust?
2) What problems was Rust designed to solve?
3) What mental models should I adjust coming from JavaScript?
4) What are common misconceptions JavaScript developers have about Rust?
```

**Response Summary (concise):**
- **Philosophy:** JS prioritizes flexibility, dynamic typing, and rapid iteration; Rust prioritizes correctness, performance, and safety at compile time.  
- **Problems solved:** Memory safety without GC, data races eliminated by the borrow checker, predictable performance for systems and services.  
- **Mental models to adjust:** Move semantics, ownership/borrowing, lifetimes, `Result`-based error handling instead of exceptions, explicit concurrency (`Send`/`Sync`).  
- **Common misconceptions:** “Rust is only for low-level systems”; “Rust can’t be productive”; “Borrow checker is the enemy” — it’s a guide to safe design.

**Reflection:**
Coming from JS, I must plan for types and lifetimes up-front. Rust’s compiler feedback replaces a lot of runtime debugging. I expect slower initial dev speed but better reliability.

---

## Step 2: Step-by-Step Breakdown (Concept Focus)

**Concept chosen:** Async & Concurrency in Rust APIs

**Prompt:**
```
I want to understand async & concurrency in Rust. Could you break down:
1) How async/await is implemented in Rust (executors, tasks, futures)
2) How it compares to the event loop & promises in JavaScript
3) The key syntax/traits (`Future`, `Send`, `Sync`) I need to understand
4) Common patterns and best practices in Actix Web
Let's not write complex code yet—focus on structure and concepts.
```

**Response Summary:**
- **Rust async:** `async fn` returns `impl Future`; executors (Tokio) poll futures; tasks can be spawned.  
- **Compare to JS:** Both are event-driven; Rust is zero-cost abstractions with explicit executors; JS hides the runtime.  
- **Key traits:** `Send`/`Sync` for thread-safety; `Future` as a state machine.  
- **Patterns:** Use Actix handlers as async fns, share state via `Data<T>` + `Mutex`/`RwLock`, avoid blocking work on async threads.

**Reflection:**
I should avoid blocking I/O and be explicit about shared state. `Send`/`Sync` help me reason about multi-threaded safety, unlike JS’s single-threaded model.

---

## Step 3: Guided Implementation

**Prompt:**
```
I'm ready to implement my first CRUD endpoint in Rust with Actix Web.
Guide me to create a simple POST /notes and GET /notes using shared in-memory state.
Explain each part of the syntax, especially how it differs from JavaScript's approach.
```

**Response Summary:**
- Define `Note` with `serde` derives.  
- Use `App::new()` with `app_data(Data<AppState>)`.  
- Handlers: `web::Json<T>` for JSON, return `HttpResponse`.  
- Shared state: `Mutex<Vec<Note>>` for simple demo; consider DB later.  
- Differences from JS: explicit types, ownership of payloads, thread-safe state wrappers.

**Reflection:**
The extractor system is elegant and type-safe. I need to manage cloning/borrowing carefully when pushing to the shared vector.

---

## Step 4: Understanding Verification

**Prompt Template (paste after you implement):**
```
I've created this Rust/Actix implementation:

[YOUR CODE HERE]

Could you:
1) Verify if I've followed Rust best practices?
2) Explain any improvements I should make (error handling, types, state)?
3) Suggest what I should learn next (testing, database, auth)?
4) Point out any JavaScript habits showing in my code?
```

**Verification Checklist:**
- Avoid `unwrap()` in handlers; prefer `?` or explicit error mapping.  
- Validate request payload (e.g., non-empty titles).  
- Generate IDs server-side (counter/UUID) rather than trusting client `id`.  
- Consider `RwLock` for read-heavy patterns.  
- Add integration tests with `actix_web::test`.  
- Structure modules for growth (`routes`, `models`, `state`).

**Next Topics:**
- Testing in Actix Web, error types with `thiserror`/`anyhow`, logging (`tracing`), persistence (SQLite with `sqlx`), and pagination.

---

## Log Table 

| Date | Step | Prompt (title) | Response Summary | Reflection / What I changed |
|------|------|-----------------|------------------|-----------------------------|
| 2025-10-27 | 1 | Conceptual Understanding | Rust vs JS philosophy; ownership; misconceptions | Rewrote handlers to avoid `unwrap` |
| 2025-10-29 | 2 | Async & Concurrency | Futures, executors, Send/Sync | Stopped blocking calls on async threads |
| 2025-10-30 | 3 | Guided Implementation | POST/GET notes | Switched to server-generated IDs |
| 2025-10-30 | 4 | Verification | Review and improvements | Added tests & better errors |
