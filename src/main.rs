use gpui::{
    Application, Context, Path, Pixels, Render, Window, WindowOptions, canvas, div, point,
    prelude::*, px,
};
struct PaintingViewer {
    length: Pixels,
}

impl PaintingViewer {
    fn new(window: &mut Window, _cx: &mut Context<Self>) -> Self {
        println!("gpu_specs: {:?}", window.gpu_specs());
        Self {
            length: px(1024.), // 1025 breaks it
        }
    }
}
impl Render for PaintingViewer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let length = self.length;

        div()
            .font_family(".SystemUIFont")
            .bg(gpui::white())
            .size_full()
            .p_4()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .gap_2()
                    .justify_between()
                    .items_center()
                    .child(format!("{}", self.length))
                    .child(
                        div()
                            .id("increase")
                            .child("Increase")
                            .bg(gpui::black())
                            .text_color(gpui::white())
                            .active(|this| this.opacity(0.8))
                            .flex()
                            .px_3()
                            .py_1()
                            .on_click(cx.listener(|this, _, _, _cx| {
                                this.length.0 += 1.0;
                            })),
                    )
                    .child(
                        div()
                            .id("decrease")
                            .child("Decrease")
                            .bg(gpui::black())
                            .text_color(gpui::white())
                            .active(|this| this.opacity(0.8))
                            .flex()
                            .px_3()
                            .py_1()
                            .on_click(cx.listener(|this, _, _, _cx| {
                                this.length.0 -= 1.0;
                            })),
                    ),
            )
            .child(
                div().size_full().child(
                    canvas(
                        move |_, _, _| {},
                        move |_, _, window, _| {
                            // it doesn't have to be a single triangle it also happens
                            // when we split it up into multiple triangles
                            // ... it's about the boundary size of the triangle list
                            let x = point(px(0.0), px(0.0));
                            let y = point(px(0.0), px(20.0));
                            let z = point(length, px(0.0));

                            let mut path = Path::new(x);
                            path.push_triangle(
                                (x.into(), y.into(), z.into()),
                                (point(0., 1.), point(0., 1.), point(0., 1.)),
                            );
                            window.paint_path(path, gpui::red());
                        },
                    )
                    .size_full(),
                ),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            |window, cx| cx.new(|cx| PaintingViewer::new(window, cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}
