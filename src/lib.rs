use iced_core::Font;

pub mod required;

pub const REQUIRED_FONT_BYTES: &[u8] = include_bytes!("../fonts/required-icons.ttf");
pub const REQUIRED_FONT: Font = Font::with_name("required-icons");

#[cfg(feature = "bootstrap")]
pub mod bootstrap;

#[cfg(feature = "bootstrap")]
pub use bootstrap::Bootstrap;

#[cfg(feature = "bootstrap")]
/// The default icon font bytes for loading the font into iced.
pub const BOOTSTRAP_FONT_BYTES: &[u8] = include_bytes!("../fonts/bootstrap-icons.ttf");

#[cfg(feature = "bootstrap")]
/// The bootstrap icon font.
pub const BOOTSTRAP_FONT: Font = Font::with_name("bootstrap-icons");

#[cfg(feature = "nerd")]
pub mod nerd;
#[cfg(feature = "nerd")]
pub use nerd::Nerd;

#[cfg(feature = "nerd")]
/// the icon font that has all nerd fonts.
pub const NERD_FONT_BYTES: &[u8] = include_bytes!("../fonts/nerd-icons.ttf");

#[cfg(feature = "nerd")]
/// The nerd icon font.
pub const NERD_FONT: Font = Font::with_name("Symbols Nerd Font");
