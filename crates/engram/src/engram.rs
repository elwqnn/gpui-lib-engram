//! engram - a small GPUI-based component library.
//!
//! `engram` is an umbrella facade that re-exports its two sibling crates:
//!
//! - [`gpui_engram_theme`] (aliased as [`theme`]) - color tokens, spacing,
//!   typography, and the [`ActiveTheme`](gpui_engram_theme::ActiveTheme) global.
//! - [`gpui_engram_ui`] (aliased as [`ui`]) - component primitives, shared traits,
//!   embedded icon assets.
//!
//! Downstream apps typically just pull in the prelude:
//!
//! ```no_run
//! use gpui_engram::prelude::*;
//! ```
//!
//! # Quickstart
//!
//! ```text
//! use gpui_engram::prelude::*;
//! use gpui::Application;
//!
//! fn main() {
//!     Application::new()
//!         .with_assets(gpui_engram::ui::Assets)
//!         .run(|cx| {
//!             gpui_engram::theme::init(cx);
//!             gpui_engram::ui::init(cx);
//!             // ... open your window and render a view
//!         });
//! }
//! ```
//!
//! Both `init` calls are required:
//! [`theme::init`](gpui_engram_theme::init) installs the default dark theme as a
//! GPUI global; [`ui::init`](gpui_engram_ui::init) registers keybindings used by
//! [`TextField`](gpui_engram_ui::TextField) and [`Menu`](gpui_engram_ui::Menu).
//!
//! # Stability
//!
//! `engram` is pre-1.0. `gpui` is git-pinned against a specific revision of
//! `zed-industries/zed` - see the workspace `Cargo.toml`. Until `gpui` is on
//! crates.io, `engram` consumers also take it as a git dependency.

#![forbid(unsafe_code)]

pub use gpui_engram_theme as theme;
pub use gpui_engram_ui as ui;

/// Everything you almost always want when building an engram UI.
///
/// This module re-exports every component, the [`ActiveTheme`](gpui_engram_theme::ActiveTheme)
/// trait, semantic tokens ([`Color`](gpui_engram_theme::Color),
/// [`Spacing`](gpui_engram_theme::Spacing), [`Radius`](gpui_engram_theme::Radius),
/// [`TextSize`](gpui_engram_theme::TextSize)), the shared traits
/// ([`Clickable`](gpui_engram_ui::Clickable), [`Disableable`](gpui_engram_ui::Disableable),
/// [`Toggleable`](gpui_engram_ui::Toggleable), [`StyledExt`](gpui_engram_ui::StyledExt)),
/// and the [`h_flex`](gpui_engram_ui::h_flex) / [`v_flex`](gpui_engram_ui::v_flex)
/// stack helpers.
pub mod prelude {
    pub use gpui_engram_ui::prelude::*;
}
