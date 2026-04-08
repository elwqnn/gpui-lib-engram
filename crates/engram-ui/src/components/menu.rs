//! Menu — vertical list of clickable items, separators, and headers,
//! packaged as the content of a [`Popover`](super::popover::Popover).
//!
//! Engram's menu is intentionally far simpler than Zed's `ContextMenu`,
//! which carries focus rings, keyboard navigation, submenus, and an
//! action / dispatch system. Here we have:
//!
//! - [`Menu`] is a builder that collects [`MenuItem`]s.
//! - Items are: `entry` (label + optional icon + click handler), `separator`,
//!   `header`, and `keybinding_entry` (entry with a trailing key chip).
//!
//! The caller is responsible for opening / closing the menu by toggling its
//! own state field, then placing the rendered `Menu` inside an
//! [`anchored_popover`](super::popover::anchored_popover) call.

use std::rc::Rc;

use engram_theme::{ActiveTheme, Color, Spacing};
use gpui::{
    AnyElement, App, ClickEvent, ElementId, IntoElement, ParentElement, RenderOnce,
    SharedString, Window, div, prelude::*, px,
};
use smallvec::SmallVec;

use crate::components::icon::{Icon, IconName, IconSize};
use crate::components::keybinding::KeyBinding;
use crate::components::label::{Label, LabelCommon, LabelSize};
use crate::components::popover::Popover;
use crate::components::stack::{h_flex, v_flex};
use crate::traits::ClickHandler;

/// One row inside a [`Menu`].
pub enum MenuItem {
    /// A clickable entry with a label, optional leading icon, and an
    /// optional trailing keybinding hint.
    Entry {
        id: ElementId,
        label: SharedString,
        icon: Option<IconName>,
        keybinding: Option<Vec<SharedString>>,
        disabled: bool,
        on_click: Option<ClickHandler>,
    },
    /// A non-interactive header row, used to title a group of entries.
    Header(SharedString),
    /// A 1-pixel divider between groups.
    Separator,
}

/// A vertical menu, rendered as the body of a popover.
#[derive(IntoElement)]
pub struct Menu {
    items: SmallVec<[MenuItem; 6]>,
    min_width: Option<gpui::Pixels>,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            items: SmallVec::new(),
            min_width: Some(px(180.0)),
        }
    }

    /// Override the popover's minimum width. The default (180px) gives
    /// menu rows a stable, non-jittery width.
    pub fn min_width(mut self, width: gpui::Pixels) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Append a clickable entry.
    pub fn entry(
        mut self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(MenuItem::Entry {
            id: id.into(),
            label: label.into(),
            icon: None,
            keybinding: None,
            disabled: false,
            on_click: Some(Rc::new(on_click)),
        });
        self
    }

    /// Append an entry with a leading icon.
    pub fn entry_with_icon(
        mut self,
        id: impl Into<ElementId>,
        icon: IconName,
        label: impl Into<SharedString>,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(MenuItem::Entry {
            id: id.into(),
            label: label.into(),
            icon: Some(icon),
            keybinding: None,
            disabled: false,
            on_click: Some(Rc::new(on_click)),
        });
        self
    }

    /// Append an entry with a trailing key chip strip (e.g. `["Cmd", "S"]`).
    pub fn keybinding_entry<I, S>(
        mut self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        keys: I,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SharedString>,
    {
        self.items.push(MenuItem::Entry {
            id: id.into(),
            label: label.into(),
            icon: None,
            keybinding: Some(keys.into_iter().map(Into::into).collect()),
            disabled: false,
            on_click: Some(Rc::new(on_click)),
        });
        self
    }

    /// Append a disabled (non-clickable, muted) entry.
    pub fn disabled_entry(
        mut self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
    ) -> Self {
        self.items.push(MenuItem::Entry {
            id: id.into(),
            label: label.into(),
            icon: None,
            keybinding: None,
            disabled: true,
            on_click: None,
        });
        self
    }

    /// Append a non-interactive header that titles the next group of entries.
    pub fn header(mut self, label: impl Into<SharedString>) -> Self {
        self.items.push(MenuItem::Header(label.into()));
        self
    }

    /// Append a 1px divider between groups.
    pub fn separator(mut self) -> Self {
        self.items.push(MenuItem::Separator);
        self
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Menu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let mut popover = Popover::new();
        if let Some(w) = self.min_width {
            popover = popover.min_width(w);
        }

        let rows: Vec<AnyElement> = self
            .items
            .into_iter()
            .map(|item| match item {
                MenuItem::Separator => div()
                    .my(px(2.0))
                    .h(px(1.0))
                    .bg(colors.border_variant)
                    .into_any_element(),
                MenuItem::Header(text) => div()
                    .px(Spacing::Medium.pixels())
                    .pt(px(6.0))
                    .pb(px(2.0))
                    .child(
                        Label::new(text)
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .into_any_element(),
                MenuItem::Entry {
                    id,
                    label,
                    icon,
                    keybinding,
                    disabled,
                    on_click,
                } => {
                    let label_color = if disabled { Color::Disabled } else { Color::Default };
                    let row = h_flex()
                        .id(id)
                        .w_full()
                        .gap(Spacing::Small.pixels())
                        .px(Spacing::Medium.pixels())
                        .py(px(4.0))
                        .items_center()
                        .when(!disabled, |this| {
                            this.cursor_pointer().hover(|s| s.bg(colors.ghost_element_hover))
                        })
                        .when_some(icon, |this, icon| {
                            this.child(
                                Icon::new(icon)
                                    .size(IconSize::Small)
                                    .color(if disabled { Color::Disabled } else { Color::Muted }),
                            )
                        })
                        .child(
                            div()
                                .flex_grow()
                                .child(Label::new(label).color(label_color)),
                        )
                        .when_some(keybinding, |this, keys| this.child(KeyBinding::new(keys)))
                        .when_some(
                            (!disabled).then_some(on_click).flatten(),
                            |this, handler| {
                                this.on_click(move |event, window, cx| handler(event, window, cx))
                            },
                        );
                    row.into_any_element()
                }
            })
            .collect();

        popover.child(v_flex().gap(Spacing::None.pixels()).children(rows))
    }
}
