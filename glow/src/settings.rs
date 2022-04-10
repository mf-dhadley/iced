//! Configure a renderer.
pub use iced_graphics::Antialiasing;

/// The settings of a [`Backend`].
///
/// [`Backend`]: crate::Backend
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settings {
    /// The bytes of the font that will be used by default.
    ///
    /// If `None` is provided, a default system font will be chosen.
    pub default_font: Option<&'static [u8]>,

    /// The default size of text.
    ///
    /// By default, it will be set to 20.
    pub default_text_size: u16,

    /// If enabled, spread text workload in multiple threads when multiple cores
    /// are available.
    ///
    /// By default, it is disabled.
    pub text_multithreading: bool,

    /// The amount of bits to use to store depth values.
    pub depth_bits: u8,

    /// The amount of bits to use to store alpha values.
    pub alpha_bits: u8,

    /// The amount of bits to use to store color values. Note: this is the sum
    /// of each `r + g + b` amount. For example. A value of 24 would mean 8 bits
    /// for each color.
    pub color_bits: u8,

    /// The antialiasing strategy that will be used for triangle primitives.
    ///
    /// By default, it is `None`.
    pub antialiasing: Option<Antialiasing>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            default_font: None,
            default_text_size: 20,
            text_multithreading: false,
            antialiasing: None,

            color_bits: 24,
            depth_bits: 24,
            alpha_bits: 0,
        }
    }
}

impl Settings {
    /// Creates new [`Settings`] using environment configuration.
    ///
    /// Currently, this is equivalent to calling [`Settings::default`].
    pub fn from_env() -> Self {
        Self::default()
    }
}
