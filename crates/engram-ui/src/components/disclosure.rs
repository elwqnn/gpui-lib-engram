//! Disclosure — a chevron toggle for expandable sections / tree rows.
//!
//! Tracks Zed's `Disclosure` shape: an interactive icon button that flips
//! between a "closed" and "opened" chevron and emits a click event. We
//! render it inline rather than wrapping our [`IconButton`] so the icon
//! coloring stays consistent with neighboring list rows.

use std::rc::Rc;

use engram_theme::{ActiveTheme, Color, Radius, Spacing};
use gpui::{
    App, ClickEvent, CursorStyle, ElementId, IntoElement, RenderOnce, Window, div, prelude::*,
};

use crate::components::icon::{Icon, IconName, IconSize};
use crate::traits::{ClickHandler, Clickable, Disableable};

#[derive(IntoElement)]
pub struct Disclosure {
    id: ElementId,
    is_open: bool,
    disabled: bool,
    opened_icon: IconName,
    closed_icon: IconName,
    cursor_style: CursorStyle,
    on_toggle: Option<ClickHandler>,
}

impl Disclosure {
    pub fn new(id: impl Into<ElementId>, is_open: bool) -> Self {
        Self {
            id: id.into(),
            is_open,
            disabled: false,
            opened_icon: IconName::ChevronDown,
            closed_icon: IconName::ChevronRight,
            cursor_style: CursorStyle::PointingHand,
            on_toggle: None,
        }
    }

    pub fn opened_icon(mut self, icon: IconName) -> Self {
        self.opened_icon = icon;
        self
    }

    pub fn closed_icon(mut self, icon: IconName) -> Self {
        self.closed_icon = icon;
        self
    }
}

impl Disableable for Disclosure {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Clickable for Disclosure {
    fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl RenderOnce for Disclosure {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let icon_color = if self.disabled { Color::Disabled } else { Color::Muted };
        let icon = if self.is_open { self.opened_icon } else { self.closed_icon };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .p(Spacing::XXSmall.pixels())
            .rounded(Radius::Small.pixels())
            .when(!self.disabled, |this| {
                this.cursor(self.cursor_style)
                    .hover(|s| s.bg(colors.ghost_element_hover))
                    .active(|s| s.bg(colors.ghost_element_active))
            })
            .child(Icon::new(icon).size(IconSize::Small).color(icon_color))
            .when_some(
                (!self.disabled).then_some(self.on_toggle).flatten(),
                |this, handler| {
                    this.on_click(move |event, window, cx| handler(event, window, cx))
                },
            )
    }
}
