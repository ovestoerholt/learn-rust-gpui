use gpui::{actions, App, Application, Bounds, WindowBounds, WindowOptions, prelude::*, px, size};

mod person_list;
mod person_list_item;

mod theme;

use person_list::PersonList;
use person_list_item::PersonListItem;

use crate::theme::Theme;

// Define a quit action
actions!(app, [Quit]);

fn main() {
    Application::new().run(|cx: &mut App| {
        // Set theme globally so all components can access it
        cx.set_global(Theme::dark());
        
        // Bind Cmd+Q to quit action
        cx.bind_keys([gpui::KeyBinding::new("cmd-q", Quit, None)]);
        
        // Handle the quit action
        cx.on_action(|_: &Quit, cx| {
            cx.quit();
        });
        
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
