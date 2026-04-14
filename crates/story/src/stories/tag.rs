use crate::prelude::*;

pub struct TagStory;

impl Render for TagStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap(Spacing::Large.pixels())
            .child(example_group(
                "Tag colors",
                vec![
                    example("Default", Tag::new("Default").into_any_element()),
                    example("Accent", Tag::new("Accent").color(Color::Accent).into_any_element()),
                    example("Success", Tag::new("Success").color(Color::Success).into_any_element()),
                    example("Warning", Tag::new("Warning").color(Color::Warning).into_any_element()),
                    example("Error", Tag::new("Error").color(Color::Error).into_any_element()),
                    example("Info", Tag::new("Info").color(Color::Info).into_any_element()),
                ],
            ))
            .child(example_group(
                "Tag sizes",
                vec![
                    example("Small", Tag::new("Small").size(TagSize::Small).into_any_element()),
                    example("Medium", Tag::new("Medium").size(TagSize::Medium).into_any_element()),
                ],
            ))
            .child(example_group(
                "Outline mode",
                vec![
                    example("Success outline", Tag::new("Success").color(Color::Success).outline(true).into_any_element()),
                    example("Error outline", Tag::new("Error").color(Color::Error).outline(true).into_any_element()),
                    example("Accent outline", Tag::new("Accent").color(Color::Accent).outline(true).into_any_element()),
                ],
            ))
    }
}

pub fn build(_window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_cx| TagStory).into()
}
