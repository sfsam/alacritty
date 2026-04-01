use serde::Serialize;

use alacritty_config_derive::ConfigDeserialize;

use crate::display::color::Rgb;

/// Default track color: dark gray.
const DEFAULT_TRACK_COLOR: Rgb = Rgb::new(34, 34, 34);

/// Default thumb color: medium gray.
const DEFAULT_THUMB_COLOR: Rgb = Rgb::new(68, 68, 68);

/// Scrollbar width in logical pixels.
const SCROLLBAR_WIDTH: f32 = 12.;

/// Scrollbar configuration.
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScrollbarConfig {
    /// Show the scrollbar.
    pub show: bool,

    /// Background color of the scrollbar track.
    pub track_color: Rgb,

    /// Color of the scrollbar thumb.
    pub thumb_color: Rgb,
}

impl Default for ScrollbarConfig {
    fn default() -> Self {
        Self { show: false, track_color: DEFAULT_TRACK_COLOR, thumb_color: DEFAULT_THUMB_COLOR }
    }
}

impl ScrollbarConfig {
    /// Returns the physical scrollbar width in pixels, scaled by `scale_factor`.
    ///
    /// Returns `0.0` when the scrollbar is hidden.
    pub fn physical_width(&self, scale_factor: f32) -> f32 {
        if self.show { (SCROLLBAR_WIDTH * scale_factor).floor() } else { 0. }
    }
}
