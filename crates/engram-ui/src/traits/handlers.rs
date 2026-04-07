//! Shared type aliases for component event handlers.
//!
//! Every interactive component in engram stores its callbacks as
//! `Rc<dyn Fn(...) + 'static>`. Before this module existed, each component
//! re-declared the same type alias (`ClickHandler`, `DismissHandler`,
//! `MenuClickHandler`, …) with slightly different names — a real maintenance
//! trap when the signature needs to change. Keeping the aliases here means:
//!
//! - there's **one** place to update the handler signature,
//! - component code reads uniformly (`ClickHandler` means the same thing
//!   everywhere),
//! - new components can pick an existing alias instead of minting a new one.
//!
//! All handlers use `Rc` rather than `Box` so that a single handler can be
//! cloned into multiple closures (e.g. `on_click` + `on_key_down`) within a
//! single render pass.
//!
//! If you need a handler shape that isn't represented here, add it — don't
//! re-declare a local alias.

use std::rc::Rc;

use gpui::{App, ClickEvent, Window};

use crate::traits::ToggleState;

/// Handler fired on a mouse click. The canonical shape for buttons, list
/// items, tabs, menu entries, disclosure toggles, and modal-backdrop dismisses.
pub type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// Handler fired with a string payload. Used by text fields for both
/// `on_change` and `on_submit`.
pub type StringHandler = Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>;

/// Handler fired when a toggleable element flips state. Used by checkboxes
/// and switches; the handler receives the *new* state after the flip.
pub type ToggleHandler = Rc<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>;

/// Handler fired when an overlay (modal, popover) wants to close itself.
/// No event payload — it's called from both mouse (backdrop click) and
/// keyboard (Escape) paths, so there's no single meaningful event.
pub type DismissHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
