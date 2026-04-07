//! engram: umbrella facade that re-exports [`engram_theme`] and [`engram_ui`].
//!
//! Downstream users should typically write `use engram::prelude::*;`.

pub use engram_theme as theme;
pub use engram_ui as ui;

/// Everything you almost always want when building an engram UI.
pub mod prelude {
    pub use engram_ui::prelude::*;
}
