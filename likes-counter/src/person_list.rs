use gpui::{Context, Entity, Window, div, prelude::*, rgb, uniform_list};

use crate::person_list_item::PersonListItem;

// List component to hold multiple PersonListItems
pub struct PersonList {
    items: Vec<Entity<PersonListItem>>,
}

impl PersonList {
    pub fn new(items: Vec<Entity<PersonListItem>>) -> Self {
        Self { items }
    }

    // Helper method to create a list with default items
    pub fn with_items(cx: &mut Context<Self>, items: Vec<(&'static str, &'static str)>) -> Self {
        let item_entities = items
            .into_iter()
            .map(|(first, last)| cx.new(|_| PersonListItem::new(first, last)))
            .collect();
        Self::new(item_entities)
    }
}

impl Render for PersonList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let items = self.items.clone();

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0x1a1a1a))
            .p_6()
            .child(
                div()
                    .text_2xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
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
