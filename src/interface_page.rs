use iced::{
    widget::{column, horizontal_rule, image, row, text, vertical_space, Column},
    Element, Length,
};

use crate::main_menu::{FreeEta, MainMenuMessage};

enum SpacePosition {
    Upper,
    Lower,
}

impl FreeEta {
    // Default interface page
    pub fn default_interface_page<'a>(&self) -> Element<'a, MainMenuMessage> {
        let page = self.default_pump_widget("static/png/Pump.png");
        page.into()
    }

    fn default_pump_widget<'a>(&self, pic_path: &str) -> Column<'a, MainMenuMessage> {
        column![
            vertical_space().height(Length::FillPortion(self.get_portion(SpacePosition::Upper))),
            row![
                horizontal_rule(1),
                image(pic_path),
                image("static/png/Switch.png"),
                text(format!("{:?}", self.window_size))
            ],
            vertical_space().height(Length::FillPortion(self.get_portion(SpacePosition::Lower))),
        ]
    }

    /// Get Length::FillPortion(`value`)
    fn get_portion(&self, position: SpacePosition) -> u16 {
        let blocks = 102.;
        let window_height = self.window_size.height;
        let center_y = window_height / 2.;
        let block_to_point = window_height / blocks;
        let gap = (self.mouse_point.y - center_y) / block_to_point;
        match position {
            SpacePosition::Upper => (blocks / 2. + gap) as u16,
            SpacePosition::Lower => (blocks / 2. - gap) as u16,
        }
    }
}
