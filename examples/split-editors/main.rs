//! Split Editors Example
//!
//! A faithful port of the Go Bubble Tea split-editors example, demonstrating:
//! - Multiple side-by-side textarea editors with line numbers
//! - Purple cursor line highlighting with white text
//! - End-of-buffer markers ("-" for empty lines)
//! - Dynamic focus management between editors
//! - Adding/removing editors (min 1, max 6)
//! - Key bindings for navigation and control
//! - Styled borders that change based on focus state
//! - Help system showing available commands
//! - Dynamic resizing based on terminal width
//!
//! Key bindings:
//! - Tab: Focus next editor
//! - Shift+Tab: Focus previous editor
//! - Ctrl+N: Add new editor (up to 6)
//! - Ctrl+W: Remove current editor (minimum 1)
//! - Esc/Ctrl+C: Quit

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
use bubbletea_widgets::help::{KeyMap as HelpKeyMap, Model as HelpModel};
use bubbletea_widgets::key::{
    matches_binding, new_binding, with_help, with_keys_str, Binding, KeyMap,
};
use crossterm::event::KeyCode;
use crossterm::terminal;
use lipgloss_extras::lipgloss::{
    hidden_border, join_horizontal, rounded_border, Color, Style, TOP,
};

// Constants matching the Go version
const INITIAL_INPUTS: usize = 2;
const MAX_INPUTS: usize = 6;
const MIN_INPUTS: usize = 1;
const HELP_HEIGHT: i32 = 5; // Space for help text at bottom

// Synthetic message used to trigger the initial render immediately after startup.
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

// ================================================================================================
// STYLING FUNCTIONS - Understanding Terminal Colors and ANSI Escape Codes
// ================================================================================================
//
// These functions define the visual appearance of our text editors using the lipgloss library.
// Under the hood, lipgloss generates ANSI escape codes that tell your terminal how to color and
// style text. For example:
// - "\x1b[48;5;57m" sets background to purple (color 57)
// - "\x1b[38;5;230m" sets foreground to light yellow (color 230)
// - "\x1b[0m" resets all formatting (this is crucial to understand!)

/// Creates the purple background style for the cursor line (active line in focused editor)
///
/// Key insight: The `color_whitespace(true)` method is critical for full-width backgrounds!
/// Without it, the background color only appears behind visible characters, not spaces.
/// With it, the entire line gets the purple background, creating the effect seen in
/// professional editors like VS Code.
fn cursor_line_style() -> Style {
    Style::new()
        .background(Color::from("57")) // Purple background (ANSI color 57)
        .foreground(Color::from("230")) // Light yellow text (ANSI color 230)
        .color_whitespace(true) // KEY: Apply background to spaces/tabs for full-width effect
}

/// Placeholder text styling when textarea is focused but empty
fn focused_placeholder_style() -> Style {
    Style::new().foreground(Color::from("99")) // Bright blue-gray for visibility
}

/// Border style for the currently focused editor
/// Uses a rounded border to provide visual feedback about which editor is active
fn focused_border_style() -> Style {
    Style::new()
        .border(rounded_border()) // Visible rounded corners
        .border_foreground(Color::from("238")) // Gray border color
        .padding(0, 1, 0, 1) // Internal spacing: top, right, bottom, left
}

/// Border style for inactive (blurred) editors  
/// Uses a hidden border but maintains the same padding for consistent layout
fn blurred_border_style() -> Style {
    Style::new()
        .border(hidden_border()) // Invisible border (no visual feedback)
        .padding(0, 1, 0, 1) // Same padding as focused to prevent layout shift
}

// ================================================================================================
// CUSTOM TEXTAREA IMPLEMENTATION - Why We Built Our Own
// ================================================================================================
//
// We implemented a custom TextArea instead of using bubbletea-widgets::textarea because:
// 1. We need precise control over line number rendering and styling
// 2. We need to implement the complex cursor line highlighting (purple background)
// 3. We need to show end-of-buffer markers ("~") like Vim/professional editors
// 4. The existing textarea widget doesn't expose enough styling hooks for our needs
//
// This demonstrates how to build custom components when existing widgets don't fit your requirements.

/// Custom TextArea that matches the Go Bubble Tea split-editors example
///
/// Features:
/// - Line numbers with proper alignment
/// - Full-width cursor line highlighting (purple background)  
/// - End-of-buffer markers ("~") for empty lines
/// - Cursor positioning with bright highlighting
/// - Focus management for multiple editors
#[derive(Debug, Clone)]
struct TextArea {
    lines: Vec<String>,  // The actual text content, one string per line
    cursor_line: usize,  // Which line the cursor is currently on (0-based)
    cursor_col: usize,   // ABSOLUTE cursor column position (0-based, can exceed visible width)
    width: usize,        // Width of the content area (excluding borders/padding)
    height: usize,       // Height of the visible area (number of lines to show)
    focused: bool,       // Whether this textarea currently has focus
    placeholder: String, // Text to show when empty and not focused

    // ====== HORIZONTAL SCROLLING SYSTEM ======
    // The key to implementing cursor-following horizontal scrolling:
    //
    // Concept: We maintain a "sliding window" over the full line content.
    //
    //   Full line: "This is a very long line that extends beyond the viewport width"
    //   Viewport:           [----visible window----]  <- width=20 chars
    //   Offset:             ^                         <- horizontal_offset=12
    //
    // The cursor_col is always absolute (position in full line), but we show only
    // the portion from horizontal_offset to (horizontal_offset + width).
    horizontal_offset: usize, // First visible column index (implements the sliding window)
}

impl TextArea {
    fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_line: 0,
            cursor_col: 0,
            width: 40,
            height: 10,
            focused: false,
            placeholder: "Type something".to_string(),
            horizontal_offset: 0,
        }
    }

    fn set_width(&mut self, width: usize) {
        // Account for: borders (2) + padding (2) = 4 for lipgloss
        // Line numbers (3) + " │ " (3) = 6 for our internal formatting
        // Total: 10 characters overhead
        self.width = width.saturating_sub(10).max(15); // Larger minimum width
    }

    fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn blur(&mut self) {
        self.focused = false;
    }

    fn is_empty(&self) -> bool {
        self.lines.len() == 1 && self.lines[0].is_empty()
    }

    /// Updates horizontal scroll offset to keep cursor visible
    ///
    /// This is the heart of the cursor-following horizontal scrolling system.
    /// It implements a "smart viewport" that automatically slides left/right to ensure
    /// the cursor always remains visible with comfortable margins.
    ///
    /// ## Algorithm Explanation:
    ///
    /// ```
    /// Scenario: width=20, cursor_col=25, horizontal_offset=10
    ///
    ///   Full line: "The quick brown fox jumps over the lazy dog"
    ///   Positions:  0123456789012345678901234567890123456789012
    ///   Viewport:             [----visible (10-29)----]
    ///   Cursor:                           ^ (at pos 25)
    ///
    /// Since cursor (25) is near right edge (29), we scroll right:
    ///   New offset: 25 + 2 - 20 + 1 = 8
    ///   New viewport:     [----visible (8-27)----]  
    ///   Cursor position:                 ^ (now has 2-char margin on right)
    /// ```
    ///
    /// ## Why Margins Matter:
    /// Without margins, scrolling would happen at the very edge, making it hard to see
    /// context. With 2-character margins, users can see what's coming next/before.
    fn update_horizontal_scroll(&mut self) {
        const SCROLL_MARGIN: usize = 2; // Keep some chars visible on both sides for context

        // Calculate the rightmost visible column (exclusive)
        let visible_end = self.horizontal_offset + self.width;

        // SCROLL LEFT: If cursor is too close to left edge (or beyond it)
        if self.cursor_col < self.horizontal_offset + SCROLL_MARGIN {
            // Move viewport left to show cursor with left margin
            // Example: cursor=5, offset=10, margin=2 -> cursor < 12, so scroll left
            self.horizontal_offset = self.cursor_col.saturating_sub(SCROLL_MARGIN);
        }
        // SCROLL RIGHT: If cursor is too close to right edge (or beyond it)
        else if self.cursor_col >= visible_end.saturating_sub(SCROLL_MARGIN) {
            // Move viewport right to show cursor with right margin
            // Formula: new_offset = cursor + margin - width + 1
            // Example: cursor=25, margin=2, width=20 -> offset = 25+2-20+1 = 8
            self.horizontal_offset = (self.cursor_col + SCROLL_MARGIN)
                .saturating_sub(self.width)
                .saturating_add(1);
        }

        // If neither condition is met, cursor is comfortably visible - no scroll needed
    }

    /// Insert a character at the current cursor position
    fn insert_char(&mut self, c: char) {
        if self.cursor_line >= self.lines.len() {
            self.lines.resize(self.cursor_line + 1, String::new());
        }

        let line = &mut self.lines[self.cursor_line];
        if self.cursor_col > line.len() {
            self.cursor_col = line.len();
        }

        line.insert(self.cursor_col, c);
        self.cursor_col += 1;
        self.update_horizontal_scroll(); // Keep cursor visible when typing
    }

    /// Insert a newline at the current cursor position
    fn insert_newline(&mut self) {
        if self.cursor_line >= self.lines.len() {
            self.lines.resize(self.cursor_line + 1, String::new());
        }

        let line = &mut self.lines[self.cursor_line];
        let remaining = line.split_off(self.cursor_col);
        self.lines.insert(self.cursor_line + 1, remaining);
        self.cursor_line += 1;
        self.cursor_col = 0;

        // SCROLLING: Reset to show beginning of new line
        // When user presses Enter, they expect to see the start of the new line,
        // not to maintain the previous line's scroll position.
        self.horizontal_offset = 0;
    }

    /// Delete character before cursor (backspace)
    fn backspace(&mut self) {
        if self.cursor_col > 0 {
            // Delete character in current line
            if self.cursor_line < self.lines.len() {
                self.lines[self.cursor_line].remove(self.cursor_col - 1);
                self.cursor_col -= 1;
                self.update_horizontal_scroll(); // Keep cursor visible after deletion
            }
        } else if self.cursor_line > 0 {
            // Merge with previous line
            let current_line = self.lines.remove(self.cursor_line);
            self.cursor_line -= 1;
            self.cursor_col = self.lines[self.cursor_line].len();
            self.lines[self.cursor_line].push_str(&current_line);
            self.update_horizontal_scroll(); // Keep cursor visible after merge
        }
    }

    /// Delete character at cursor position
    fn delete_char(&mut self) {
        if self.cursor_line < self.lines.len() {
            let line = &mut self.lines[self.cursor_line];
            if self.cursor_col < line.len() {
                line.remove(self.cursor_col);
            } else if self.cursor_line + 1 < self.lines.len() {
                // Merge with next line
                let next_line = self.lines.remove(self.cursor_line + 1);
                self.lines[self.cursor_line].push_str(&next_line);
            }
        }
    }

    /// Move cursor up
    fn cursor_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            let line_len = if self.cursor_line < self.lines.len() {
                self.lines[self.cursor_line].len()
            } else {
                0
            };
            self.cursor_col = self.cursor_col.min(line_len);

            // SCROLLING: Reset when changing lines vertically
            // Different lines may have different lengths and content, so it's cleaner
            // to start fresh rather than maintain horizontal scroll across lines.
            // This matches behavior of most editors (VS Code, Vim, etc.).
            self.horizontal_offset = 0;
        }
    }

    /// Move cursor down
    fn cursor_down(&mut self) {
        if self.cursor_line + 1 < self.lines.len()
            || (self.cursor_line + 1 == self.lines.len()
                && !self.lines[self.cursor_line].is_empty())
        {
            self.cursor_line += 1;
            if self.cursor_line >= self.lines.len() {
                self.lines.push(String::new());
            }
            let line_len = self.lines[self.cursor_line].len();
            self.cursor_col = self.cursor_col.min(line_len);

            // SCROLLING: Reset when changing lines vertically (same rationale as cursor_up)
            self.horizontal_offset = 0;
        }
    }

    /// Move cursor left
    fn cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
            // SCROLLING: Update scroll position to keep cursor visible
            // If cursor moves off the left edge, viewport will slide left
            self.update_horizontal_scroll();
        } else if self.cursor_line > 0 {
            // Wrap to end of previous line
            self.cursor_line -= 1;
            self.cursor_col = if self.cursor_line < self.lines.len() {
                self.lines[self.cursor_line].len()
            } else {
                0
            };
            // SCROLLING: Update for potentially long previous line
            // Cursor may now be at end of a long line, requiring right scroll
            self.update_horizontal_scroll();
        }
    }

    /// Move cursor right
    fn cursor_right(&mut self) {
        if self.cursor_line < self.lines.len() {
            let line_len = self.lines[self.cursor_line].len();
            if self.cursor_col < line_len {
                self.cursor_col += 1;
                // SCROLLING: Update scroll position to keep cursor visible
                // If cursor moves off the right edge, viewport will slide right
                self.update_horizontal_scroll();
            } else if self.cursor_line + 1 < self.lines.len() {
                // Wrap to start of next line
                self.cursor_line += 1;
                self.cursor_col = 0;
                // SCROLLING: Reset to show beginning of next line (fresh start)
                self.horizontal_offset = 0;
            }
        }
    }

    /// Handle key input for the textarea
    fn handle_key(&mut self, key: &KeyMsg) {
        if !self.focused {
            return;
        }

        match key.key {
            KeyCode::Char(c) => self.insert_char(c),
            KeyCode::Enter => self.insert_newline(),
            KeyCode::Backspace => self.backspace(),
            KeyCode::Delete => self.delete_char(),
            KeyCode::Up => self.cursor_up(),
            KeyCode::Down => self.cursor_down(),
            KeyCode::Left => self.cursor_left(),
            KeyCode::Right => self.cursor_right(),
            _ => {}
        }
    }

    /// Render the textarea content without borders (borders will be applied externally)
    fn view(&self) -> String {
        let mut lines = Vec::new();
        let max_lines = self.height;

        // Show placeholder if empty and not focused
        let display_lines = if self.is_empty() && !self.focused {
            vec![self.placeholder.clone()]
        } else {
            self.lines.clone()
        };

        for line_idx in 0..max_lines {
            let mut line_content = String::new();

            // Line number (always 3 chars wide)
            let (line_num_style, line_num_text) = if line_idx < display_lines.len() {
                // Real line number
                let style = if self.focused && line_idx == self.cursor_line {
                    cursor_line_style()
                } else {
                    Style::new().foreground(Color::from("238"))
                };
                let text = format!("{:>3}", line_idx + 1);
                (style, text)
            } else {
                // End-of-buffer marker
                let style = Style::new().foreground(Color::from("235")); // Dim gray for ~
                let text = "  ~".to_string();
                (style, text)
            };

            line_content.push_str(&line_num_style.render(&line_num_text));

            // CRITICAL: Style the line number separator to complete the full-width effect
            // The " │ " separator must also get the purple background when it's part of the cursor line,
            // otherwise you'll see a gap in the purple highlighting between line numbers and content.
            let separator = if self.focused && line_idx == self.cursor_line {
                cursor_line_style().render(" │ ") // Purple background on separator too
            } else {
                " │ ".to_string() // Normal separator for non-cursor lines
            };
            line_content.push_str(&separator);

            // ========================================================================
            // LINE CONTENT RENDERING WITH HORIZONTAL SCROLLING
            // ========================================================================
            //
            // This is where we implement the "sliding window" that makes cursor-following
            // horizontal scrolling work. We extract only the visible portion of each line.

            if line_idx < display_lines.len() {
                let line = &display_lines[line_idx];

                // COORDINATE TRANSFORMATION: Convert from full line to visible window
                //
                // Example:
                //   Full line: "The quick brown fox jumps over the lazy dog" (43 chars)
                //   Offset: 10, Width: 20
                //
                //   Full:     "The quick brown fox jumps over the lazy dog"
                //   Indices:   01234567890123456789012345678901234567890123
                //   Window:              [----visible (10-29)----]
                //   Extract:              "brown fox jumps ov"
                let content_text = if line.len() > self.horizontal_offset {
                    // Extract the visible slice: [offset..offset+width] (bounded by line length)
                    let start = self.horizontal_offset;
                    let end = (start + self.width).min(line.len());
                    &line[start..end]
                } else {
                    // Edge case: offset is beyond line end (e.g., scrolled past end of short line)
                    // Show empty content rather than panicking
                    ""
                };

                // ============================================================================
                // CURSOR LINE RENDERING - The ANSI Reset Problem and Solution
                // ============================================================================
                //
                // PROBLEM: When you nest styled strings (call .render() on content that already
                // contains ANSI escape codes), the inner styled content includes reset codes (\x1b[0m)
                // that cancel ALL formatting, including the parent style's background color.
                //
                // EXAMPLE OF THE PROBLEM:
                // ```
                // let cursor = Style::new().background("212").render("█");  // Contains: "█\x1b[0m"
                // let line = Style::new().background("57").render(format!("text{}more", cursor));
                // // Result: "text" gets purple background, "█" gets bright background,
                // //         but "more" has NO background because \x1b[0m reset it!
                // ```
                //
                // SOLUTION: Style each part separately, then concatenate the results.
                // This way, each part gets its own complete ANSI sequence without interfering reset codes.

                let styled_content = if self.focused && line_idx == self.cursor_line {
                    // ========================================================================
                    // STEP-BY-STEP CURSOR LINE RENDERING (No Nested Styling!)
                    // ========================================================================

                    let chars: Vec<char> = content_text.chars().collect();

                    // CRITICAL COORDINATE TRANSFORMATION: Absolute → Visible cursor position
                    //
                    // The cursor_col is always absolute (position in full line), but we're now
                    // working with content_text which is only the visible portion of the line.
                    // We must convert the absolute cursor position to a relative position within
                    // the visible window.
                    //
                    // Example:
                    //   Full line: "The quick brown fox jumps over"  (cursor_col = 15)
                    //   Offset: 10, so visible: "brown fox jumps"
                    //   Absolute cursor pos: 15 (in full line)
                    //   Visible cursor pos:  15 - 10 = 5 (in visible portion)
                    //
                    //   Result: cursor shows after "brow|n" (position 5 in visible text)
                    let visible_cursor_pos = self.cursor_col.saturating_sub(self.horizontal_offset);
                    let cursor_pos = visible_cursor_pos.min(chars.len());

                    // STEP 1: Split the line into separate parts (no styling yet!)
                    // This allows us to style each part independently without nesting
                    let before_cursor = if cursor_pos > 0 {
                        chars[..cursor_pos].iter().collect::<String>()
                    } else {
                        String::new()
                    };

                    let cursor_char = if cursor_pos < chars.len() {
                        chars[cursor_pos].to_string()
                    } else {
                        " ".to_string() // Cursor at end of line shows as space
                    };

                    let after_cursor = if cursor_pos < chars.len() {
                        chars[cursor_pos + 1..].iter().collect::<String>()
                    } else {
                        String::new()
                    };

                    // STEP 2: Calculate padding needed to fill the full line width
                    // Account for content length + cursor (if at end) and pad to full width
                    let visible_content_len =
                        chars.len() + if cursor_pos >= chars.len() { 1 } else { 0 };
                    let padding = " ".repeat(self.width.saturating_sub(visible_content_len));

                    // STEP 3: Style each part separately (this is the key insight!)
                    // Each .render() call produces a complete ANSI sequence: start + content + reset
                    let styled_before = if !before_cursor.is_empty() {
                        cursor_line_style().render(&before_cursor)
                    } else {
                        String::new()
                    };

                    // The cursor gets special treatment: purple line style + bright magenta override
                    let styled_cursor = cursor_line_style()
                        .background(Color::from("212")) // Bright magenta overrides the purple (212 = bright magenta)
                        .foreground(Color::from("0")) // Black text on bright background for contrast
                        .render(&cursor_char);

                    let styled_after = if !after_cursor.is_empty() {
                        cursor_line_style().render(&after_cursor)
                    } else {
                        String::new()
                    };

                    let styled_padding = if !padding.is_empty() {
                        cursor_line_style().render(&padding)
                    } else {
                        String::new()
                    };

                    // STEP 4: Concatenate the styled parts (no further .render() calls!)
                    // Each part is already a complete ANSI sequence, so we just join them together.
                    // Result: Full-width purple background with bright cursor - just like VS Code!
                    format!(
                        "{}{}{}{}",
                        styled_before, styled_cursor, styled_after, styled_padding
                    )
                } else {
                    // Normal line styling
                    let padded_line = format!("{:<width$}", content_text, width = self.width);
                    if self.is_empty() && !self.focused {
                        focused_placeholder_style().render(&padded_line)
                    } else {
                        padded_line
                    }
                };

                line_content.push_str(&styled_content);
            } else {
                // End of buffer - just empty space since ~ is in line number area
                let padding = " ".repeat(self.width);
                line_content.push_str(&padding);
            }

            lines.push(line_content);
        }

        lines.join("\n")
    }
}

/// Creates a new textarea with default settings
fn new_textarea() -> TextArea {
    TextArea::new()
}

/// Keymap defines the key bindings for the application
#[derive(Debug, Clone)]
struct Keymap {
    next: Binding,
    prev: Binding,
    add: Binding,
    remove: Binding,
    quit: Binding,
}

impl Keymap {
    fn new() -> Self {
        Self {
            next: new_binding(vec![with_keys_str(&["tab"]), with_help("tab", "next")]),
            prev: new_binding(vec![
                with_keys_str(&["shift+tab"]),
                with_help("shift+tab", "prev"),
            ]),
            add: new_binding(vec![
                with_keys_str(&["ctrl+n"]),
                with_help("ctrl+n", "add an editor"),
            ]),
            remove: new_binding(vec![
                with_keys_str(&["ctrl+w"]),
                with_help("ctrl+w", "remove an editor"),
            ]),
            quit: new_binding(vec![
                with_keys_str(&["esc", "ctrl+c"]),
                with_help("esc", "quit"),
            ]),
        }
    }
}

impl KeyMap for Keymap {
    fn short_help(&self) -> Vec<&Binding> {
        vec![&self.next, &self.prev, &self.add, &self.remove, &self.quit]
    }

    fn full_help(&self) -> Vec<Vec<&Binding>> {
        vec![vec![
            &self.next,
            &self.prev,
            &self.add,
            &self.remove,
            &self.quit,
        ]]
    }
}

/// Main application model
struct SplitEditorsModel {
    width: i32,
    height: i32,
    keymap: Keymap,
    help: HelpModel,
    inputs: Vec<TextArea>,
    focus: usize,
}

impl SplitEditorsModel {
    fn new() -> Self {
        let mut inputs = Vec::with_capacity(INITIAL_INPUTS);
        for _ in 0..INITIAL_INPUTS {
            inputs.push(new_textarea());
        }

        let mut model = Self {
            width: 80,
            height: 24,
            keymap: Keymap::new(),
            help: HelpModel::new(),
            inputs,
            focus: 0,
        };

        // Focus the first textarea
        model.inputs[model.focus].focus();
        model.update_keybindings();
        model
    }

    /// Updates which key bindings are enabled based on current state
    fn update_keybindings(&mut self) {
        // This would update enabled state if the API supported it
        // For now, we'll handle the logic in the update function
    }

    /// Sizes all textareas based on current terminal dimensions
    fn size_inputs(&mut self) {
        // Make editors narrower - use about 70% of available width per editor
        let available_width = (self.width as f32 * 0.7) as i32;
        let width_per_input = available_width / self.inputs.len() as i32;

        // Leave room for help at bottom, and make editors reasonable height (not full screen)
        let available_height = self.height - HELP_HEIGHT - 4; // Extra margin
        let editor_height = (available_height * 2 / 3).clamp(10, 20); // Reasonable height range

        for input in &mut self.inputs {
            input.set_width(width_per_input.max(25) as usize); // Reasonable minimum width
            input.set_height(editor_height as usize);
        }
    }
}

// Implement HelpKeyMap trait to connect our keymap to the help widget
impl HelpKeyMap for SplitEditorsModel {
    fn short_help(&self) -> Vec<&Binding> {
        // Filter bindings based on current state
        let mut bindings = vec![&self.keymap.next, &self.keymap.prev];

        // Only show add binding if we can add more
        if self.inputs.len() < MAX_INPUTS {
            bindings.push(&self.keymap.add);
        }

        // Only show remove binding if we can remove
        if self.inputs.len() > MIN_INPUTS {
            bindings.push(&self.keymap.remove);
        }

        bindings.push(&self.keymap.quit);
        bindings
    }

    fn full_help(&self) -> Vec<Vec<&Binding>> {
        vec![self.short_help()]
    }
}

impl Model for SplitEditorsModel {
    fn init() -> (Self, Option<Cmd>) {
        let mut model = Self::new();

        // Set initial terminal size if available
        if let Ok((w, h)) = terminal::size() {
            model.width = w as i32;
            model.height = h as i32;
            model.help.width = w as usize;
            model.size_inputs();
        }

        // Trigger initial render
        (model, Some(init_render_cmd()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        let cmds = Vec::new();

        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if matches_binding(key_msg, &self.keymap.quit) {
                // Blur all textareas before quitting
                for input in &mut self.inputs {
                    input.blur();
                }
                return Some(quit());
            } else if matches_binding(key_msg, &self.keymap.next) {
                // Move to next textarea
                self.inputs[self.focus].blur();
                self.focus = (self.focus + 1) % self.inputs.len();
                self.inputs[self.focus].focus();
            } else if matches_binding(key_msg, &self.keymap.prev) {
                // Move to previous textarea
                self.inputs[self.focus].blur();
                if self.focus == 0 {
                    self.focus = self.inputs.len() - 1;
                } else {
                    self.focus -= 1;
                }
                self.inputs[self.focus].focus();
            } else if matches_binding(key_msg, &self.keymap.add) && self.inputs.len() < MAX_INPUTS {
                // Add new textarea
                self.inputs.push(new_textarea());
                self.size_inputs();
            } else if matches_binding(key_msg, &self.keymap.remove)
                && self.inputs.len() > MIN_INPUTS
            {
                // Remove current textarea
                self.inputs.remove(self.focus);
                if self.focus >= self.inputs.len() {
                    self.focus = self.inputs.len() - 1;
                }
                self.inputs[self.focus].focus();
                self.size_inputs();
            } else {
                // Forward key events to the focused textarea for text editing
                self.inputs[self.focus].handle_key(key_msg);
            }
        } else if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            // Handle terminal resize
            self.width = size_msg.width as i32;
            self.height = size_msg.height as i32;
            self.help.width = size_msg.width as usize;
            self.size_inputs();
        } else if msg.downcast_ref::<InitRenderMsg>().is_some() {
            // No-op: receiving this message merely triggers the initial render
        }

        self.update_keybindings();

        // Return batch command if we have any commands
        if cmds.is_empty() {
            None
        } else {
            Some(bubbletea_rs::batch(cmds))
        }
    }

    fn view(&self) -> String {
        // Generate help text
        let help = self.help.view(self);

        // Create styled views for each textarea
        let mut views = Vec::new();
        for (i, input) in self.inputs.iter().enumerate() {
            // Get textarea rendered content with line numbers, cursor highlighting, etc.
            let textarea_view = input.view();

            // Apply border styling based on focus state
            let styled_view = if i == self.focus {
                focused_border_style().render(&textarea_view)
            } else {
                blurred_border_style().render(&textarea_view)
            };

            views.push(styled_view);
        }

        // Join all textarea views horizontally
        let editor_views: Vec<&str> = views.iter().map(|s| s.as_str()).collect();
        let editors = join_horizontal(TOP, &editor_views);

        format!("{}\n\n{}", editors, help)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<SplitEditorsModel>::builder()
        .alt_screen(true)
        .signal_handler(true)
        .build()?;

    program.run().await?;

    Ok(())
}
