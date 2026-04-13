use crate::prelude::*;
use crate::layout::{example, example_group};

pub struct HoverCardStory;

impl Render for HoverCardStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let card = cx.new(|_| {
            HoverCard::new()
                .title("Preview")
                .min_width(px(220.0))
                .child(Label::new("Alice Smith").size(LabelSize::Default))
                .child(Label::new("Senior Engineer at Acme").color(Color::Muted))
                .child(
                    h_flex()
                        .gap(Spacing::Small.pixels())
                        .child(Icon::new(IconName::Mail).size(IconSize::Small).color(Color::Muted))
                        .child(Label::new("alice@example.com").size(LabelSize::Small).color(Color::Muted)),
                )
        });

        v_flex()
            .gap(Spacing::Large.pixels())
            .child(example_group(
                "HoverCard",
                vec![
                    example(
                        "Rich content card",
                        card.into_any_element(),
                    ),
                    example(
                        "Minimal card",
                        cx.new(|_| {
                            HoverCard::new()
                                .title("Link Preview")
                                .child(Label::new("https://example.com").color(Color::Accent))
                        })
                        .into_any_element(),
                    ),
                ],
            ))
    }
}

pub fn build(_window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_cx| HoverCardStory).into()
}
