//! HoverCard - a richer tooltip-like surface for preview content.
//!
//! Where [`Tooltip`](super::tooltip) shows a title and optional metadata
//! line, `HoverCard` is a full [`Popover`]-backed card that accepts
//! arbitrary children - user profiles, link previews, definition
//! summaries, etc.
//!
//! Like `Tooltip`, `HoverCard` is a view (`impl Render`) so it can be
//! handed to GPUI's `.tooltip(builder)` method on any stateful element.
//! The [`HoverCard::build`] helper produces the closure.

use gpui::{
    AnyElement, AnyView, App, Context, IntoElement, ParentElement, Pixels, Render, SharedString,
    Styled, Window, div, prelude::*,
};
use gpui_engram_theme::{ActiveTheme, Radius, Spacing};
use smallvec::SmallVec;

use crate::components::label::{Label, LabelCommon, LabelSize};
use crate::components::stack::v_flex;
use crate::styles::ElevationIndex;

/// A rich hover card for preview content.
pub struct HoverCard {
    title: Option<SharedString>,
    min_width: Option<Pixels>,
    children: SmallVec<[AnyElement; 2]>,
}

impl HoverCard {
    pub fn new() -> Self {
        Self {
            title: None,
            min_width: None,
            children: SmallVec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn min_width(mut self, width: Pixels) -> Self {
        self.min_width = Some(width);
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Build a tooltip-builder closure that produces this hover card.
    /// Pass the result directly to gpui's `.tooltip(...)` method.
    pub fn build(
        make: impl Fn() -> HoverCard + 'static,
    ) -> impl Fn(&mut Window, &mut App) -> AnyView + 'static {
        move |_window, cx| cx.new(|_| make()).into()
    }
}

impl Default for HoverCard {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for HoverCard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Render for HoverCard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();

        div()
            .pl(Spacing::XSmall.pixels())
            .pt(Spacing::Small.pixels())
            .child(
                v_flex()
                    .when_some(self.min_width, |this, w| this.min_w(w))
                    .gap(Spacing::Small.pixels())
                    .px(Spacing::Medium.pixels())
                    .py(Spacing::Small.pixels())
                    .rounded(Radius::Medium.pixels())
                    .bg(colors.elevated_surface_background)
                    .border_1()
                    .border_color(colors.border)
                    .shadow(ElevationIndex::ElevatedSurface.shadow(cx))
                    .when_some(self.title.take(), |this, title| {
                        this.child(
                            Label::new(title)
                                .size(LabelSize::Small)
                                .weight(gpui::FontWeight::SEMIBOLD),
                        )
                    })
                    .children(self.children.drain(..)),
            )
    }
}
