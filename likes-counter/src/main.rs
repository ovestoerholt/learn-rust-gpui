use gpui::{App, AppContext, Application, Bounds, WindowBounds, WindowOptions, px, size};

mod person_list;
mod person_list_item;

use person_list::PersonList;
use person_list_item::PersonListItem;

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
                cx.new(|cx| {
                    let items = vec![
                        cx.new(|_| PersonListItem::new("Mick", "Jagger")),
                        cx.new(|_| PersonListItem::new("Curt", "Cobain")),
                        cx.new(|_| PersonListItem::new("Paul", "McCartney")),
                        cx.new(|_| PersonListItem::new("John", "Lennon")),
                        cx.new(|_| PersonListItem::new("George", "Harrison")),
                        cx.new(|_| PersonListItem::new("Ringo", "Starr")),
                        cx.new(|_| PersonListItem::new("David", "Bowie")),
                        cx.new(|_| PersonListItem::new("Freddie", "Mercury")),
                        cx.new(|_| PersonListItem::new("Elvis", "Presley")),
                        cx.new(|_| PersonListItem::new("Bob", "Dylan")),
                        cx.new(|_| PersonListItem::new("Jimi", "Hendrix")),
                        cx.new(|_| PersonListItem::new("Janis", "Joplin")),
                        cx.new(|_| PersonListItem::new("Jim", "Morrison")),
                        cx.new(|_| PersonListItem::new("Amy", "Winehouse")),
                        cx.new(|_| PersonListItem::new("Whitney", "Houston")),
                    ];
                    PersonList::new(items)
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
