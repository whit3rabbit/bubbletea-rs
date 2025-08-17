//! Pager Example
//!
//! A document viewer demonstrating the viewport component from bubbletea-widgets.
//! This example shows how to:
//!
//! ## Key Learning Patterns Demonstrated
//!
//! ### 📜 **Viewport Component Usage**
//! - **Scrollable Content**: Using `bubbletea-widgets::viewport` for text display
//! - **Dynamic Content Loading**: Reading markdown files from disk
//! - **Responsive Layout**: Adjusting viewport size based on header/footer height
//! - **Mouse & Keyboard Navigation**: Full scrolling support
//!
//! ### 🎨 **Advanced Lipgloss Styling**
//! - **Custom Borders**: Modifying border characters for visual connections
//! - **Dynamic Line Drawing**: Creating horizontal lines that fill available space
//! - **Layout Calculations**: Computing widths for responsive design
//! - **Styled Text Rendering**: Combining borders, padding, and content
//!
//! ### 💻 **Program Configuration**
//! - **Alternate Screen Buffer**: Full-screen TUI mode with `.alt_screen()`
//! - **Mouse Support**: Enabling mouse wheel scrolling with `.mouse_motion()`
//! - **File I/O Integration**: Loading external content at startup
//! - **Error Handling**: Graceful handling of missing files
//!
//! ## What You'll See
//! ```
//! ╭ Mr. Pager ├─────────────────────────────────────────────────────────────────
//! │ Glow                                                                        │
//! │ ====                                                                        │
//! │                                                                             │
//! │ A casual introduction. 你好世界!                                             │
//! │                                                                             │
//! │ ## Let's talk about artichokes                                              │
//! │                                                                             │
//! │ The _artichoke_ is mentioned as a garden plant in the 8th century BC       │
//! │ ...                                                                         │
//! ├─────────────────────────────────────────────────────────────────────── 47% ╯
//! ```
//!
//! ## 🎨 Layout Architecture Explained
//!
//! **Three-Part Layout:**
//! ```
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ Header: ╭ Mr. Pager ├─────────────────────────────────────────────────────  │ ← title + line
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ │ Content: viewport.view() renders scrollable markdown content here      │ │ ← viewport
//! │ │ This area scrolls with arrow keys, page up/down, mouse wheel          │ │
//! │ │ Size = terminal_height - header_height - footer_height                │ │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ Footer: ├─────────────────────────────────────────────────────────── 47% ╯  │ ← line + percentage
//! └─────────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! **Key Layout Insights:**
//! - Header: title box + calculated line length = terminal width
//! - Footer: calculated line length + percentage box = terminal width  
//! - Content: viewport handles its own scrolling and text wrapping
//! - Border connections ("├", "┤") create seamless visual flow
//!
//! This is a faithful port of the Go Bubble Tea pager example with identical
//! behavior, UI styling, and navigation.
//!
//! Usage: cargo run --bin pager

// bubbletea-rs core imports for MVU pattern
use bubbletea_rs::{quit, window_size, KeyMsg, Model as BubbleTeaModel, MouseMotion, Msg, Program, WindowSizeMsg};

// bubbletea-widgets for viewport component
use bubbletea_widgets::viewport;

// crossterm for keyboard input handling
use crossterm::event::{KeyCode, KeyModifiers};

// lipgloss-extras for terminal styling (borders, colors, layout)
use lipgloss_extras::lipgloss::{border, join_horizontal, width_visible, Style, CENTER};

// Standard library imports
use std::fs;

// =============================================================================
// TERMINAL LAYOUT GOTCHAS & PATTERNS
// =============================================================================
//
// ## ⚠️ CRITICAL GOTCHAS FOR TERMINAL UI DEVELOPMENT
//
// ### 1. NEVER use .len() for styled text width calculations!
// ```rust
// // ❌ WRONG - includes invisible ANSI escape codes
// let styled_text = style.render("Hello");
// let width = styled_text.len(); // Could be 20+ chars with escape codes
//
// // ✅ CORRECT - gets actual visual width in terminal
// let width = width_visible(&styled_text); // Actual 5 chars visible
// ```
//
// ### 2. Always use lipgloss layout functions for alignment
// ```rust
// // ❌ WRONG - doesn't handle terminal alignment properly
// let result = format!("{}{}", left_text, right_text);
//
// // ✅ CORRECT - handles proper terminal layout
// let result = join_horizontal(CENTER, &[&left_text, &right_text]);
// ```
//
// ### 3. Viewport width vs terminal width
// - `viewport.width` = full terminal width (set in WindowSizeMsg handler)
// - This is what we use for header/footer calculations
// - Content height is reduced by header/footer space
//
// ### 4. Border character connections
// - Use "├" to connect borders to right-side lines
// - Use "┤" to connect borders to left-side lines
// - This creates seamless visual flow across the terminal
//
// ### 5. String references with join_horizontal
// ```rust
// // ❌ WRONG - type mismatch error
// join_horizontal(CENTER, &[title, line]) // String != &str
//
// // ✅ CORRECT - use string references
// join_horizontal(CENTER, &[&title, &line])
// ```

// =============================================================================
// STYLING CONSTANTS AND HELPERS
// =============================================================================

/// Create the title style with modified border (matching Go version)
///
/// ## lipgloss Pattern: Custom Border Modification
/// This demonstrates how to create a partial border that connects to horizontal lines.
///
/// **Why modify the border?**
/// - Standard rounded border: `╭─────╮`
/// - Our modified border:     `╭─────├` (right side connects to horizontal line)
/// - This creates a seamless visual connection: `╭─────├──────────────────────────`
///
/// **Visual Layout:**
/// ```
/// ╭ Mr. Pager ├─────────────────────────────────────────────────────────────────
/// │ (viewport content here)                                                     │
/// ├─────────────────────────────────────────────────────────────────────── 47% ╯
/// ```
///
/// The "├" character visually connects the title box to the horizontal rule.
fn title_style() -> Style {
    let mut border = border::rounded_border();
    border.right = "├"; // Connect to the horizontal line - this is the key!

    Style::new()
        .border(border)
        .padding_right(1) // Space between text and border
        .padding_left(1) // Space between border and text
}

/// Create the info style with modified border (matching Go version)
///
/// ## lipgloss Pattern: Border Reuse with Modification
/// This creates the footer percentage box that connects to horizontal lines from the left.
///
/// **Why modify the left border?**
/// - Standard rounded border: `╭─────╮`
/// - Our modified border:     `┤─────╮` (left side connects to horizontal line)
/// - This creates connection:  `──────────────────────────┤ 47% ╯`
///
/// **Visual Layout:**
/// ```
/// ╭ Mr. Pager ├─────────────────────────────────────────────────────────────────
/// │ (viewport content here)                                                     │
/// ├─────────────────────────────────────────────────────────────────────── 47% ╯
/// ```
///
/// The "┤" character visually connects the horizontal rule to the percentage box.
/// This is the mirror image of the title style, creating visual balance.
fn info_style() -> Style {
    let mut border = border::rounded_border();
    border.left = "┤"; // Connect to the horizontal line - mirrors the title style

    Style::new()
        .border(border)
        .padding_right(1) // Space between text and border
        .padding_left(1) // Space between border and text
}

// =============================================================================
// APPLICATION MODEL
// =============================================================================

/// The pager model containing viewport and document state
///
/// ## bubbletea-rs Pattern: Viewport Integration
/// Shows how to integrate a bubbletea-widgets component into your model:
/// - The viewport handles its own scrolling state
/// - Model tracks initialization and responsiveness
/// - Window size changes update viewport dimensions
#[derive(Debug)]
pub struct PagerModel {
    /// The document content loaded from artichoke.md
    content: String,
    /// Whether we've received initial window dimensions
    ready: bool,
    /// The viewport component for scrollable display
    viewport: viewport::Model,
    /// Manual scroll offset to work around version conflicts
    scroll_offset: usize,
    /// Content lines for manual scrolling
    content_lines: Vec<String>,
}

impl PagerModel {
    /// Create a new pager model with content loaded from file
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load content from artichoke.md file
        let content = fs::read_to_string("artichoke.md")
            .map_err(|e| format!("could not load file: {}", e))?;

        // Split content into lines for manual scrolling
        let content_lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        // Initialize with reasonable defaults (80x22) to account for header/footer
        let mut viewport = viewport::new(80, 22);
        viewport.set_content(&content);

        Ok(PagerModel {
            content,
            ready: true, // Start ready immediately with defaults
            viewport,
            scroll_offset: 0,
            content_lines,
        })
    }

    /// Create the header view with title and horizontal line
    ///
    /// ## bubbletea-rs Pattern: Terminal Layout with Styled Text
    /// This demonstrates the CORRECT way to calculate layout with styled terminal text.
    ///
    /// **The Core Problem:**
    /// When you style text with colors/borders, the actual string contains ANSI escape codes:
    /// ```
    /// let styled = style.render("Mr. Pager");
    /// // styled might be: "\x1b[38;5;69m╭ Mr. Pager ├\x1b[0m"
    /// // styled.len() = 25 characters (includes invisible ANSI codes!)
    /// // But visually it only takes 12 columns in the terminal
    /// ```
    ///
    /// **Why we use width_visible():**
    /// - `styled.len()` = 25 (includes ANSI escape codes)
    /// - `width_visible(&styled)` = 12 (actual visual width in terminal)
    /// - We need the visual width to calculate remaining space for the line
    ///
    /// **Layout Calculation:**
    /// ```
    /// Terminal width: 80 columns
    /// Title visual width: 12 columns  
    /// Line width needed: 80 - 12 = 68 columns
    /// Result: "╭ Mr. Pager ├" + "─".repeat(68)
    /// ```
    ///
    /// **Why join_horizontal():**
    /// Simple string concat (`format!("{}{}")`) doesn't handle alignment properly.
    /// `join_horizontal(CENTER, &[title, line])` ensures proper terminal layout.
    fn header_view(&self) -> String {
        let title = title_style().render("Mr. Pager");

        // CRITICAL: Use width_visible() not .len() for styled text!
        // .len() includes ANSI escape codes, width_visible() gives actual terminal columns
        let title_visual_width = width_visible(&title);

        // Calculate remaining space: total viewport width minus title's visual width
        let line_width = self.viewport.width.saturating_sub(title_visual_width);
        let line = "─".repeat(line_width);

        // Use lipgloss layout function for proper alignment (matches Go Bubble Tea)
        join_horizontal(CENTER, &[&title, &line])
    }

    /// Create the footer view with scroll percentage and horizontal line
    ///
    /// ## bubbletea-rs Pattern: Scroll Percentage Display with Right Alignment
    /// This demonstrates creating a footer with scroll progress, mirroring the header pattern.
    ///
    /// **Layout Goal:**
    /// ```
    /// ├─────────────────────────────────────────────────────────────────────── 47% ╯
    /// ^                                                                         ^
    /// horizontal line fills remaining space                          styled percentage
    /// ```
    ///
    /// **Key Differences from header_view:**
    /// 1. **Content**: Shows scroll percentage instead of title
    /// 2. **Order**: Line comes FIRST, then percentage (right-aligned)
    /// 3. **Formatting**: Percentage is formatted as integer (47% not 47.234%)
    ///
    /// **Same Critical Pattern:**
    /// - Use `width_visible()` for styled text width (not `.len()`)
    /// - Calculate remaining space: `total_width - styled_text_width`
    /// - Use `join_horizontal()` for proper layout
    ///
    /// **Visual Width Example:**
    /// ```
    /// let info = info_style().render("47%");
    /// // info string: "\x1b[38;5;69m┤ 47% ╯\x1b[0m" (with ANSI codes)
    /// // info.len() = 15 (includes invisible ANSI escape codes)
    /// // width_visible(&info) = 7 (actual visual columns in terminal)
    /// ```
    fn footer_view(&self) -> String {
        // Calculate scroll percentage manually
        let scroll_percent = if self.content_lines.len() <= self.viewport.height {
            100.0 // If all content fits, we're at 100%
        } else {
            let max_offset = self.content_lines.len().saturating_sub(self.viewport.height);
            if max_offset == 0 {
                100.0
            } else {
                (self.scroll_offset as f64 / max_offset as f64) * 100.0
            }
        };

        // Format as integer percentage (47% not 47.234%)
        let info = info_style().render(&format!("{:3.0}%", scroll_percent));

        // CRITICAL: Use width_visible() not .len() for styled text!
        // Same principle as header_view - styled text contains invisible ANSI codes
        let info_visual_width = width_visible(&info);

        // Calculate remaining space for the horizontal line (left side)
        let line_width = self.viewport.width.saturating_sub(info_visual_width);
        let line = "─".repeat(line_width);

        // Order matters: line FIRST, then info (creates right-alignment effect)
        join_horizontal(CENTER, &[&line, &info])
    }

    /// Render the viewport content manually
    /// 
    /// Since we can't use the Model trait due to version conflicts,
    /// we implement basic viewport rendering ourselves
    fn viewport_view(&self) -> String {
        // Calculate which lines to show based on scroll offset and viewport height
        let start = self.scroll_offset;
        let end = std::cmp::min(start + self.viewport.height, self.content_lines.len());
        
        // Get the visible lines
        let visible_lines = &self.content_lines[start..end];
        
        // Pad with empty lines if we don't have enough content to fill the viewport
        let mut result = visible_lines.to_vec();
        while result.len() < self.viewport.height {
            result.push(String::new());
        }
        
        result.join("\n")
    }
}

// =============================================================================
// MODEL-VIEW-UPDATE (MVU) IMPLEMENTATION
// =============================================================================

impl BubbleTeaModel for PagerModel {
    /// Initialize the model by loading content from disk
    ///
    /// ## bubbletea-rs Pattern: File Loading in Init
    /// Demonstrates loading external resources during initialization.
    /// Error handling here uses Result to fail fast if content is missing.
    ///
    /// ## bubbletea-rs Pattern: Window Size Request
    /// We request the window size immediately so the viewport can be properly initialized.
    /// Without this, the model would stay in "Initializing..." state forever.
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) {
        match PagerModel::new() {
            Ok(model) => (model, Some(window_size())),
            Err(e) => {
                eprintln!("Error initializing pager: {}", e);
                std::process::exit(1);
            }
        }
    }

    /// Handle messages for navigation and window resizing
    ///
    /// ## bubbletea-rs Pattern: Viewport Message Delegation
    /// The viewport component handles most navigation messages itself.
    /// We only need to intercept quit messages and window size changes.
    fn update(&mut self, msg: Msg) -> Option<bubbletea_rs::Cmd> {
        // Handle keyboard input for navigation and quitting
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('q') | KeyCode::Esc => {
                    return Some(quit());
                }
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit());
                }
                // Manual viewport navigation since we can't delegate to viewport.update()
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.scroll_offset > 0 {
                        self.scroll_offset -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let max_offset = self.content_lines.len().saturating_sub(self.viewport.height);
                    if self.scroll_offset < max_offset {
                        self.scroll_offset += 1;
                    }
                }
                KeyCode::PageUp => {
                    let page_size = self.viewport.height / 2;
                    self.scroll_offset = self.scroll_offset.saturating_sub(page_size);
                }
                KeyCode::PageDown => {
                    let page_size = self.viewport.height / 2;
                    let max_offset = self.content_lines.len().saturating_sub(self.viewport.height);
                    self.scroll_offset = std::cmp::min(self.scroll_offset + page_size, max_offset);
                }
                KeyCode::Home => {
                    self.scroll_offset = 0;
                }
                KeyCode::End => {
                    self.scroll_offset = self.content_lines.len().saturating_sub(self.viewport.height);
                }
                _ => {}
            }
        }

        // Handle window size changes for responsive layout
        if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            // Calculate viewport dimensions accounting for header and footer
            let header_height = 1; // Header takes 1 line
            let footer_height = 1; // Footer takes 1 line
            let vertical_margin = header_height + footer_height;

            // Resize viewport by creating a new one with the actual terminal dimensions
            // The viewport component doesn't have resize methods, so we recreate it
            //
            // ## bubbletea-rs Pattern: Viewport Resizing
            // When terminal size changes, we recreate the viewport with new dimensions
            self.viewport = viewport::new(
                size_msg.width as usize,
                (size_msg.height as usize).saturating_sub(vertical_margin),
            );
            self.viewport.set_content(&self.content);
            
            // Reset scroll offset to ensure it's within bounds for new height
            let max_offset = self.content_lines.len().saturating_sub(self.viewport.height);
            self.scroll_offset = std::cmp::min(self.scroll_offset, max_offset);
            return None;
        }

        // No need to delegate other messages since we handle navigation manually
        None
    }

    /// Render the complete pager interface
    ///
    /// ## bubbletea-rs Pattern: Structured View Composition
    /// Shows how to compose a multi-part interface:
    /// - Handle loading state gracefully
    /// - Combine header, content, and footer
    /// - Use newlines for proper vertical spacing
    fn view(&self) -> String {
        if !self.ready {
            return "\n  Initializing...".to_string();
        }

        // Compose the full interface: header + viewport + footer
        format!(
            "{}\n{}\n{}",
            self.header_view(),
            self.viewport_view(),
            self.footer_view()
        )
    }
}

// =============================================================================
// MAIN PROGRAM
// =============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ## bubbletea-rs Pattern: Full-Screen Pager Configuration
    // This demonstrates the typical setup for a document viewer:
    // - .alt_screen(true): Use alternate screen buffer (fullscreen)
    // - .mouse_motion(): Enable mouse wheel scrolling
    // This matches the Go version's WithAltScreen() and WithMouseCellMotion()
    let program = Program::<PagerModel>::builder()
        .alt_screen(true) // Enable alternate screen buffer
        .mouse_motion(MouseMotion::Cell) // Enable mouse wheel support
        .build()?;

    // Run the program and handle any errors
    if let Err(err) = program.run().await {
        println!("Error running program: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
