//! Button family: text [`Button`] and [`IconButton`], plus shared style
//! variants.

use std::rc::Rc;

use engram_theme::{ActiveTheme, Color, Radius, Spacing, TextSize, ThemeColors};
use gpui::{
    AnyView, App, ClickEvent, CursorStyle, ElementId, IntoElement, RenderOnce, SharedString,
    Window, div, prelude::*, px,
};

use crate::components::icon::{Icon, IconName, IconSize};
use crate::components::label::Label;
use crate::traits::{ClickHandler, Clickable, Disableable, ToggleState, Toggleable};

/// Visual variant of a button.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    /// Solid background in the theme's filled-element color. The default.
    #[default]
    Filled,
    /// Transparent background; shows only on hover. Used for toolbar-style
    /// buttons that shouldn't compete with surrounding content.
    Ghost,
    /// Bordered, transparent background. Used when a ghost button needs to
    /// feel more tangible (form actions, secondary CTAs).
    Outlined,
    /// Accent-colored filled background. Used for primary CTAs.
    Primary,
}

/// Button size presets. These control padding and label text size.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Compact,
    #[default]
    Default,
    Large,
}

impl ButtonSize {
    fn padding(self) -> (Spacing, Spacing) {
        // (horizontal, vertical)
        match self {
            Self::Compact => (Spacing::Small, Spacing::XXSmall),
            Self::Default => (Spacing::Large, Spacing::Small),
            Self::Large => (Spacing::XLarge, Spacing::Medium),
        }
    }

    fn text_size(self) -> TextSize {
        match self {
            Self::Compact => TextSize::Small,
            Self::Default => TextSize::Default,
            Self::Large => TextSize::Large,
        }
    }

    fn icon_size(self) -> IconSize {
        match self {
            Self::Compact => IconSize::Small,
            Self::Default => IconSize::Medium,
            Self::Large => IconSize::Large,
        }
    }
}

type TooltipBuilder = Rc<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>;

/// Resolved fill / foreground colors for a given style + state.
struct Palette {
    background: Option<gpui::Hsla>,
    hover_background: gpui::Hsla,
    active_background: gpui::Hsla,
    foreground: gpui::Hsla,
    border: Option<gpui::Hsla>,
}

fn palette(
    style: ButtonStyle,
    disabled: bool,
    toggled: bool,
    colors: &ThemeColors,
) -> Palette {
    if disabled {
        return Palette {
            background: match style {
                ButtonStyle::Filled | ButtonStyle::Primary | ButtonStyle::Outlined => {
                    Some(colors.element_disabled)
                }
                ButtonStyle::Ghost => None,
            },
            hover_background: colors.element_disabled,
            active_background: colors.element_disabled,
            foreground: colors.text_disabled,
            border: matches!(style, ButtonStyle::Outlined).then_some(colors.border),
        };
    }

    if toggled {
        return Palette {
            background: Some(colors.element_selected),
            hover_background: colors.element_selected,
            active_background: colors.element_selected,
            foreground: colors.text,
            border: matches!(style, ButtonStyle::Outlined).then_some(colors.border_selected),
        };
    }

    match style {
        ButtonStyle::Filled => Palette {
            background: Some(colors.element_background),
            hover_background: colors.element_hover,
            active_background: colors.element_active,
            foreground: colors.text,
            border: None,
        },
        ButtonStyle::Ghost => Palette {
            background: None,
            hover_background: colors.ghost_element_hover,
            active_background: colors.ghost_element_active,
            foreground: colors.text,
            border: None,
        },
        ButtonStyle::Outlined => Palette {
            background: None,
            hover_background: colors.ghost_element_hover,
            active_background: colors.ghost_element_active,
            foreground: colors.text,
            border: Some(colors.border),
        },
        ButtonStyle::Primary => Palette {
            background: Some(colors.accent),
            hover_background: colors.accent,
            active_background: colors.accent,
            foreground: colors.background,
            border: None,
        },
    }
}

/// A text button with an optional leading icon.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    icon: Option<IconName>,
    style: ButtonStyle,
    size: ButtonSize,
    disabled: bool,
    toggle: ToggleState,
    cursor_style: CursorStyle,
    on_click: Option<ClickHandler>,
    tooltip: Option<TooltipBuilder>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            style: ButtonStyle::default(),
            size: ButtonSize::default(),
            disabled: false,
            toggle: ToggleState::default(),
            cursor_style: CursorStyle::PointingHand,
            on_click: None,
            tooltip: None,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
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

impl Clickable for Button {
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

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for Button {
    fn toggle_state(mut self, state: impl Into<ToggleState>) -> Self {
        self.toggle = state.into();
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let palette = palette(self.style, self.disabled, self.toggle.selected(), colors);
        let (pad_x, pad_y) = self.size.padding();
        let text_size = self.size.text_size();
        let icon_size = self.size.icon_size();

        let foreground_color = match self.style {
            // Primary buttons always use their own contrasting foreground and
            // ignore the default text color from `Color::Default`.
            ButtonStyle::Primary if !self.disabled => palette.foreground,
            _ => palette.foreground,
        };

        let label_color = if self.disabled {
            Color::Disabled
        } else {
            match self.style {
                ButtonStyle::Primary => Color::Custom(foreground_color),
                _ => Color::Default,
            }
        };

        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(Spacing::XSmall.pixels())
            .px(pad_x.pixels())
            .py(pad_y.pixels())
            .rounded(Radius::Medium.pixels())
            .text_color(foreground_color)
            .when_some(palette.background, |this, bg| this.bg(bg))
            .when_some(palette.border, |this, border| {
                this.border_1().border_color(border)
            })
            .when(!self.disabled, |this| {
                this.cursor(self.cursor_style)
                    .hover(|s| s.bg(palette.hover_background))
                    .active(|s| s.bg(palette.active_background))
            })
            .when_some(self.icon, |this, icon| {
                this.child(
                    Icon::new(icon)
                        .size(icon_size)
                        .color(label_color),
                )
            })
            .child(Label::new(self.label).size(text_size).color(label_color))
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

/// A square icon-only button. Inherits the same [`ButtonStyle`] and
/// [`ButtonSize`] palette as [`Button`].
#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: IconName,
    style: ButtonStyle,
    size: ButtonSize,
    disabled: bool,
    toggle: ToggleState,
    cursor_style: CursorStyle,
    on_click: Option<ClickHandler>,
    tooltip: Option<TooltipBuilder>,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        Self {
            id: id.into(),
            icon,
            style: ButtonStyle::default(),
            size: ButtonSize::default(),
            disabled: false,
            toggle: ToggleState::default(),
            cursor_style: CursorStyle::PointingHand,
            on_click: None,
            tooltip: None,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
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

impl Clickable for IconButton {
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

impl Disableable for IconButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for IconButton {
    fn toggle_state(mut self, state: impl Into<ToggleState>) -> Self {
        self.toggle = state.into();
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let palette = palette(self.style, self.disabled, self.toggle.selected(), colors);
        let icon_size = self.size.icon_size();

        // Square button: use text_size as the padding step so the hit target
        // grows with the icon.
        let pad = match self.size {
            ButtonSize::Compact => px(4.0),
            ButtonSize::Default => px(6.0),
            ButtonSize::Large => px(8.0),
        };

        let icon_color = if self.disabled {
            Color::Disabled
        } else {
            match self.style {
                ButtonStyle::Primary => Color::Custom(palette.foreground),
                _ => Color::Default,
            }
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .p(pad)
            .rounded(Radius::Medium.pixels())
            .when_some(palette.background, |this, bg| this.bg(bg))
            .when_some(palette.border, |this, border| {
                this.border_1().border_color(border)
            })
            .when(!self.disabled, |this| {
                this.cursor(self.cursor_style)
                    .hover(|s| s.bg(palette.hover_background))
                    .active(|s| s.bg(palette.active_background))
            })
            .child(Icon::new(self.icon).size(icon_size).color(icon_color))
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
