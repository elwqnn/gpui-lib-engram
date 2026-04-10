//! [`SplitButton`] — a two-part button with a primary action on the left
//! and a secondary action (e.g. dropdown trigger) on the right, visually
//! separated by a divider.

use engram_theme::ActiveTheme;
use gpui::{
    AnyElement, App, BoxShadow, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    hsla, point, prelude::*, px, relative,
};

use crate::components::button::button_like::ButtonLike;
use crate::components::button::icon_button::IconButton;
use crate::components::stack::h_flex;
/// What kind of element the left side of the split button contains.
pub enum SplitButtonKind {
    ButtonLike(ButtonLike),
    IconButton(IconButton),
}

impl From<ButtonLike> for SplitButtonKind {
    fn from(b: ButtonLike) -> Self {
        Self::ButtonLike(b)
    }
}

impl From<IconButton> for SplitButtonKind {
    fn from(b: IconButton) -> Self {
        Self::IconButton(b)
    }
}

/// Visual style of the split button container.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SplitButtonStyle {
    Filled,
    Outlined,
    Transparent,
}

/// A button with two parts separated by a thin divider.
#[derive(IntoElement)]
pub struct SplitButton {
    left: SplitButtonKind,
    right: AnyElement,
    style: SplitButtonStyle,
}

impl SplitButton {
    pub fn new(left: impl Into<SplitButtonKind>, right: AnyElement) -> Self {
        Self {
            left: left.into(),
            right,
            style: SplitButtonStyle::Filled,
        }
    }

    pub fn style(mut self, style: SplitButtonStyle) -> Self {
        self.style = style;
        self
    }
}

impl RenderOnce for SplitButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_filled_or_outlined = matches!(
            self.style,
            SplitButtonStyle::Filled | SplitButtonStyle::Outlined
        );

        h_flex()
            .when(is_filled_or_outlined, |this| {
                this.rounded_sm()
                    .border_1()
                    .border_color(cx.theme().colors().border.opacity(0.8))
            })
            .when(self.style == SplitButtonStyle::Transparent, |this| {
                this.gap_px()
            })
            .child(div().flex_grow().child(match self.left {
                SplitButtonKind::ButtonLike(b) => b.into_any_element(),
                SplitButtonKind::IconButton(b) => b.into_any_element(),
            }))
            .child(
                div()
                    .h(relative(0.8))
                    .w_px()
                    .bg(cx.theme().colors().border.opacity(0.5)),
            )
            .child(self.right)
            .when(self.style == SplitButtonStyle::Filled, |this| {
                this.bg(cx.theme().colors().element_background)
                    .shadow(vec![BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.16),
                        offset: point(px(0.), px(1.)),
                        blur_radius: px(0.),
                        spread_radius: px(0.),
                    }])
            })
    }
}
