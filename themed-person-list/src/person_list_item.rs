use gpui::{Context, SharedString, Window, div, prelude::*};

use crate::theme::Theme;

// Simplified PersonListItem component
pub struct PersonListItem {
    first_name: SharedString,
    last_name: SharedString,
    likes: u32,
}

impl PersonListItem {
    pub fn new(first_name: impl Into<SharedString>, last_name: impl Into<SharedString>) -> Self {
        Self {
            first_name: first_name.into(),
            last_name: last_name.into(),
            likes: 0,
        }
    }

    fn increment_likes(&mut self, cx: &mut Context<Self>) {
        self.likes += 1;
        cx.notify();
    }
}

impl Render for PersonListItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>().clone();
        
        div()
            .flex()
            .gap_4()
            .p_4()
            .m_2()
            .bg(theme.surface_secondary)
            .border_1()
            .border_color(theme.border_subtle)
            .rounded_lg()
            .items_center()
            // Avatar with initials
            .child(
                div()
                    .size_12()
                    .bg(theme.accent_primary)
                    .rounded_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(theme.text_on_accent)
                    .font_weight(gpui::FontWeight::BOLD)
                    .child(format!(
                        "{}{}",
                        self.first_name.chars().next().unwrap_or('?'),
                        self.last_name.chars().next().unwrap_or('?')
                    )),
            )
            // Name and likes info
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .child(
                        div()
                            .text_color(theme.text_primary)
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .child(format!("{} {}", &self.first_name, &self.last_name)),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_secondary)
                            .child(format!("{} likes", self.likes)),
                    ),
            )
            // Like button
            .child(
                div()
                    .id("like-button")
                    .px_3()
                    .py_2()
                    .bg(theme.accent_primary)
                    .text_color(theme.text_on_accent)
                    .rounded_lg()
                    .cursor_pointer()
                    .hover(|style| style.bg(theme.accent_hover))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.increment_likes(cx);
                    }))
                    .child(format!("❤️ {}", self.likes)),
            )
    }
}
