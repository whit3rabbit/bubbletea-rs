use crossterm::style::{style, Color, Stylize};

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
    let mut s = String::with_capacity(filled_width * 10);
    for i in 0..filled_width {
        let t = if filled_width <= 1 { 1.0 } else { i as f64 / (filled_width - 1) as f64 };
        let (r, g, b) = lerp_rgb(start, end, t);
        s.push_str(&style(ch).with(Color::Rgb { r, g, b }).to_string());
    }
    s
}
