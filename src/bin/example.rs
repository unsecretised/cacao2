use cacao2::{
    application::{ActivationPolicy, Application},
    view::{Anchor, Radius, View, button::Button, text::Text},
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

    view.set_size((500., 500.));
    view.anchor_in_window(&window, Anchor::Center);

    view.set_corner_radius(rad);

    let text = Text::new("Something");

    text.set_font_size(30.);

    text.set_size(500., 500.);

    text.set_selectable(true);
    text.set_editable(true);

    let text_view: View = text.into();

    text_view.anchor_in_view(&view, Anchor::Center);
    view.add_subview(&text_view);

    window.set_bg_color((0.3, 0.1, 0.4, 0.7));
    window.set_titlebar_config(&titlebar);

    window.view(&view);

    let button = Button::new("Click me");
    button.set_size(100., 100.);
    button.set_pos(100., 100.);
    button.set_bg_color((0.3, 0.3, 0.5, 1.));

    let button_view: View = button.into();

    button_view.anchor_in_view(&view, Anchor::Center);
    view.add_subview(&button_view);

    app.run();
}
