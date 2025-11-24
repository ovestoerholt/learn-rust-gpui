use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};

struct ColorMixer {
    red: f32,
    green: f32,
    blue: f32,
}

impl ColorMixer {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
        }
    }

    fn set_red(&mut self, value: f32, cx: &mut Context<Self>) {
        self.red = value;
        cx.notify();
    }

    fn set_green(&mut self, value: f32, cx: &mut Context<Self>) {
        self.green = value;
        cx.notify();
    }

    fn set_blue(&mut self, value: f32, cx: &mut Context<Self>) {
        self.blue = value;
        cx.notify();
    }

    fn render_channel_control(
        &self,
        label: &'static str,
        value: f32,
        setter: impl Fn(&mut Self, f32, &mut Context<Self>) + 'static + Copy,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(label)
            .child(div().flex().gap_1().children((0..20).map(|i| {
                let v = (i as f32) / 19.0;
                let active = v <= value;
                div()
                    .id((label, i as usize))
                    .w_4()
                    .h_6()
                    .bg(if active { rgb(0xffffff) } else { rgb(0x505050) })
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        setter(this, v, cx);
                    }))
            })))
    }
}

impl Render for ColorMixer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let color = rgb(((self.red * 255.0) as u32) << 16
            | ((self.green * 255.0) as u32) << 8
            | ((self.blue * 255.0) as u32));

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x202020))
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .flex()
                    .size_full()
                    .bg(color)
                    .justify_center()
                    .items_center()
                    .child(format!(
                        "R: {:.2} G: {:.2} B: {:.2}",
                        self.red, self.green, self.blue
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_4()
                    .bg(rgb(0x303030))
                    .child(self.render_channel_control(
                        "Red",
                        self.red,
                        |this, val, cx| this.set_red(val, cx),
                        cx,
                    ))
                    .child(self.render_channel_control(
                        "Green",
                        self.green,
                        |this, val, cx| this.set_green(val, cx),
                        cx,
                    ))
                    .child(self.render_channel_control(
                        "Blue",
                        self.blue,
                        |this, val, cx| this.set_blue(val, cx),
                        cx,
                    )),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| ColorMixer::new(cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}
