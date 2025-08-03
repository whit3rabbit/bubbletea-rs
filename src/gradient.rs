// Note: Crossterm style imports removed for manual ANSI sequence generation for better performance

/// Fast u8 to string conversion without allocations.
/// Manually converts a u8 (0-255) to decimal string and appends to buffer.
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

/// Returns the Charm default gradient endpoints used by Bubble Tea progress bars.
/// Start: #FF7CCB, End: #FDFF8C
#[inline]
pub fn charm_default_gradient() -> ((u8, u8, u8), (u8, u8, u8)) {
    ((0xFF, 0x7C, 0xCB), (0xFD, 0xFF, 0x8C))
}

/// Compute the RGB at position t in [0,1] between start and end.
#[inline]
pub fn lerp_rgb(start: (u8, u8, u8), end: (u8, u8, u8), t: f64) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    let r = (start.0 as f64 + (end.0 as f64 - start.0 as f64) * t).round() as u8;
    let g = (start.1 as f64 + (end.1 as f64 - start.1 as f64) * t).round() as u8;
    let b = (start.2 as f64 + (end.2 as f64 - start.2 as f64) * t).round() as u8;
    (r, g, b)
}

/// Build a gradient-colored filled segment of given width using the provided character.
/// Colors follow Charm's default gradient.
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
            1.0
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

/// Optimized version that reuses a buffer for better performance.
/// Useful when calling gradient_filled_segment repeatedly.
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
            1.0
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
