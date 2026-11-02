use cacao2::{
    application::{ActivationPolicy, Application},
    view::{Alignment, Color, Radius, View, button::Button, text::Text},
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
        .set_corner_radius(rad);

    let btn_view: View = Button::new("Hello", || println!("Hello world")).into();
    btn_view.set_pos((0., 0.));
    view.add_subview(&btn_view);

    window.set_bg_color((0., 0., 0., 0.3));
    window.set_titlebar_config(&titlebar);

    window.view(&view);

    app.run();
}
