pub struct TitlebarConfig {
    pub(crate) title: String,
    pub(crate) appears_transparent: bool,
    pub(crate) show_titlebar: bool,
    pub(crate) show_traffic_lights: bool,
}

impl TitlebarConfig {
    pub fn new(title: String) -> Self {
        Self {
            title,
            appears_transparent: false,
            show_titlebar: true,
            show_traffic_lights: false,
        }
    }

    pub fn set_transparent(&mut self, transparent: bool) -> &mut Self {
        self.appears_transparent = transparent;
        self
    }

    pub fn set_traffic_lights(&mut self, visible: bool) -> &mut Self {
        self.show_traffic_lights = visible;
        self
    }

    pub fn set_visible(&mut self, visible: bool) -> &mut Self {
        self.show_titlebar = visible;
        self
    }
}
