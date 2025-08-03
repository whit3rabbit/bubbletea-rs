Based on the provided API documentation for `bubbletea-rs`, the following features are present in the Rust implementation but are missing from or differ significantly in the standard Go `bubbletea` library.

Below is a TODO/task list detailing these missing features, with explanations of what needs to be implemented and in which corresponding Go files this logic would likely reside.

***

### TODO/Task List for Go Implementation

The following tasks are based on features available in the `bubbletea-rs` Rust implementation that are not standard in the Go `bubbletea` core library.

#### 1. Implement Memory Monitoring Utilities

The Rust implementation includes a dedicated module for monitoring resource usage, which is not present in the Go core library. This provides valuable diagnostics for application health.

*   **File:** `memory/memory.go` (New Package)

*   **Tasks:**
    1.  **`TODO: Create MemoryMonitor struct`**
        *   **Rust Equivalent:** `struct MemoryMonitor` in `src/memory.rs`.
        *   **Explanation:** Implement a `MemoryMonitor` struct using `sync/atomic` counters to track runtime metrics. It should contain fields for `activeTimers`, `activeTasks`, `channelDepth`, `messagesProcessed`, and `peakMemoryBytes`.

    2.  **`TODO: Implement MemorySnapshot and MemoryHealth structs`**
        *   **Rust Equivalents:** `struct MemorySnapshot`, `struct MemoryHealth` in `src/memory.rs`.
        *   **Explanation:** Create structs to represent a point-in-time snapshot of metrics (`MemorySnapshot`) and a health check result (`MemoryHealth`) that includes a list of potential issues.

    3.  **`TODO: Implement MemoryMonitor methods`**
        *   **Rust Equivalents:** `fn snapshot()`, `fn check_health()` in `src/memory.rs`.
        *   **Explanation:** Add methods to the `MemoryMonitor` to take a `snapshot()` of the current metrics and to `check_health()`, which analyzes the snapshot for potential issues like high timer counts or channel backlogs.

    4.  **`TODO: Add WithMemoryMonitoring program option`**
        *   **Rust Equivalent:** `ProgramConfig.memory_monitoring` in `docs/API.md`.
        *   **Explanation:** In the `program.go` or `options.go` file, add a new `tea.ProgramOption` called `WithMemoryMonitoring()`. When this option is used, the `tea.Program` should initialize and continuously update the `MemoryMonitor` instance.

#### 2. Add Gradient and ANSI Color Utilities

The Rust implementation provides built-in utilities for creating gradient text, which is useful for rich UI styling but is not part of the Go core.

*   **File:** `gradient/gradient.go` (New Package)

*   **Tasks:**
    1.  **`TODO: Implement RGB linear interpolation`**
        *   **Rust Equivalent:** `fn lerp_rgb()` in `src/gradient.rs`.
        *   **Explanation:** Create a `LerpRGB` function that calculates an intermediate RGB color between two given colors based on a factor `t` (from 0.0 to 1.0).

    2.  **`TODO: Implement GradientFilledSegment function`**
        *   **Rust Equivalent:** `fn gradient_filled_segment()` in `src/gradient.rs`.
        *   **Explanation:** Implement a function `GradientFilledSegment(text string, startColor, endColor color.Color) string` that takes a string and returns a new string with ANSI escape codes to render the text with a smooth color gradient. This would likely rely on a lipgloss-style renderer.

#### 3. Implement Explicit Timer Cancellation Commands

The Rust implementation offers commands to explicitly cancel timers by an ID, a feature not directly available in Go, which typically relies on contexts or channels for cancellation.

*   **File:** `tea.go` or `command.go`

*   **Tasks:**
    1.  **`TODO: Create EveryWithID command`**
        *   **Rust Equivalent:** `fn every_with_id()` in `docs/API.md`.
        *   **Explanation:** Implement a new command constructor, `EveryWithID(d time.Duration, fn func(time.Time) tea.Msg) (tea.Cmd, int)`, that returns both a command and a unique timer ID. The program runtime will need to track these IDs.

    2.  **`TODO: Create CancelTimer command`**
        *   **Rust Equivalent:** `fn cancel_timer(id: u64)` in `docs/API.md`.
        *   **Explanation:** Implement `CancelTimer(timerID int) tea.Cmd`. This command will send a message to the program runtime, instructing it to find and stop the ticker associated with the given `timerID`.

    3.  **`TODO: Create CancelAllTimers command`**
        *   **Rust Equivalent:** `fn cancel_all_timers()` in `docs/API.md`.
        *   **Explanation:** Implement `CancelAllTimers() tea.Cmd`. This command signals the runtime to stop all active timers that were created with `EveryWithID`.

#### 4. Add Advanced Program Configuration Options

The `ProgramConfig` in Rust includes features that could be ported to Go as new `ProgramOption`s.

*   **File:** `program.go` or `options.go`

*   **Tasks:**
    1.  **`TODO: Implement WithPanicCatching option`**
        *   **Rust Equivalent:** `ProgramConfig.catch_panics` in `docs/API.md`.
        *   **Explanation:** Create a `WithPanicCatching()` program option. When enabled, the program's event loop should wrap calls to the model's `Update` and `View` methods with a `recover()` block. If a panic is caught, it should be converted into an `error` and returned by `program.Run()`, preventing the application from crashing.

#### 5. Add a Simple Logging Helper

The Rust implementation provides a convenient, feature-gated logging utility that is useful for debugging TUI applications without corrupting the display.

*   **File:** `log/log.go` (New Package)

*   **Tasks:**
    1.  **`TODO: Create LogToFile function`**
        *   **Rust Equivalent:** `fn log_to_file()` in `docs/API.md`.
        *   **Explanation:** Implement a helper function `LogToFile(path string) error`. This function would configure the default Go `log` package to write its output to the specified file. This is a common pattern for TUI development, as `fmt.Println` or direct stderr writes interfere with the rendered view.