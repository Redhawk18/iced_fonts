//! Bootstrap Required Icons.
//! Machine generated code. Do not change!

/// Bootstrap RequiredIcons
#[derive(Copy, Clone, Debug, Hash)]
pub enum RequiredIcons {
    /// caret-down-fill
    CaretDownFill,
    /// caret-left-fill
    CaretLeftFill,
    /// caret-right-fill
    CaretRightFill,
    /// caret-up-fill
    CaretUpFill,
    /// check
    Check,
    /// x
    X,
}

/// Converts an Bootstrap into a char.
#[must_use]
pub const fn icon_to_char(icon: RequiredIcons) -> char {
    match icon {
        RequiredIcons::CaretDownFill => '\u{f217}',
        RequiredIcons::CaretLeftFill => '\u{f21b}',
        RequiredIcons::CaretRightFill => '\u{f21f}',
        RequiredIcons::CaretUpFill => '\u{f223}',
        RequiredIcons::Check => '\u{f25c}',
        RequiredIcons::X => '\u{f5ae}',
    }
}

/// Converts an Bootstrap into a String.
#[must_use]
pub fn icon_to_string(icon: RequiredIcons) -> String {
    icon_to_char(icon).to_string()
}

impl From<RequiredIcons> for char {
    fn from(icon: RequiredIcons) -> Self {
        icon_to_char(icon)
    }
}

impl From<RequiredIcons> for String {
    fn from(icon: RequiredIcons) -> Self {
        format!("{}", icon_to_char(icon))
    }
}

impl std::fmt::Display for RequiredIcons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", icon_to_char(*self))
    }
}
