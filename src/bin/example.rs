use cacao2::{
    application::Application,
    view::{View, button::Button, text::Text},
    window::{Window, titlebar::TitlebarConfig},
};

pub struct State {
    counter: u32,
}

pub enum Message {
    ButtonClicked,
}

impl State {
    pub fn view(&self) -> View {
        let view = View::new();
        view.set_bg_color((0., 0., 0., 0.));
        view.add_subview(&Text::new(&format!("Count at: {}", self.counter)).into());
        view
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::ButtonClicked => self.counter += 1,
        }
    }
}

fn main() {
    let state = State { counter: 10 };
    let mut app = Application::new(state, Box::new(State::view), Box::new(State::update));
    let window = Window::new();
    window.set_bg_color((1., 0., 0., 1.));
    window.set_opaque(false);

    let mut title_conf = TitlebarConfig::new("Abcd".to_string());
    title_conf.set_visible(true);
    title_conf.set_transparent(true);

    window.set_titlebar_config(&title_conf);

    app.add_window(window);
    app.run();
}
