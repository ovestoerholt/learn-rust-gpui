use gpui::{Context, Entity, Window, div, prelude::*, uniform_list};

use crate::{
    person_list_item::PersonListItem,
    theme::Theme,
};

// List component to hold multiple PersonListItems
pub struct PersonList {
    items: Vec<Entity<PersonListItem>>,
}

impl PersonList {
    pub fn new(items: Vec<Entity<PersonListItem>>) -> Self {
        Self { items }
    }
}


impl Render for PersonList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let items = self.items.clone();
        let theme = cx.global::<Theme>().clone();

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme.surface_primary)
            .p_6()
            .child(
                div()
                    .text_2xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(theme.text_primary)
                    .mb_4()
                    .child("Person List"),
            )
            .child(
                uniform_list("person-list", items.len(), move |range, _window, _cx| {
                    range
                        .map(|ix| div().w_full().child(items[ix].clone()))
                        .collect::<Vec<_>>()
                })
                .flex_1(),
            )
    }
}
