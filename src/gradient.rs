//! # Gradient Utilities
//!
//! This module provides high-performance utilities for rendering gradient-colored text
//! in terminal applications, specifically designed for progress bars and other UI elements
//! in the Bubble Tea TUI framework.
//!
//! The module focuses on performance by manually generating ANSI escape sequences rather
//! than using higher-level styling libraries, making it suitable for real-time rendering
//! of animated progress bars and other dynamic UI elements.
//!
//! ## Features
//!
//! - Fast RGB color interpolation for smooth gradients
//! - Optimized ANSI escape sequence generation
//! - Buffer reuse support for high-frequency rendering
//! - Charm Bubble Tea compatible default gradient colors
//!
//! ## Example
//!
//! ```rust
//! use bubbletea_rs::gradient::{gradient_filled_segment, charm_default_gradient, lerp_rgb};
//!
//! // Create a gradient progress bar
//! let progress_bar = gradient_filled_segment(20, '█');
//! println!("{}", progress_bar);
//!
//! // Get the default Charm gradient colors
//! let (start, end) = charm_default_gradient();
//! println!("Start: RGB({}, {}, {})", start.0, start.1, start.2);
//!
//! // Interpolate between colors
//! let mid_color = lerp_rgb(start, end, 0.5);
//! println!("Mid-point: RGB({}, {}, {})", mid_color.0, mid_color.1, mid_color.2);
//! ```

// Note: Crossterm style imports removed for manual ANSI sequence generation for better performance

/// Fast u8 to string conversion without allocations.
///
/// This function manually converts a u8 value (0-255) to its decimal string representation
/// and appends it directly to the provided string buffer. This avoids the overhead of
/// format macros and temporary allocations when building ANSI escape sequences.
///
/// # Arguments
///
/// * `s` - The string buffer to append the decimal representation to
/// * `value` - The u8 value to convert (0-255)
///
/// # Performance Notes
///
/// This function is optimized for the specific use case of generating ANSI color codes
/// where we frequently need to convert RGB values (0-255) to strings. It uses a
/// stack-allocated array to build the digits and avoids any heap allocations.
///
/// # Examples
///
/// ```rust,ignore
/// // This is a private function used internally for ANSI sequence generation
/// let mut buffer = String::new();
/// write_u8_to_string(&mut buffer, 255);
/// assert_eq!(buffer, "255");
///
/// buffer.clear();
/// write_u8_to_string(&mut buffer, 0);
/// assert_eq!(buffer, "0");
/// ```
#[inline]
fn write_u8_to_string(s: &mut String, mut value: u8) {
    if value == 0 {
        s.push('0');
        return;
    }

    // Convert to decimal digits (at most 3 digits for u8)
    let mut digits = [0u8; 3];
    let mut count = 0;

    while value > 0 {
        digits[count] = value % 10;
        value /= 10;
        count += 1;
    }

    // Push digits in reverse order (most significant first)
    for i in (0..count).rev() {
        s.push((b'0' + digits[i]) as char);
    }
}

/// Returns the default gradient color endpoints used by Charm's Bubble Tea framework.
///
/// This function provides the standard gradient colors used in Bubble Tea progress bars
/// and other UI elements, ensuring visual consistency with the original Go implementation.
/// The gradient transitions from a pink-purple color to a bright yellow-green.
///
/// # Returns
///
/// A tuple containing two RGB color tuples:
/// - First tuple: Start color `#FF7CCB` (255, 124, 203) - pink-purple
/// - Second tuple: End color `#FDFF8C` (253, 255, 140) - yellow-green
///
/// # Examples
///
/// ```rust
/// use bubbletea_rs::gradient::charm_default_gradient;
///
/// let (start, end) = charm_default_gradient();
/// assert_eq!(start, (0xFF, 0x7C, 0xCB)); // #FF7CCB
/// assert_eq!(end, (0xFD, 0xFF, 0x8C));   // #FDFF8C
///
/// println!("Start: RGB({}, {}, {})", start.0, start.1, start.2);
/// println!("End: RGB({}, {}, {})", end.0, end.1, end.2);
/// ```
///
/// # See Also
///
/// - [`lerp_rgb`] - For interpolating between these colors
/// - [`gradient_filled_segment`] - For creating gradient text using these colors
#[inline]
pub fn charm_default_gradient() -> ((u8, u8, u8), (u8, u8, u8)) {
    ((0xFF, 0x7C, 0xCB), (0xFD, 0xFF, 0x8C))
}

/// Performs linear interpolation between two RGB colors.
///
/// This function computes an intermediate RGB color at position `t` along the linear
/// path between the `start` and `end` colors. The interpolation is performed separately
/// for each color channel (red, green, blue) and the results are rounded to the nearest
/// integer values.
///
/// # Arguments
///
/// * `start` - The starting RGB color as a tuple of (red, green, blue) values (0-255)
/// * `end` - The ending RGB color as a tuple of (red, green, blue) values (0-255)  
/// * `t` - The interpolation parameter, where 0.0 returns `start`, 1.0 returns `end`,
///   and values in between return interpolated colors. Values outside \[0,1\] are clamped.
///
/// # Returns
///
/// An RGB color tuple representing the interpolated color at position `t`.
///
/// # Examples
///
/// ```rust
/// use bubbletea_rs::gradient::lerp_rgb;
///
/// let red = (255, 0, 0);
/// let blue = (0, 0, 255);
///
/// // Get the starting color
/// let start = lerp_rgb(red, blue, 0.0);
/// assert_eq!(start, (255, 0, 0));
///
/// // Get the ending color  
/// let end = lerp_rgb(red, blue, 1.0);
/// assert_eq!(end, (0, 0, 255));
///
/// // Get a color halfway between red and blue (purple)
/// let middle = lerp_rgb(red, blue, 0.5);
/// assert_eq!(middle, (128, 0, 128));
///
/// // Values outside [0,1] are clamped
/// let clamped = lerp_rgb(red, blue, 2.0);
/// assert_eq!(clamped, (0, 0, 255)); // Same as t=1.0
/// ```
///
/// # Performance Notes
///
/// This function uses floating-point arithmetic for interpolation and rounds the final
/// results. It's optimized for gradient generation where smooth color transitions are
/// more important than absolute performance.
#[inline]
pub fn lerp_rgb(start: (u8, u8, u8), end: (u8, u8, u8), t: f64) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    let r = (start.0 as f64 + (end.0 as f64 - start.0 as f64) * t).round() as u8;
    let g = (start.1 as f64 + (end.1 as f64 - start.1 as f64) * t).round() as u8;
    let b = (start.2 as f64 + (end.2 as f64 - start.2 as f64) * t).round() as u8;
    (r, g, b)
}

/// Creates a gradient-colored text segment for terminal display.
///
/// This function generates a string containing a gradient-colored sequence of characters
/// using ANSI escape codes. Each character is colored using linear interpolation between
/// Charm's default gradient colors, creating a smooth color transition from left to right.
/// This is commonly used for progress bars, loading indicators, and other visual elements
/// in terminal user interfaces.
///
/// # Arguments
///
/// * `filled_width` - The number of characters to include in the gradient segment.
///   If 0, returns an empty string.
/// * `ch` - The character to repeat for each position in the gradient (commonly '█', '▓', etc.)
///
/// # Returns
///
/// A `String` containing the gradient-colored characters with embedded ANSI escape sequences.
/// Each character includes both the foreground color code and a reset sequence.
///
/// # Performance Notes
///
/// This function pre-allocates string capacity and manually constructs ANSI sequences
/// to avoid the overhead of format macros and style library allocations. It's optimized
/// for repeated use in animation loops and real-time rendering.
///
/// # Examples
///
/// ```rust
/// use bubbletea_rs::gradient::gradient_filled_segment;
///
/// // Create a 10-character gradient progress bar
/// let progress = gradient_filled_segment(10, '█');
/// println!("{}", progress);
///
/// // Create a loading spinner segment
/// let spinner = gradient_filled_segment(5, '▓');
/// print!("Loading: {}\r", spinner);
///
/// // Empty width returns empty string
/// let empty = gradient_filled_segment(0, '█');
/// assert_eq!(empty, "");
/// ```
///
/// # ANSI Escape Sequence Format
///
/// Each character in the output follows the pattern:
/// `\x1b[38;2;r;g;bm{char}\x1b[0m`
///
/// Where:
/// - `\x1b[38;2;r;g;bm` sets the foreground color to RGB(r,g,b)
/// - `{char}` is the specified character
/// - `\x1b[0m` resets all formatting
///
/// # See Also
///
/// - [`gradient_filled_segment_with_buffer`] - Buffer-reusing variant for better performance
/// - [`charm_default_gradient`] - The gradient colors used by this function
/// - [`lerp_rgb`] - The color interpolation function used internally
pub fn gradient_filled_segment(filled_width: usize, ch: char) -> String {
    let (start, end) = charm_default_gradient();
    if filled_width == 0 {
        return String::new();
    }

    // Pre-allocate with better capacity estimation
    // ANSI color codes are typically ~19 bytes: \x1b[38;2;r;g;bmCHAR\x1b[0m
    let estimated_capacity = filled_width * 25; // 25 bytes per colored char (with some padding)
    let mut s = String::with_capacity(estimated_capacity);

    for i in 0..filled_width {
        let t = if filled_width <= 1 {
            0.0
        } else {
            i as f64 / (filled_width - 1) as f64
        };
        let (r, g, b) = lerp_rgb(start, end, t);

        // Manually construct ANSI escape sequence to avoid style() allocations
        // Format: \x1b[38;2;r;g;bm{char}\x1b[0m
        s.push_str("\x1b[38;2;");
        write_u8_to_string(&mut s, r);
        s.push(';');
        write_u8_to_string(&mut s, g);
        s.push(';');
        write_u8_to_string(&mut s, b);
        s.push('m');
        s.push(ch);
        s.push_str("\x1b[0m"); // Reset color
    }
    s
}

/// Creates a gradient-colored text segment using a reusable buffer for optimal performance.
///
/// This is a buffer-reusing variant of [`gradient_filled_segment`] designed for scenarios
/// where gradient segments are generated frequently, such as animated progress bars or
/// real-time UI updates. By reusing the same buffer, this function eliminates repeated
/// heap allocations and improves performance in tight rendering loops.
///
/// # Arguments
///
/// * `filled_width` - The number of characters to include in the gradient segment.
///   If 0, the function clears the buffer and returns an empty string reference.
/// * `ch` - The character to repeat for each position in the gradient (commonly '█', '▓', etc.)
/// * `buffer` - A mutable reference to a `String` that will be cleared and used to build
///   the gradient segment. The buffer's capacity is preserved and extended if needed.
///
/// # Returns
///
/// A string slice (`&str`) reference to the buffer's contents containing the gradient-colored
/// characters with embedded ANSI escape sequences.
///
/// # Performance Benefits
///
/// - **No allocations**: Reuses the provided buffer's existing capacity
/// - **Reduced fragmentation**: Avoids creating temporary strings
/// - **Cache efficiency**: Better memory locality when used in loops
/// - **Optimal for animation**: Perfect for 60fps+ rendering scenarios
///
/// # Examples
///
/// ```rust
/// use bubbletea_rs::gradient::gradient_filled_segment_with_buffer;
///
/// let mut buffer = String::new();
///
/// // Simulate an animated progress bar
/// for progress in 0..=10 {
///     let segment = gradient_filled_segment_with_buffer(progress, '█', &mut buffer);
///     println!("Progress: [{}{}]", segment, " ".repeat(10 - progress));
///     // Buffer is automatically reused for the next iteration
/// }
///
/// // The buffer retains its capacity for future use
/// assert!(buffer.capacity() >= 250); // Approximate capacity after 10 characters
/// ```
///
/// # Usage Patterns
///
/// ```rust,no_run
/// use bubbletea_rs::gradient::gradient_filled_segment_with_buffer;
///
/// // Pattern 1: Reuse buffer in animation loop
/// let mut gradient_buffer = String::new();
/// # fn calculate_progress_width() -> usize { 10 }
/// # fn render_ui_with_progress_bar(bar: &str) { println!("{}", bar); }
/// loop {
///     let width = calculate_progress_width();
///     let bar = gradient_filled_segment_with_buffer(width, '█', &mut gradient_buffer);
///     render_ui_with_progress_bar(bar);
/// #   break; // Prevent infinite loop in doc test
/// }
///
/// // Pattern 2: Multiple gradient elements with separate buffers
/// let mut bar_buffer = String::new();
/// let mut spinner_buffer = String::new();
///
/// let progress_bar = gradient_filled_segment_with_buffer(15, '█', &mut bar_buffer);
/// let loading_spinner = gradient_filled_segment_with_buffer(3, '▓', &mut spinner_buffer);
/// ```
///
/// # Buffer Management
///
/// - The buffer is cleared on each call but its capacity is preserved
/// - If the buffer's capacity is insufficient, it will be extended as needed
/// - The buffer can be reused indefinitely across multiple calls
/// - For optimal performance, pre-allocate buffer capacity if the maximum width is known
///
/// # See Also
///
/// - [`gradient_filled_segment`] - Single-use variant that returns an owned String
/// - [`charm_default_gradient`] - The gradient colors used by this function
/// - [`lerp_rgb`] - The color interpolation function used internally
pub fn gradient_filled_segment_with_buffer(
    filled_width: usize,
    ch: char,
    buffer: &mut String,
) -> &str {
    buffer.clear();

    let (start, end) = charm_default_gradient();
    if filled_width == 0 {
        return buffer;
    }

    // Reserve capacity for the gradient
    let estimated_capacity = filled_width * 25;
    buffer.reserve(estimated_capacity);

    for i in 0..filled_width {
        let t = if filled_width <= 1 {
            0.0
        } else {
            i as f64 / (filled_width - 1) as f64
        };
        let (r, g, b) = lerp_rgb(start, end, t);

        // Manually construct ANSI escape sequence
        buffer.push_str("\x1b[38;2;");
        write_u8_to_string(buffer, r);
        buffer.push(';');
        write_u8_to_string(buffer, g);
        buffer.push(';');
        write_u8_to_string(buffer, b);
        buffer.push('m');
        buffer.push(ch);
        buffer.push_str("\x1b[0m");
    }
    buffer
}
