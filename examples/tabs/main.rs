//! Tabs Example
//!
//! A sophisticated tabbed interface demonstrating advanced layout techniques,
//! border manipulation, and keyboard navigation patterns using bubbletea-rs.
//!
//! This example shows how to:
//! - Create visually connected tab interfaces with custom borders
//! - Handle multi-directional keyboard navigation (arrows, vim keys, tab keys)
//! - Dynamically adjust content layout based on tab row width
//! - Use Unicode box-drawing characters for seamless visual connections
//! - Apply consistent styling across active and inactive states
//!
//! The implementation creates 5 cosmetic product tabs that users can navigate
//! between, with each tab displaying unique content in a connected window below.

// Core bubbletea-rs imports for the Model-View-Update architecture
use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};

// Crossterm for keyboard input handling
use crossterm::event::{KeyCode, KeyModifiers};

// Lipgloss-extras for advanced styling and layout
use lipgloss_extras::lipgloss::{
    join_horizontal, // Combines multiple styled elements horizontally
    rounded_border,  // Provides rounded corner border style
    width,           // Calculates display width of styled content
    Border,          // Border configuration struct
    Color,           // Color management
    Style,           // Primary styling interface
    TOP,             // Alignment constant for top-aligned joining
};

// Standard library imports for math operations
use std::cmp::{max, min};

// ============================================================================
// INITIAL RENDER TRIGGER
// ============================================================================

/// Synthetic message used to trigger the initial render immediately after startup.
///
/// Without this, the interface would remain blank until the user presses a key.
/// This pattern ensures users see the tabbed interface immediately when the
/// program starts, creating a better user experience.
#[derive(Debug)]
struct InitRenderMsg;

/// Creates a command that triggers the initial render.
///
/// Returns a boxed async future that immediately resolves with an InitRenderMsg.
/// This is called from init() to force an immediate UI update on program start.
fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

// ============================================================================
// APPLICATION MODEL
// ============================================================================

/// The main application state for the tabbed interface.
///
/// This struct follows the Model-View-Update (MVU) pattern where all application
/// state is contained in a single struct. The state is modified only through
/// the update() method and displayed through the view() method.
#[derive(Debug)]
struct TabModel {
    /// Names of all available tabs, displayed in the tab row.
    /// The order in this vector determines the tab order in the UI.
    tabs: Vec<String>,

    /// Content to display for each tab. Must have the same length as `tabs`.
    /// The content at index `i` corresponds to the tab at index `i`.
    tab_content: Vec<String>,

    /// Index of the currently active/selected tab.
    /// Must always be a valid index into both `tabs` and `tab_content` vectors.
    /// This determines which tab appears active and which content is displayed.
    active_tab: usize,
}

impl Default for TabModel {
    /// Creates the default tab model with cosmetic product tabs.
    ///
    /// This sets up 5 tabs themed around makeup/cosmetic products, demonstrating
    /// a realistic use case where tabs organize related but distinct content areas.
    ///
    /// The active_tab starts at 0, meaning the first tab ("Lip Gloss") is
    /// initially selected when the program starts.
    fn default() -> Self {
        // Tab names that will appear in the tab row
        // These are kept concise to fit well in the interface
        let tabs = vec![
            "Lip Gloss".to_string(),
            "Blush".to_string(),
            "Eye Shadow".to_string(),
            "Mascara".to_string(),
            "Foundation".to_string(),
        ];

        // Content for each tab - in a real application, this might be
        // complex data structures, widgets, or dynamically generated content
        let tab_content = vec![
            "Lip Gloss Tab".to_string(),
            "Blush Tab".to_string(),
            "Eye Shadow Tab".to_string(),
            "Mascara Tab".to_string(),
            "Foundation Tab".to_string(),
        ];

        Self {
            tabs,
            tab_content,
            active_tab: 0, // Start with first tab selected
        }
    }
}

// ============================================================================
// BORDER STYLING UTILITIES
// ============================================================================

/// Creates a custom border with specified bottom edge characters.
///
/// This function is crucial for creating the visual connection between tabs
/// and the content window. It modifies the default rounded border to use
/// specific Unicode box-drawing characters for the bottom edge.
///
/// # Arguments
/// * `left` - Bottom-left corner character (e.g., "┘" for active, "┴" for inactive)
/// * `middle` - Bottom edge character (e.g., " " for active, "─" for inactive)  
/// * `right` - Bottom-right corner character (e.g., "└" for active, "┴" for inactive)
///
/// # Border Character Explanation
///
/// **Active Tab Border**: `("┘", " ", "└")`
/// ```text
/// ┌─────────┐
/// │ Active  │
/// └─────────┘  ← Open bottom connects to content window
/// ```
///
/// **Inactive Tab Border**: `("┴", "─", "┴")`
/// ```text
/// ┌─────────┐
/// │ Inactive│
/// └─────────┘  ← Closed bottom separates from content
/// ```
///
/// # Unicode Box-Drawing Characters Used
/// - `┌` `┐` `└` `┘` - Corners
/// - `─` - Horizontal line
/// - `│` - Vertical line  
/// - `┴` - T-junction pointing up
/// - `├` `┤` - T-junctions pointing right/left
fn tab_border_with_bottom(left: &'static str, middle: &'static str, right: &'static str) -> Border {
    // Start with lipgloss's standard rounded border
    let mut border = rounded_border();

    // Customize only the bottom edge to create tab-specific connections
    border.bottom_left = left; // Controls left corner connection
    border.bottom = middle; // Controls bottom edge (open for active, closed for inactive)
    border.bottom_right = right; // Controls right corner connection

    border
}

// ============================================================================
// MODEL IMPLEMENTATION
// ============================================================================

impl Model for TabModel {
    /// Initializes the application model and triggers the first render.
    ///
    /// This method is called once when the program starts. It returns the
    /// initial model state and optionally a command to execute immediately.
    ///
    /// The init_render_cmd() ensures the interface appears immediately rather
    /// than waiting for user input to trigger the first render.
    fn init() -> (Self, Option<Cmd>) {
        (TabModel::default(), Some(init_render_cmd()))
    }

    /// Processes messages and updates the application state.
    ///
    /// This is the heart of the Model-View-Update architecture. All state
    /// changes happen here in response to messages (keyboard input, timer
    /// events, custom messages, etc.).
    ///
    /// Returns an optional command to execute after the state update.
    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle keyboard input messages
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                // ============================================================
                // EXIT COMMANDS
                // ============================================================
                /// Ctrl+C - Standard terminal interrupt signal
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit());
                }

                /// 'q' - Quick quit key, common in terminal applications
                KeyCode::Char('q') => {
                    return Some(quit());
                }

                // ============================================================
                // NAVIGATION - NEXT TAB
                // ============================================================
                /// Multiple ways to move to the next tab:
                /// - Right Arrow: Standard navigation
                /// - 'l': Vim-style right movement
                /// - 'n': Next (mnemonic)
                /// - Tab: Standard tab navigation
                KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('n') | KeyCode::Tab => {
                    // Use min() to prevent going past the last tab
                    // tabs.len() - 1 gives us the index of the last tab
                    self.active_tab = min(self.active_tab + 1, self.tabs.len() - 1);
                }

                // ============================================================
                // NAVIGATION - PREVIOUS TAB
                // ============================================================
                /// Multiple ways to move to the previous tab:
                /// - Left Arrow: Standard navigation
                /// - 'h': Vim-style left movement
                /// - 'p': Previous (mnemonic)  
                /// - Shift+Tab: Standard reverse tab navigation
                KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('p') | KeyCode::BackTab => {
                    // Use saturating_sub() to handle underflow when at tab 0
                    // max() ensures we never go below 0
                    self.active_tab = max(self.active_tab.saturating_sub(1), 0);
                }

                // Ignore all other key presses
                _ => {}
            }
        }

        // Handle the synthetic initial render message
        if let Some(_init_msg) = msg.downcast_ref::<InitRenderMsg>() {
            // No action needed - this just triggers the initial view() call
            // to display the interface when the program starts
        }

        // No additional commands needed after processing these messages
        None
    }

    /// Renders the complete tabbed interface.
    ///
    /// This method creates the visual representation of the current application
    /// state. It's called automatically whenever the state changes or when the
    /// terminal needs to refresh the display.
    ///
    /// # Rendering Pipeline
    /// 1. Define border styles for active and inactive tabs
    /// 2. Create styling for tabs, content window, and overall document
    /// 3. Render each tab with appropriate styling and border connections
    /// 4. Join tabs horizontally with proper alignment
    /// 5. Calculate content width to match tab row width
    /// 6. Render content window with matching width
    /// 7. Combine tab row and content vertically
    /// 8. Apply document-level styling (padding, etc.)
    ///
    /// # Visual Layout
    /// ```text
    /// ┌─────────────┬────────┬────────────┬─────────┬────────────┐
    /// │  Lip Gloss  │ Blush  │ Eye Shadow │ Mascara │ Foundation │ ← Tab Row
    /// └─────────────┴──┬─────┴────────────┴─────────┴────────────┘ ← Border Connection
    ///                  │
    /// ┌────────────────┴──────────────────────────────────────────┐
    /// │                                                            │
    /// │                    Lip Gloss Tab                           │ ← Content Area
    /// │                                                            │
    /// └────────────────────────────────────────────────────────────┘
    /// ```
    fn view(&self) -> String {
        // ====================================================================
        // STEP 1: DEFINE TAB BORDER STYLES
        // ====================================================================

        /// Border for inactive tabs - fully enclosed with T-junctions
        /// ┌─────────┐
        /// │ Inactive│
        /// └─────────┘  ← Closed bottom with ┴ characters
        let inactive_tab_border = tab_border_with_bottom("┴", "─", "┴");

        /// Border for active tabs - open bottom connects to content window
        /// ┌─────────┐
        /// │ Active  │
        /// ┘         └  ← Open bottom with corner characters
        let active_tab_border = tab_border_with_bottom("┘", " ", "└");

        // ====================================================================
        // STEP 2: CREATE STYLING DEFINITIONS
        // ====================================================================

        /// Base style for inactive tabs
        /// - Purple border color (#874BFD) for consistent theming
        /// - 1 unit horizontal padding for text spacing
        /// - All borders enabled to create complete enclosure
        let inactive_tab_style = Style::new()
            .border_style(inactive_tab_border)
            .border_foreground(Color::from("#874BFD")) // Purple theme color
            .padding(0, 1, 0, 1) // top, right, bottom, left padding
            .border_top(true) // Enable all borders
            .border_left(true)
            .border_right(true)
            .border_bottom(true);

        /// Active tab style inherits from inactive but uses different border
        /// This ensures consistent styling except for the connection behavior
        let active_tab_style = inactive_tab_style.clone().border_style(active_tab_border);

        /// Content window style
        /// - No top border to connect seamlessly with tabs
        /// - Purple border matching tab colors
        /// - Vertical padding for content spacing
        /// - Center-aligned text
        let window_style = Style::new()
            .border_foreground(Color::from("#874BFD"))
            .padding(2, 0, 2, 0) // Vertical padding for readability
            .align_horizontal(lipgloss_extras::lipgloss::CENTER)
            .border_style(lipgloss_extras::lipgloss::normal_border())
            .border_top(false) // No top border - connects to tabs
            .border_left(true)
            .border_right(true)
            .border_bottom(true);

        /// Overall document style adds outer padding around the entire interface
        let doc_style = Style::new().padding(1, 2, 1, 2);

        // ====================================================================
        // STEP 3: RENDER INDIVIDUAL TABS
        // ====================================================================

        let mut rendered_tabs = Vec::new();

        for (i, tab_name) in self.tabs.iter().enumerate() {
            // Determine tab position and state for border customization
            let is_first = i == 0;
            let is_last = i == self.tabs.len() - 1;
            let is_active = i == self.active_tab;

            // Start with appropriate base style (active or inactive)
            let mut style = if is_active {
                active_tab_style.clone()
            } else {
                inactive_tab_style.clone()
            };

            // Get current border configuration to modify edge connections
            let (mut border, _, _, _, _) = style.get_border();

            // ================================================================
            // STEP 3A: CUSTOMIZE BORDER CONNECTIONS FOR FIRST/LAST TABS
            // ================================================================

            /// The first and last tabs need special border characters to
            /// connect properly with the content window's left and right edges.
            ///
            /// First Tab Connections:
            /// - Active: │ (vertical line continues to content border)
            /// - Inactive: ├ (T-junction allows content border to continue)
            ///
            /// Last Tab Connections:  
            /// - Active: │ (vertical line continues to content border)
            /// - Inactive: ┤ (T-junction allows content border to continue)
            if is_first && is_active {
                border.bottom_left = "│"; // Vertical connection to content
            } else if is_first && !is_active {
                border.bottom_left = "├"; // T-junction for content border
            } else if is_last && is_active {
                border.bottom_right = "│"; // Vertical connection to content
            } else if is_last && !is_active {
                border.bottom_right = "┤"; // T-junction for content border
            }

            // Apply the customized border and render the tab
            style = style.border_style(border);
            rendered_tabs.push(style.render(tab_name));
        }

        // ====================================================================
        // STEP 4: COMBINE TABS HORIZONTALLY
        // ====================================================================

        /// Join all rendered tabs horizontally with TOP alignment
        /// This ensures all tabs align at their top edges, creating a clean
        /// tab row regardless of individual tab content or styling differences
        let row = join_horizontal(
            TOP,
            &rendered_tabs.iter().map(String::as_str).collect::<Vec<_>>(),
        );

        // ====================================================================
        // STEP 5: CREATE CONTENT WINDOW WITH MATCHING WIDTH
        // ====================================================================

        /// Calculate content width to match the tab row width exactly
        /// - width(&row) gets the total display width of the tab row
        /// - get_horizontal_frame_size() accounts for the content window's
        ///   left and right border thickness
        /// - This ensures the content window aligns perfectly with the tabs
        let content_width = width(&row) as i32 - window_style.get_horizontal_frame_size();

        /// Render the content window with calculated width
        /// Uses the content for the currently active tab
        let content = window_style
            .width(content_width)
            .render(&self.tab_content[self.active_tab]);

        // ====================================================================
        // STEP 6: COMBINE TAB ROW AND CONTENT VERTICALLY
        // ====================================================================

        /// Stack the tab row above the content window
        /// The \n creates the vertical separation between the two elements
        let result = format!("{}\n{}", row, content);

        // ====================================================================
        // STEP 7: APPLY DOCUMENT-LEVEL STYLING
        // ====================================================================

        /// Apply overall padding and any document-level styling
        /// This creates spacing around the entire tabbed interface
        doc_style.render(&result)
    }
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

/// Entry point for the tabs example application.
///
/// This function sets up and runs the bubbletea-rs program with our TabModel.
/// The tokio::main attribute enables async/await support for the application.
///
/// # Error Handling
///
/// The function demonstrates the standard error handling pattern for
/// bubbletea-rs applications, including proper exit codes for different
/// termination scenarios.
///
/// # Program Builder
///
/// Uses the builder pattern to configure the program. For this simple example,
/// we use the default configuration, but the builder allows customization of:
/// - Signal handling (Ctrl+C, Ctrl+Z)
/// - Mouse support
/// - Focus reporting
/// - Alt screen usage
/// - Input modes
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ========================================================================
    // PROGRAM SETUP
    // ========================================================================

    /// Create and configure the bubbletea-rs program
    ///
    /// Program::<TabModel> specifies that this program will use TabModel
    /// as its application state. The type parameter ensures type safety
    /// throughout the entire application lifecycle.
    ///
    /// The builder pattern allows for future customization without breaking
    /// existing code. Common builder options include:
    /// - .signal_handler(true) for Ctrl+C/Ctrl+Z support
    /// - .mouse_support(true) for mouse input
    /// - .alt_screen(true) for full-screen applications
    let program = Program::<TabModel>::builder().build()?;

    // ========================================================================
    // PROGRAM EXECUTION WITH ERROR HANDLING
    // ========================================================================

    /// Run the program and handle different types of termination
    ///
    /// program.run() returns a Result that indicates how the program ended:
    /// - Ok(model) - Normal termination via quit() command
    /// - Err(Error::Interrupted) - Ctrl+C or signal termination
    /// - Err(Error::ProgramKilled) - Force kill via kill() command
    /// - Err(other) - Unexpected errors (I/O, terminal issues, etc.)
    if let Err(err) = program.run().await {
        match err {
            /// Unix convention: exit code 130 for SIGINT (Ctrl+C)
            /// This allows shell scripts and other programs to distinguish
            /// between user interruption and other types of program termination
            bubbletea_rs::Error::Interrupted => {
                std::process::exit(130);
            }

            /// Exit code 1 for force kill - indicates abnormal termination
            /// This is used when the program needs to exit immediately
            /// without normal cleanup procedures
            bubbletea_rs::Error::ProgramKilled => {
                std::process::exit(1);
            }

            /// Handle unexpected errors (I/O failures, terminal issues, etc.)
            /// Print the error message and exit with code 1 to indicate failure
            _ => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    }

    // ========================================================================
    // NORMAL TERMINATION
    // ========================================================================

    /// If we reach this point, the program terminated normally via quit()
    /// Return Ok(()) to indicate successful completion to the operating system
    Ok(())
}
