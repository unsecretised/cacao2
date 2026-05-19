use cacao2::{
    application::{ActivationPolicy, Application},
    view::{Alignment, Color, Radius, View, text::Text},
    window::{WindowLevel, titlebar::TitlebarConfig},
};

fn main() {
    let mut app = Application::new((), ActivationPolicy::Accessory);

    let window = app.new_window();

    window.set_level(WindowLevel::Above);
    app.add_window(&window);

    window.show();

    let monitor_dims = app.monitor_dimensions().unwrap();

    window.move_window(monitor_dims.0 / 2., monitor_dims.1 / 2.);

    let mut titlebar = TitlebarConfig::new("Someone".to_string());
    titlebar
        .set_visible(false)
        .set_transparent(true)
        .set_traffic_lights(true);

    let frame = window.frame();
    let rad = Radius::new(10.);

    let view = View::new();
    let view = view
        .set_size((500., 500.))
        .align_x(frame, Alignment::Center)
        .align_y(frame, Alignment::Center)
        .set_bg_color((1., 1., 1., 1.))
        .set_corner_radius(rad);

    let text_view: View = Text::new("Something")
        .set_font_size(50.)
        .set_font("Iosevka")
        .set_text_color(Color {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 1.,
        })
        .text_align(Alignment::Center)
        .set_selectable(true)
        .into();

    text_view.align_y(view.frame(), Alignment::Center);
    text_view.align_x(view.frame(), Alignment::Center);

    view.add_subview(&text_view);

    window.set_bg_color((0., 0., 0., 0.3));
    window.set_titlebar_config(&titlebar);

    window.view(&view);

    app.run();
}
