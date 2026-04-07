//! Hand-tuned default themes.

use gpui::SharedString;

use crate::{Appearance, Theme};
use crate::colors::{StatusColors, ThemeColors, hsl, hsla};

/// The default dark theme. Slate base with a blue accent.
pub fn dark() -> Theme {
    Theme {
        name: SharedString::new_static("Engram Dark"),
        appearance: Appearance::Dark,
        colors: ThemeColors {
            background: hsl(220.0, 0.13, 0.10),
            surface_background: hsl(220.0, 0.13, 0.13),
            elevated_surface_background: hsl(220.0, 0.13, 0.17),

            border: hsl(220.0, 0.13, 0.22),
            border_variant: hsl(220.0, 0.13, 0.17),
            border_focused: hsl(210.0, 1.00, 0.60),
            border_selected: hsl(210.0, 1.00, 0.60),
            border_disabled: hsl(220.0, 0.10, 0.18),
            border_transparent: hsla(0.0, 0.00, 0.00, 0.00),

            text: hsl(220.0, 0.15, 0.92),
            text_muted: hsl(220.0, 0.10, 0.65),
            text_placeholder: hsl(220.0, 0.10, 0.50),
            text_disabled: hsl(220.0, 0.10, 0.40),
            text_accent: hsl(210.0, 1.00, 0.70),

            icon: hsl(220.0, 0.15, 0.82),
            icon_muted: hsl(220.0, 0.10, 0.60),
            icon_disabled: hsl(220.0, 0.10, 0.40),
            icon_accent: hsl(210.0, 1.00, 0.70),

            element_background: hsl(220.0, 0.13, 0.20),
            element_hover: hsl(220.0, 0.13, 0.25),
            element_active: hsl(220.0, 0.13, 0.30),
            element_selected: hsl(210.0, 0.65, 0.28),
            element_disabled: hsl(220.0, 0.13, 0.16),

            ghost_element_background: hsla(0.0, 0.00, 0.00, 0.00),
            ghost_element_hover: hsla(220.0, 0.13, 0.50, 0.12),
            ghost_element_active: hsla(220.0, 0.13, 0.50, 0.20),
            ghost_element_selected: hsla(210.0, 0.65, 0.50, 0.18),
            ghost_element_disabled: hsla(0.0, 0.00, 0.00, 0.00),

            status: StatusColors {
                info: hsl(210.0, 0.90, 0.60),
                info_background: hsla(210.0, 0.90, 0.60, 0.18),
                info_border: hsla(210.0, 0.90, 0.60, 0.55),

                success: hsl(142.0, 0.70, 0.50),
                success_background: hsla(142.0, 0.70, 0.50, 0.18),
                success_border: hsla(142.0, 0.70, 0.50, 0.55),

                warning: hsl(38.0, 0.90, 0.55),
                warning_background: hsla(38.0, 0.90, 0.55, 0.18),
                warning_border: hsla(38.0, 0.90, 0.55, 0.55),

                error: hsl(0.0, 0.70, 0.60),
                error_background: hsla(0.0, 0.70, 0.60, 0.18),
                error_border: hsla(0.0, 0.70, 0.60, 0.55),

                hint: hsl(210.0, 0.80, 0.65),
                hint_background: hsla(210.0, 0.80, 0.65, 0.18),
                hint_border: hsla(210.0, 0.80, 0.65, 0.55),

                hidden: hsl(220.0, 0.08, 0.45),
                hidden_background: hsla(220.0, 0.08, 0.45, 0.18),
                hidden_border: hsla(220.0, 0.08, 0.45, 0.55),

                ignored: hsl(220.0, 0.08, 0.38),
                ignored_background: hsla(220.0, 0.08, 0.38, 0.18),
                ignored_border: hsla(220.0, 0.08, 0.38, 0.55),
            },

            accent: hsl(210.0, 1.00, 0.60),
        },
    }
}

/// The default light theme. Slate base with a blue accent.
pub fn light() -> Theme {
    Theme {
        name: SharedString::new_static("Engram Light"),
        appearance: Appearance::Light,
        colors: ThemeColors {
            background: hsl(220.0, 0.20, 0.98),
            surface_background: hsl(220.0, 0.20, 0.96),
            elevated_surface_background: hsl(220.0, 0.20, 1.00),

            border: hsl(220.0, 0.15, 0.85),
            border_variant: hsl(220.0, 0.15, 0.90),
            border_focused: hsl(210.0, 1.00, 0.55),
            border_selected: hsl(210.0, 1.00, 0.55),
            border_disabled: hsl(220.0, 0.15, 0.92),
            border_transparent: hsla(0.0, 0.00, 0.00, 0.00),

            text: hsl(220.0, 0.20, 0.10),
            text_muted: hsl(220.0, 0.10, 0.40),
            text_placeholder: hsl(220.0, 0.10, 0.55),
            text_disabled: hsl(220.0, 0.10, 0.70),
            text_accent: hsl(210.0, 1.00, 0.45),

            icon: hsl(220.0, 0.15, 0.25),
            icon_muted: hsl(220.0, 0.10, 0.45),
            icon_disabled: hsl(220.0, 0.10, 0.70),
            icon_accent: hsl(210.0, 1.00, 0.50),

            element_background: hsl(220.0, 0.15, 0.94),
            element_hover: hsl(220.0, 0.15, 0.90),
            element_active: hsl(220.0, 0.15, 0.85),
            element_selected: hsl(210.0, 0.80, 0.88),
            element_disabled: hsl(220.0, 0.15, 0.96),

            ghost_element_background: hsla(0.0, 0.00, 0.00, 0.00),
            ghost_element_hover: hsla(220.0, 0.13, 0.20, 0.06),
            ghost_element_active: hsla(220.0, 0.13, 0.20, 0.12),
            ghost_element_selected: hsla(210.0, 0.80, 0.50, 0.14),
            ghost_element_disabled: hsla(0.0, 0.00, 0.00, 0.00),

            status: StatusColors {
                info: hsl(210.0, 0.85, 0.48),
                info_background: hsla(210.0, 0.85, 0.48, 0.14),
                info_border: hsla(210.0, 0.85, 0.48, 0.45),

                success: hsl(142.0, 0.65, 0.38),
                success_background: hsla(142.0, 0.65, 0.38, 0.14),
                success_border: hsla(142.0, 0.65, 0.38, 0.45),

                warning: hsl(38.0, 0.90, 0.48),
                warning_background: hsla(38.0, 0.90, 0.48, 0.14),
                warning_border: hsla(38.0, 0.90, 0.48, 0.45),

                error: hsl(0.0, 0.70, 0.50),
                error_background: hsla(0.0, 0.70, 0.50, 0.14),
                error_border: hsla(0.0, 0.70, 0.50, 0.45),

                hint: hsl(210.0, 0.85, 0.45),
                hint_background: hsla(210.0, 0.85, 0.45, 0.14),
                hint_border: hsla(210.0, 0.85, 0.45, 0.45),

                hidden: hsl(220.0, 0.08, 0.62),
                hidden_background: hsla(220.0, 0.08, 0.62, 0.14),
                hidden_border: hsla(220.0, 0.08, 0.62, 0.45),

                ignored: hsl(220.0, 0.08, 0.68),
                ignored_background: hsla(220.0, 0.08, 0.68, 0.14),
                ignored_border: hsla(220.0, 0.08, 0.68, 0.45),
            },

            accent: hsl(210.0, 1.00, 0.50),
        },
    }
}
