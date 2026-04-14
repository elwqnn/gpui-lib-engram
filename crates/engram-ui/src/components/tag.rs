//! Tag — a compact status indicator label.
//!
//! Tags are rectangular badges with rounded corners, sized variants, and
//! an optional outline mode. They differ from [`Chip`](super::avatar::Chip)
//! in two ways:
//!
//! - **Shape**: Tags use a small corner radius, chips are fully rounded (pill).
//! - **Sizing**: Tags support [`TagSize`] variants; chips are always XSmall.
//!
//! Use a Tag when the indicator sits in a denser layout (tables, lists,
//! sidebars) and the pill shape of a Chip would be too prominent.

use engram_theme::{ActiveTheme, Color, Radius, Spacing};
use gpui::{App, IntoElement, RenderOnce, SharedString, Styled, Window, div, prelude::*};

use crate::components::label::{Label, LabelCommon, LabelSize};

/// Display size for a [`Tag`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TagSize {
    Small,
    #[default]
    Medium,
}

/// A compact, non-interactive status label.
#[derive(IntoElement)]
pub struct Tag {
    label: SharedString,
    color: Color,
    size: TagSize,
    outline: bool,
}

impl Tag {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            color: Color::Default,
            size: TagSize::default(),
            outline: false,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: TagSize) -> Self {
        self.size = size;
        self
    }

    /// Render as an outlined tag (transparent background, colored border).
    pub fn outline(mut self, outline: bool) -> Self {
        self.outline = outline;
        self
    }
}

impl RenderOnce for Tag {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let status = &colors.status;

        let fg = self.color;
        let (bg, border) = if self.outline {
            (gpui::transparent_black(), self.color.hsla(&colors))
        } else {
            match self.color {
                Color::Success => (status.success_background, status.success_border),
                Color::Warning => (status.warning_background, status.warning_border),
                Color::Error => (status.error_background, status.error_border),
                Color::Info => (status.info_background, status.info_border),
                Color::Accent => (colors.element_background, colors.border),
                _ => (colors.element_background, colors.border),
            }
        };

        let (label_size, px_x, px_y) = match self.size {
            TagSize::Small => (LabelSize::XSmall, Spacing::XSmall, Spacing::XXSmall),
            TagSize::Medium => (LabelSize::Small, Spacing::Small, Spacing::XXSmall),
        };

        div()
            .px(px_x.pixels())
            .py(px_y.pixels())
            .rounded(Radius::Small.pixels())
            .border_1()
            .border_color(border)
            .bg(bg)
            .child(Label::new(self.label).size(label_size).color(fg))
    }
}
