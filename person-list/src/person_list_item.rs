use gpui::{Context, SharedString, Window, div, prelude::*, rgb};

// Reusable PersonListItem component (Flutter ListTile style)
pub struct PersonListItem {
    first_name: SharedString,
    last_name: SharedString,
    likes: u32,
}

impl PersonListItem {
    // Constructor that takes name parameters (like Flutter ListTile)
    pub fn new(first_name: impl Into<SharedString>, last_name: impl Into<SharedString>) -> Self {
        Self {
            first_name: first_name.into(),
            last_name: last_name.into(),
            likes: 0,
        }
    }

    // Component methods
    fn increment_likes(&mut self, cx: &mut Context<Self>) {
        self.likes += 1;
        cx.notify();
    }

    // Render avatar/initials (like Flutter ListTile leading)
    fn render_avatar(&self) -> impl IntoElement {
        let initials = format!(
            "{}{}",
            self.first_name.chars().next().unwrap_or('?'),
            self.last_name.chars().next().unwrap_or('?')
        );

        div()
            .flex()
            .size_12()
            .bg(rgb(0x4a90e2))
            .rounded_full()
            .items_center()
            .justify_center()
            .text_color(rgb(0xffffff))
            .text_lg()
            .font_weight(gpui::FontWeight::BOLD)
            .child(initials)
    }

    // Render name and likes info (like Flutter ListTile title/subtitle)
    fn render_content(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .gap_1()
            .child(
                div()
                    .text_lg()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(rgb(0xffffff))
                    .child(format!("{} {}", &self.first_name, &self.last_name)),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x999999))
                    .child(format!("{} likes", self.likes)),
            )
    }

    // Render like button (like Flutter ListTile trailing)
    fn render_like_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("like-button")
            .flex()
            .items_center()
            .gap_2()
            .px_3()
            .py_2()
            .bg(rgb(0x3399ff))
            .text_color(rgb(0xffffff))
            .rounded_lg()
            .cursor_pointer()
            .hover(|style| style.bg(rgb(0x4da6ff)))
            .on_click(cx.listener(|this, _, _, cx| {
                this.increment_likes(cx);
            }))
            .child("❤️")
            .child(self.likes.to_string())
    }
}

impl Render for PersonListItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .bg(rgb(0x2a2a2a))
            .border_1()
            .border_color(rgb(0x404040))
            .rounded_lg()
            .p_4()
            .gap_4()
            .items_center()
            .hover(|style| style.bg(rgb(0x333333)))
            .child(self.render_avatar())
            .child(self.render_content())
            .child(self.render_like_button(cx))
    }
}
