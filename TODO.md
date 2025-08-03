### TODO: Achieve Feature Parity with Go `bubbletea`

This task list outlines the features and semantic differences noted in `docs/API.md` that need to be addressed in the `bubbletea-rs` implementation to align it more closely with the Go `bubbletea` library.

---

### Priority: High

### Priority: Medium

### Priority: Low

#### 4. Enhance `Program::kill()` Semantics

Currently, `Program::kill()` is an alias for `Program::quit()`. To better match the Go library's semantics, `kill` could be made more immediate or forceful.

*   **File(s):**
    *   `src/program.rs`: To modify the `kill` method and event loop.
    *   `src/event.rs`: To potentially add a `KillMsg`.

*   **Tasks:**
    1.  **`TODO: Differentiate KillMsg from QuitMsg`**
        *   **File:** `src/event.rs`
        *   **Instructions:** Introduce a new `KillMsg` struct to distinguish it from a graceful `QuitMsg`.
    2.  **`TODO: Update Program::kill method`**
        *   **File:** `src/program.rs`
        *   **Instructions:** Change `Program::kill()` to send a `KillMsg` instead of a `QuitMsg`.
    3.  **`TODO: Handle KillMsg in the event loop`**
        *   **File:** `src/program.rs`
        *   **Instructions:** In the `Program::run` loop, add a case to handle `KillMsg`. This could trigger an immediate exit by breaking the loop and returning a `ProgramKilled` error, bypassing any further message processing.

#### 5. Implement Environment Variable Configuration for Commands

The Go library has a `WithEnvironment` option to set environment variables for commands, which is missing in the Rust implementation.

*   **File(s):**
    *   `src/program.rs`: For the `ProgramConfig` and `ProgramBuilder`.
    *   `src/command.rs`: To make `exec_process` aware of the environment.

*   **Tasks:**
    1.  **`TODO: Add environment field to ProgramConfig`**
        *   **Go Equivalent:** `tea.WithEnvironment`
        *   **File:** `src/program.rs`
        *   **Instructions:** Add a field like `pub environment: Option<std::collections::HashMap<String, String>>` to `ProgramConfig`.
    2.  **`TODO: Add .with_environment() builder method`**
        *   **File:** `src/program.rs`
        *   **Instructions:** Add a corresponding method to `ProgramBuilder` to set this configuration.
    3.  **`TODO: Update exec_process to use the configured environment`**
        *   **File:** `src/command.rs`
        *   **Instructions:** The `exec_process` function currently takes a `std::process::Command`. It needs to be modified or supplemented with a variant that can access the program's configured environment variables and apply them to the `Command` before execution. This might involve passing the environment from the main `Program` loop down into the command execution context.