//! engram — a small GPUI-based component library.
//!
//! `engram` is an umbrella facade that re-exports its two sibling crates:
//!
//! - [`engram_theme`] (aliased as [`theme`]) — color tokens, spacing,
//!   typography, and the [`ActiveTheme`](engram_theme::ActiveTheme) global.
//! - [`engram_ui`] (aliased as [`ui`]) — component primitives, shared traits,
//!   embedded icon assets.
//!
//! Downstream apps typically just pull in the prelude:
//!
//! ```no_run
//! use engram::prelude::*;
//! ```
//!
//! # Quickstart
//!
//! ```text
//! use engram::prelude::*;
//! use gpui::Application;
//!
//! fn main() {
//!     Application::new()
//!         .with_assets(engram::ui::Assets)
//!         .run(|cx| {
//!             engram::theme::init(cx);
//!             engram::ui::init(cx);
//!             // ... open your window and render a view
//!         });
//! }
//! ```
//!
//! Both `init` calls are required:
//! [`theme::init`](engram_theme::init) installs the default dark theme as a
//! GPUI global; [`ui::init`](engram_ui::init) registers keybindings used by
//! [`TextField`](engram_ui::TextField) and [`Menu`](engram_ui::Menu).
//!
//! # Stability
//!
//! `engram` is pre-1.0. `gpui` is git-pinned against a specific revision of
//! `zed-industries/zed` — see the workspace `Cargo.toml`. Until `gpui` is on
//! crates.io, `engram` consumers also take it as a git dependency.

#![forbid(unsafe_code)]

pub use engram_theme as theme;
pub use engram_ui as ui;

/// Everything you almost always want when building an engram UI.
///
/// This module re-exports every component, the [`ActiveTheme`](engram_theme::ActiveTheme)
/// trait, semantic tokens ([`Color`](engram_theme::Color),
/// [`Spacing`](engram_theme::Spacing), [`Radius`](engram_theme::Radius),
/// [`TextSize`](engram_theme::TextSize)), the shared traits
/// ([`Clickable`](engram_ui::Clickable), [`Disableable`](engram_ui::Disableable),
/// [`Toggleable`](engram_ui::Toggleable), [`StyledExt`](engram_ui::StyledExt)),
/// and the [`h_flex`](engram_ui::h_flex) / [`v_flex`](engram_ui::v_flex)
/// stack helpers.
pub mod prelude {
    pub use engram_ui::prelude::*;
}
