use gpui::{
    AppContext, Application, Font, FontFeatures, FontStyle, FontWeight, ParentElement, Render,
    SharedString, StrikethroughStyle, Styled, StyledText, TextRun, UnderlineStyle, WindowOptions,
    div, hsla, px, rgb,
};

struct RootView;

impl Render for RootView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .flex()
            .justify_center()
            .items_center()
            .text_3xl()
            .child(StyledText::new("Text").with_runs(vec![
                TextRun {
                    len: 2,
                    font: Font {
                        family: SharedString::new_static(".SystemUIFont"),
                        features: FontFeatures::default(),
                        fallbacks: None,
                        weight: FontWeight::default(),
                        style: FontStyle::default(),
                    },
                    color: hsla(0., 1., 0.5, 1.),
                    background_color: None,
                    underline: None,
                    strikethrough: Some(StrikethroughStyle {
                        thickness: px(1.),
                        color: Some(hsla(0., 1., 0.5, 1.)),
                    }),
                },
                TextRun {
                    len: 3,
                    font: Font {
                        family: SharedString::new_static(".SystemUIFont"),
                        features: FontFeatures::default(),
                        fallbacks: None,
                        weight: FontWeight::default(),
                        style: FontStyle::default(),
                    },
                    color: hsla(240. / 360., 1., 0.5, 1.),
                    background_color: None,
                    underline: Some(UnderlineStyle {
                        thickness: px(1.),
                        color: Some(hsla(240. / 360., 1., 0.5, 1.)),
                        wavy: true,
                    }),
                    strikethrough: None,
                },
            ]))
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |_window, app| {
            app.new(|_cx| RootView)
        })
        .unwrap();
    });
}
