use cacao2::{
    application::{ActivationPolicy, Application},
    view::{Radius, View},
    window::{Window, titlebar::TitlebarConfig},
};

fn main() {
    let window = Window::new();
    let mut app = Application::new((), ActivationPolicy::Regular);

    app.add_window(&window);

    window.show();

    let mut titlebar = TitlebarConfig::new("Someone".to_string());
    titlebar
        .set_visible(false)
        .set_transparent(true)
        .set_traffic_lights(false);

    let view = View::new();

    view.set_bg_color((0.3, 0.3, 0.5, 1.));

    let rad = Radius::new(10.);

    view.set_position((100., 200.));

    view.set_corner_radius(rad);

    window.set_bg_color((0.3, 0.1, 0.4, 0.7));
    window.set_titlebar_config(&titlebar);

    window.view(&view);

    app.run();
}
