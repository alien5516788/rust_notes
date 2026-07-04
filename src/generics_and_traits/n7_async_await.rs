// Rust Async & Tokio Cheat Sheet
// ============================== TODO

/*
    Rust async allows writing concurrent code without threads by using
    Futures, async/await, and async runtimes like Tokio.

    Key points:
        - `async fn` produces a Future (lazy computation)
        - `.await` polls a future to completion, pausing the current task
        - Futures are lazy; they do nothing until polled
        - Tokio (or other runtimes) schedules and polls tasks
        - Use `tokio::spawn` for fire-and-forget or background tasks
        - Async closures exist, but must be called to produce a Future
        - Cooperative multitasking: tasks yield at `.await` points
*/

use tokio::time::{sleep, Duration};

async fn example_task(name: &str) {
    println!("{} started", name);
    sleep(Duration::from_secs(2)).await; // pause here, runtime can schedule other tasks
    println!("{} finished", name);
}

// ==============================
// 1️⃣ Running Async Functions
// ==============================

#[tokio::main] // starts the Tokio runtime
async fn _main() {
    // Sequential execution: await blocks current task
    example_task("A").await; // main halts until A finishes

    // Concurrent execution: create Futures without awaiting immediately
    let t1 = example_task("B");
    let t2 = example_task("C");

    tokio::join!(t1, t2); // polls both concurrently
    println!("Main continues after B and C finish");

    // Background task: fire-and-forget
    tokio::spawn(example_task("D")); // runs independently, main does not wait
    println!("Spawned D in background");

    // Give background task time to finish
    sleep(Duration::from_secs(3)).await;
}

// ==============================
// 2️⃣ Async Closures
// ==============================

fn async_closure_examples() {
    let x = 42;

    // async closure; must call `()` to produce Future
    let closure = async move || {
        println!("Async closure with x = {}", x);
        10
    };

    // spawn closure
    // tokio::spawn(closure()); // note the () to run the closure.  error TODO
}

// ==============================
// 3️⃣ Futures & Lazy Execution
// ==============================

/*
    - async fn and async closures do not run until polled
    - Creating a Future does nothing by itself
    - spawn polls the future in the runtime
    - join! polls multiple futures concurrently
*/

// ==============================
// 4️⃣ Runtime Mechanics (Tokio)
// ==============================

/*
    - Runtime: user-space scheduler inside your process
    - Cooperative multitasking: tasks yield at `.await` points
    - Uses OS primitives:
        - epoll (Linux)
        - kqueue (macOS/BSD)
        - IOCP (Windows)
    - Not a kernel/OS; runs entirely inside the process
    - Can spawn lightweight tasks without creating OS threads
*/

// ==============================
// 5️⃣ Await vs Spawn
// ==============================

/*
    | Feature       | Blocks current task? | Runs independently? |
    |---------------|-------------------|------------------|
    | .await        | Yes               | No               |
    | tokio::spawn  | No                | Yes              |
*/

// ==============================
// 6️⃣ Observing Task State
// ==============================

/*
    - Futures are opaque; runtime owns polling
    - Cannot directly check "started" or "finished"
    - To track progress, use channels (watch/mpsc) to communicate state
*/

/*
    Note on `.await` syntax:
        - `.await` is not an operator, it’s special syntax in Rust.
        - It’s like a built-in language keyword that can be used on any value implementing Future.
        - The dot is intentional — it reads like “on this future, await completion”
*/
