# GPUI Themes and Key Bindings

This document covers theming and key binding systems in GPUI 0.2.2.

## Themes

### Theme System Overview

GPUI uses a flexible theming system that allows you to define colors, typography, and other visual properties for your application.

### Color Definition

Colors in GPUI are defined using hexadecimal values with the `rgb()` function:

```rust
use gpui::rgb;

// Define colors
let background = rgb(0x1a1a1a);  // Dark gray
let foreground = rgb(0xe5e5e5);  // Light gray
let accent = rgb(0x3399ff);      // Blue
```

### Common Color Patterns

```rust
// Background colors
const BACKGROUND_DARK: u32 = 0x0f0f0f;
const BACKGROUND_MEDIUM: u32 = 0x1a1a1a;
const BACKGROUND_LIGHT: u32 = 0x2a2a2a;

// Text colors
const TEXT_PRIMARY: u32 = 0xffffff;
const TEXT_SECONDARY: u32 = 0xe5e5e5;
const TEXT_MUTED: u32 = 0x999999;

// Border colors
const BORDER_DEFAULT: u32 = 0x404040;
const BORDER_LIGHT: u32 = 0x4d4d4d;

// Interactive colors
const BUTTON_BG: u32 = 0x3399ff;
const BUTTON_HOVER: u32 = 0x4da6ff;
const BUTTON_ACTIVE: u32 = 0x2588ee;
```

### Applying Colors to Elements

```rust
div()
    .bg(rgb(0x1a1a1a))                    // Background
    .text_color(rgb(0xffffff))            // Text color
    .border_color(rgb(0x404040))          // Border color
    .hover(|style| style.bg(rgb(0x2a2a2a))) // Hover state
```

### Named Color Helpers

GPUI provides built-in color helpers:

```rust
use gpui::{red, green, blue, yellow, black, white};

div()
    .bg(red())      // Predefined red
    .bg(green())    // Predefined green
    .bg(blue())     // Predefined blue
    .bg(yellow())   // Predefined yellow
    .bg(black())    // Predefined black
    .bg(white())    // Predefined white
```

### Typography

```rust
div()
    .text_sm()                           // Small text
    .text_lg()                           // Large text
    .text_xl()                           // Extra large text
    .text_2xl()                          // 2x extra large text
    .text_3xl()                          // 3x extra large text
    .font_weight(gpui::FontWeight::BOLD)       // Bold
    .font_weight(gpui::FontWeight::SEMIBOLD)   // Semi-bold
    .font_weight(gpui::FontWeight::NORMAL)     // Normal
```

## Key Bindings

### Event Handling

GPUI uses event listeners for handling user interactions.

### Mouse Events

#### Click Events

For click events to work, the element must have an ID:

```rust
div()
    .id("my-button")  // ID is required for click events
    .on_click(cx.listener(|this, _event, _window, cx| {
        // Handle click
        this.do_something(cx);
    }))
```

**Important**: The `on_click` method requires:
1. An element ID (`.id()`)
2. A listener created with `cx.listener()`

#### Mouse Down Events

```rust
use gpui::MouseButton;

div()
    .id("clickable")
    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
        // Handle left mouse button down
    }))
    .on_mouse_down(MouseButton::Right, cx.listener(|this, _event, _window, cx| {
        // Handle right mouse button down
    }))
```

#### Hover Events

```rust
div()
    .hover(|style| {
        style
            .bg(rgb(0x2a2a2a))           // Change background on hover
            .text_color(rgb(0xffffff))   // Change text color
    })
```

### Listener Pattern

The `cx.listener()` method creates a closure that provides access to:
- `this`: Mutable reference to your component
- `_event`: The event data (often unused)
- `_window`: The window reference (often unused) 
- `cx`: The context for triggering updates

```rust
cx.listener(|this, _event, _window, cx| {
    // Modify component state
    this.count += 1;
    
    // Notify GPUI to re-render
    cx.notify();
})
```

### State Updates

After modifying state, always call `cx.notify()` to trigger a re-render:

```rust
fn increment(&mut self, cx: &mut Context<Self>) {
    self.value += 1;
    cx.notify();  // Important: triggers re-render
}
```

### Cursor Styling

```rust
div()
    .cursor_pointer()  // Shows pointer cursor on hover
```

## Complete Example

```rust
use gpui::{div, rgb, prelude::*, Context, MouseButton};

struct Button {
    label: String,
    count: u32,
}

impl Button {
    fn handle_click(&mut self, cx: &mut Context<Self>) {
        self.count += 1;
        cx.notify();
    }
}

impl Render for Button {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("my-button")
            .flex()
            .px_4()
            .py_2()
            .bg(rgb(0x3399ff))
            .text_color(rgb(0xffffff))
            .rounded_lg()
            .cursor_pointer()
            .hover(|style| style.bg(rgb(0x4da6ff)))
            .on_click(cx.listener(|this, _, _, cx| {
                this.handle_click(cx);
            }))
            .child(format!("{}: {}", self.label, self.count))
    }
}
```

## Best Practices

### Theming

1. **Use constants for colors** to maintain consistency:
   ```rust
   const THEME_BG: u32 = 0x1a1a1a;
   const THEME_TEXT: u32 = 0xffffff;
   ```

2. **Create color palettes** for your application
3. **Use semantic naming** (e.g., `PRIMARY`, `SECONDARY`, `DANGER`)

### Key Bindings

1. **Always add IDs** to clickable elements
2. **Call `cx.notify()`** after state changes
3. **Use descriptive listener methods** for clarity
4. **Handle events at the appropriate component level**

## Common Patterns

### Interactive List Item

```rust
div()
    .id(("list-item", index))
    .flex()
    .p_4()
    .bg(rgb(0x2a2a2a))
    .hover(|style| style.bg(rgb(0x333333)))
    .cursor_pointer()
    .on_click(cx.listener(move |this, _, _, cx| {
        this.select_item(index, cx);
    }))
    .child(content)
```

### Toggle Button

```rust
div()
    .id("toggle")
    .bg(if self.active { rgb(0x3399ff) } else { rgb(0x666666) })
    .hover(|style| {
        let color = if self.active { 0x4da6ff } else { 0x777777 };
        style.bg(rgb(color))
    })
    .on_click(cx.listener(|this, _, _, cx| {
        this.active = !this.active;
        cx.notify();
    }))
    .child(label)
```

## Troubleshooting

### Click Events Not Working

**Problem**: `on_click` doesn't fire
**Solution**: Ensure the element has an ID:
```rust
div()
    .id("must-have-id")  // Required!
    .on_click(...)
```

### State Not Updating

**Problem**: UI doesn't update after changing state
**Solution**: Always call `cx.notify()`:
```rust
fn update(&mut self, cx: &mut Context<Self>) {
    self.value = new_value;
    cx.notify();  // Don't forget this!
}
```

### Hover Effects Not Showing

**Problem**: Hover styles don't apply
**Solution**: Use the `.hover()` method with a closure:
```rust
.hover(|style| style.bg(rgb(0x333333)))
```

## Additional Resources

- See `likes-counter/src/person_list_item.rs` for a complete working example
- GPUI API Reference: `docs/GPUI_API_REFERENCE.md`
