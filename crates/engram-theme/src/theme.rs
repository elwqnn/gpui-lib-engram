//! Theme foundation for engram: color tokens, spacing, typography, and
//! the [`ActiveTheme`] accessor registered as a GPUI global.

mod colors;
mod default;
mod tokens;

use std::sync::Arc;

use gpui::{App, Global, SharedString};

pub use crate::colors::{Color, StatusColors, ThemeColors};
pub use crate::default::{dark as default_dark, light as default_light};
pub use crate::tokens::{Radius, Spacing, TextSize};

/// Whether a theme is a light or dark variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Appearance {
    Light,
    Dark,
}

impl Appearance {
    pub fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }
}

/// The primary theme definition. Bundles semantic color tokens with
/// appearance metadata for components that need to branch on light vs. dark.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub name: SharedString,
    pub appearance: Appearance,
    pub colors: ThemeColors,
}

impl Theme {
    pub fn colors(&self) -> &ThemeColors {
        &self.colors
    }

    pub fn appearance(&self) -> Appearance {
        self.appearance
    }
}

/// Trait for accessing the currently active [`Theme`] through any GPUI context.
pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        &self.global::<GlobalTheme>().0
    }
}

/// Wraps the active [`Theme`] so it can live as a [`Global`] on [`App`].
pub struct GlobalTheme(pub Arc<Theme>);

impl Global for GlobalTheme {}

/// Install `theme` as the active theme for the app.
pub fn set_theme(theme: Theme, cx: &mut App) {
    cx.set_global(GlobalTheme(Arc::new(theme)));
}

/// Install the default dark theme. Convenience used by most examples.
pub fn init(cx: &mut App) {
    set_theme(default_dark(), cx);
}
