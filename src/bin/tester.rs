use cacao2::{application::Application, window::Window};

fn main() {
    let window = Window::new((100., 100.));
    let mut app = Application::new(());

    app.add_window(&window);

    window.show();

    app.run();
}
