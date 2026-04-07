//! List and ListItem — the workhorse of sidebars, command palettes, and
//! settings panes.
//!
//! [`List`] is a thin vertical container: stack of children plus an optional
//! header and empty-state message. [`ListItem`] is a stateful row with an
//! optional start slot (icon / avatar), label content, an optional end slot
//! (accessory / chevron), click handler, and hover / selected styling.

use std::rc::Rc;

use engram_theme::{ActiveTheme, Color, Radius, Spacing, TextSize};
use gpui::{
    AnyElement, AnyView, App, ClickEvent, CursorStyle, ElementId, IntoElement, ParentElement,
    RenderOnce, SharedString, Window, div, prelude::*,
};
use smallvec::SmallVec;

use crate::components::label::Label;
use crate::components::stack::{h_flex, v_flex};
use crate::traits::{ClickHandler, Clickable, Disableable, ToggleState, Toggleable};

type TooltipBuilder = Rc<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>;

/// A single row inside a [`List`]. Supports start / end slots, selection
/// state, disabled state, click handlers, and tooltips.
#[derive(IntoElement)]
pub struct ListItem {
    id: ElementId,
    disabled: bool,
    selected: bool,
    start_slot: Option<AnyElement>,
    end_slot: Option<AnyElement>,
    children: SmallVec<[AnyElement; 2]>,
    cursor_style: CursorStyle,
    on_click: Option<ClickHandler>,
    tooltip: Option<TooltipBuilder>,
}

impl ListItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            disabled: false,
            selected: false,
            start_slot: None,
            end_slot: None,
            children: SmallVec::new(),
            cursor_style: CursorStyle::PointingHand,
            on_click: None,
            tooltip: None,
        }
    }

    pub fn start_slot(mut self, slot: impl IntoElement) -> Self {
        self.start_slot = Some(slot.into_any_element());
        self
    }

    pub fn end_slot(mut self, slot: impl IntoElement) -> Self {
        self.end_slot = Some(slot.into_any_element());
        self
    }

    /// Attach a tooltip builder. Typically used with
    /// [`Tooltip::text`](crate::components::Tooltip::text).
    pub fn tooltip(
        mut self,
        tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        self.tooltip = Some(Rc::new(tooltip));
        self
    }
}

impl Clickable for ListItem {
    fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl Disableable for ListItem {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for ListItem {
    fn toggle_state(mut self, state: impl Into<ToggleState>) -> Self {
        self.selected = state.into().selected();
        self
    }
}

impl ParentElement for ListItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ListItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let interactive = !self.disabled;

        let background = if self.selected {
            Some(colors.element_selected)
        } else {
            None
        };

        h_flex()
            .id(self.id)
            .w_full()
            .gap(Spacing::Small.pixels())
            .px(Spacing::Medium.pixels())
            .py(Spacing::XSmall.pixels())
            .rounded(Radius::Small.pixels())
            .when_some(background, |this, bg| this.bg(bg))
            .when(interactive, |this| {
                this.cursor(self.cursor_style)
                    .hover(|s| s.bg(colors.ghost_element_hover))
                    .active(|s| s.bg(colors.ghost_element_active))
            })
            .when_some(self.start_slot, |this, slot| this.child(slot))
            .child(
                h_flex()
                    .flex_grow()
                    .gap(Spacing::Small.pixels())
                    .children(self.children),
            )
            .when_some(self.end_slot, |this, slot| this.child(slot))
            .when_some(
                (!self.disabled).then_some(self.on_click).flatten(),
                |this, handler| {
                    this.on_click(move |event, window, cx| handler(event, window, cx))
                },
            )
            .when_some(self.tooltip, |this, builder| {
                this.tooltip(move |window, cx| builder(window, cx))
            })
    }
}

/// A vertical container for [`ListItem`]s with an optional header label and a
/// message displayed when the list has no children.
#[derive(IntoElement)]
pub struct List {
    header: Option<SharedString>,
    empty_message: SharedString,
    children: SmallVec<[AnyElement; 2]>,
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl List {
    pub fn new() -> Self {
        Self {
            header: None,
            empty_message: "No items".into(),
            children: SmallVec::new(),
        }
    }

    pub fn header(mut self, header: impl Into<SharedString>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn empty_message(mut self, message: impl Into<SharedString>) -> Self {
        self.empty_message = message.into();
        self
    }
}

impl ParentElement for List {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for List {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let is_empty = self.children.is_empty();

        v_flex()
            .w_full()
            .gap(Spacing::XXSmall.pixels())
            .when_some(self.header, |this, header| {
                this.child(
                    div().px(Spacing::Medium.pixels()).py(Spacing::XSmall.pixels()).child(
                        Label::new(header)
                            .size(TextSize::Small)
                            .color(Color::Muted),
                    ),
                )
            })
            .map(|this| {
                if is_empty {
                    this.child(
                        div().px(Spacing::Medium.pixels()).py(Spacing::Small.pixels()).child(
                            Label::new(self.empty_message)
                                .size(TextSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                } else {
                    this.children(self.children)
                }
            })
    }
}
