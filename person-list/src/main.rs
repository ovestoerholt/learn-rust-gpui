use gpui::{
    App, AppContext, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

mod person_list_item;
use person_list_item::PersonListItem;

// List component to hold multiple PersonListItems
pub struct PersonList {}

impl PersonList {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for PersonList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0x1a1a1a))
            .p_6()
            .gap_4()
            .child(
                div()
                    .text_2xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .mb_4()
                    .child("Person List"),
            )
            .child(cx.new(|_| PersonListItem::new("Mick", "Jagger")))
            .child(cx.new(|_| PersonListItem::new("Curt", "Cobain")))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(640.0), px(480.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                // Create PersonList component containing multiple PersonListItems
                cx.new(|_| PersonList::new())
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
