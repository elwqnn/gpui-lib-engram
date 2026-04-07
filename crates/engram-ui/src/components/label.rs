//! Static text label.

use engram_theme::{ActiveTheme, Color, TextSize};
use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*};

/// A single-line text label with a semantic color and size.
#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    size: TextSize,
    color: Color,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            size: TextSize::default(),
            color: Color::default(),
        }
    }

    pub fn size(mut self, size: TextSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        div()
            .text_size(self.size.pixels())
            .text_color(self.color.hsla(colors))
            .child(self.text)
    }
}
