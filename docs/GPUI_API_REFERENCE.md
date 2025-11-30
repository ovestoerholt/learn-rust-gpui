# GPUI API Reference

A quick reference guide for the GPUI framework API based on practical usage.

## Table of Contents
- [Core Concepts](#core-concepts)
- [Application Setup](#application-setup)
- [Components & Rendering](#components--rendering)
- [Styling (Tailwind-like)](#styling-tailwind-like)
- [Layout](#layout)
- [Interactive Elements](#interactive-elements)
- [Lists & Scrolling](#lists--scrolling)
- [State Management](#state-management)

---

## Core Concepts

| Concept | Description | Example |
|---------|-------------|---------|
| `Entity<T>` | A reactive component with its own state and context | `Entity<PersonListItem>` |
| `Context<Self>` | Component-specific context for state management | Used in `render()` and lifecycle methods |
| `Window` | Window context for rendering | Passed to `render()` method |
| `App` | Application-level context | Used in `main()` |
| `IntoElement` | Trait for types that can be rendered | Implemented by all renderable components |

---

## Application Setup

| API | Description | Example |
|-----|-------------|---------|
| `Application::new()` | Create a new application | `Application::new().run(\|cx\| { ... })` |
| `cx.open_window()` | Open a new window | `cx.open_window(WindowOptions { ... }, \|_, cx\| { ... })` |
| `Bounds::centered()` | Center window on screen | `Bounds::centered(None, size(px(640.0), px(480.0)), cx)` |
| `WindowBounds::Windowed()` | Create a windowed bounds | `WindowBounds::Windowed(bounds)` |
| `size()` | Create a size dimension | `size(px(640.0), px(480.0))` |
| `px()` | Convert float to pixel unit | `px(16.0)` |

---

## Components & Rendering

| API | Description | Example |
|-----|-------------|---------|
| `impl Render` | Implement rendering for a component | `impl Render for MyComponent { ... }` |
| `fn render()` | Define how component renders | `fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>)` |
| `cx.new()` | Create a new component entity | `cx.new(\|_\| PersonListItem::new("John", "Doe"))` |
| `cx.notify()` | Notify GPUI of state changes (triggers re-render) | `cx.notify()` after state update |
| `SharedString` | Efficient shared string type | `first_name: SharedString` |
| `.into()` | Convert to SharedString | `"Hello".into()` |

---

## Styling (Tailwind-like)

### Colors

| API | Description | Example |
|-----|-------------|---------|
| `rgb()` | Create RGB color | `rgb(0xff0000)` for red |
| `rgb(0xRRGGBB)` | Hex color notation | `rgb(0x2a2a2a)` |
| `gpui::red()` | Predefined color | `gpui::red()`, `gpui::blue()`, etc. |
| `.bg()` | Background color | `.bg(rgb(0x2a2a2a))` |
| `.text_color()` | Text color | `.text_color(rgb(0xffffff))` |
| `.border_color()` | Border color | `.border_color(rgb(0x404040))` |

### Spacing

| API | Description | Example |
|-----|-------------|---------|
| `.p_1()` to `.p_8()` | Padding (all sides) | `.p_4()` |
| `.px_3()` | Padding horizontal | `.px_3()` |
| `.py_2()` | Padding vertical | `.py_2()` |
| `.pt_4()` | Padding top | `.pt_4()` |
| `.pb_4()` | Padding bottom | `.pb_4()` |
| `.pl_4()` | Padding left | `.pl_4()` |
| `.pr_4()` | Padding right | `.pr_4()` |
| `.m_1()` to `.m_8()` | Margin (all sides) | `.m_2()` |
| `.mx_4()` | Margin horizontal | `.mx_4()` |
| `.my_4()` | Margin vertical | `.my_4()` |
| `.mt_4()` | Margin top | `.mt_4()` |
| `.mb_4()` | Margin bottom | `.mb_4()` |
| `.gap_1()` to `.gap_8()` | Gap between flex items | `.gap_4()` |

### Sizing

| API | Description | Example |
|-----|-------------|---------|
| `.w_full()` | Width 100% | `.w_full()` |
| `.h_full()` | Height 100% | `.h_full()` |
| `.size_8()` | Width & height (8 units) | `.size_8()` |
| `.size_12()` | Width & height (12 units) | `.size_12()` |
| `.w()` | Custom width | `.w(px(200.0))` |
| `.h()` | Custom height | `.h(px(100.0))` |

### Borders & Corners

| API | Description | Example |
|-----|-------------|---------|
| `.border_1()` | 1px border | `.border_1()` |
| `.rounded_lg()` | Large border radius | `.rounded_lg()` |
| `.rounded_md()` | Medium border radius | `.rounded_md()` |
| `.rounded_full()` | Fully rounded (circle) | `.rounded_full()` |

### Typography

| API | Description | Example |
|-----|-------------|---------|
| `.text_sm()` | Small text | `.text_sm()` |
| `.text_lg()` | Large text | `.text_lg()` |
| `.text_xl()` | Extra large text | `.text_xl()` |
| `.text_2xl()` | 2x extra large text | `.text_2xl()` |
| `.font_weight()` | Font weight | `.font_weight(gpui::FontWeight::BOLD)` |
| `FontWeight::BOLD` | Bold font | `.font_weight(gpui::FontWeight::BOLD)` |
| `FontWeight::SEMIBOLD` | Semi-bold font | `.font_weight(gpui::FontWeight::SEMIBOLD)` |

---

## Layout

| API | Description | Example |
|-----|-------------|---------|
| `.flex()` | Enable flexbox layout | `.flex()` |
| `.flex_row()` | Flex direction row (default) | `.flex_row()` |
| `.flex_col()` | Flex direction column | `.flex_col()` |
| `.flex_1()` | Flex grow (take remaining space) | `.flex_1()` |
| `.items_center()` | Align items center (cross-axis) | `.items_center()` |
| `.justify_center()` | Justify content center (main-axis) | `.justify_center()` |

---

## Interactive Elements

| API | Description | Example |
|-----|-------------|---------|
| `.id()` | Set element ID (required for interactions) | `.id("my-button")` |
| `.on_click()` | Handle click events | `.on_click(cx.listener(\|this, _, _, cx\| { ... }))` |
| `cx.listener()` | Create event listener | `cx.listener(\|this, event, window, cx\| { ... })` |
| `.cursor_pointer()` | Show pointer cursor on hover | `.cursor_pointer()` |
| `.hover()` | Apply styles on hover | `.hover(\|style\| style.bg(rgb(0x333333)))` |

---

## Lists & Scrolling

| API | Description | Example |
|-----|-------------|---------|
| `uniform_list()` | Scrollable virtualized list | `uniform_list("list-id", count, render_fn)` |
| ID parameter | Unique identifier for list | `"person-list"` |
| Count parameter | Total number of items | `items.len()` |
| Render function | Closure that renders visible items | `\|range, _window, _cx\| { ... }` |
| `.flex_1()` | Make list fill available space | Applied to `uniform_list()` |

**uniform_list Example:**
```rust
uniform_list("my-list", items.len(), move |range, _window, _cx| {
    range
        .map(|ix| div().child(items[ix].clone()))
        .collect::<Vec<_>>()
})
.flex_1()
```

---

## State Management

| Pattern | Description | Example |
|---------|-------------|---------|
| Component state | Store state in struct fields | `likes: u32` |
| Mutate state | Use `&mut self` in methods | `fn increment(&mut self, cx: &mut Context<Self>)` |
| Notify changes | Call `cx.notify()` after mutations | `self.likes += 1; cx.notify();` |
| Entity storage | Store components as entities | `Vec<Entity<PersonListItem>>` |
| Clone entities | Entities are cheap to clone | `items[ix].clone()` |

**State Update Pattern:**
```rust
fn increment_likes(&mut self, cx: &mut Context<Self>) {
    self.likes += 1;  // Mutate state
    cx.notify();      // Trigger re-render
}
```

---

## Building Elements

### Basic div

```rust
div()
    .flex()
    .p_4()
    .bg(rgb(0x2a2a2a))
    .child("Hello, World!")
```

### Button with click handler

```rust
div()
    .id("my-button")
    .px_3()
    .py_2()
    .bg(rgb(0x3399ff))
    .rounded_lg()
    .cursor_pointer()
    .hover(|style| style.bg(rgb(0x4da6ff)))
    .on_click(cx.listener(|this, _, _, cx| {
        this.handle_click(cx);
    }))
    .child("Click me")
```

### Conditional children

```rust
div()
    .child("Always shown")
    .when(condition, |div| {
        div.child("Only shown when condition is true")
    })
```

---

## Common Patterns

### Creating a Component

```rust
pub struct MyComponent {
    data: SharedString,
}

impl MyComponent {
    pub fn new(data: impl Into<SharedString>) -> Self {
        Self {
            data: data.into(),
        }
    }
}

impl Render for MyComponent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(self.data.clone())
    }
}
```

### Using a Component

```rust
// In another component's render or in window creation:
cx.new(|_| MyComponent::new("Hello"))
```

### Parent-Child Relationship

```rust
// Parent stores child entities
pub struct Parent {
    children: Vec<Entity<Child>>,
}

// Render children
impl Render for Parent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .children(self.children.iter().map(|child| child.clone()))
    }
}
```

---

## Tips & Best Practices

1. **Always call `cx.notify()` after state changes** to trigger re-renders
2. **Use `Entity<T>` for components that need independent state**
3. **Interactive elements require `.id()`** before using `.on_click()`
4. **Clone entities freely** - they're lightweight references
5. **Use `uniform_list` for long lists** - it virtualizes rendering for performance
6. **Separate components into their own files** for better organization
7. **Use `SharedString` for strings** that might be shared across components
8. **Import traits** like `AppContext` and `Render` to use their methods

---

## Module Organization

```
src/
├── main.rs              # Application entry point
├── component1.rs        # Component 1 definition
└── component2.rs        # Component 2 definition

// main.rs
mod component1;
mod component2;
use component1::Component1;
use component2::Component2;
```

---

## Common Imports

```rust
// For components
use gpui::{
    Context, Entity, Window, div, prelude::*, rgb, uniform_list
};

// For main application
use gpui::{
    App, AppContext, Application, Bounds, WindowBounds, 
    WindowOptions, px, size
};
```

---

## Resources

- **GPUI GitHub**: https://github.com/zed-industries/zed/tree/main/crates/gpui
- **Zed Editor Source**: Great examples of GPUI usage
- **This is based on**: GPUI v0.2.2

---

*Last Updated: 2024*
*This reference is based on practical usage and may not be exhaustive.*