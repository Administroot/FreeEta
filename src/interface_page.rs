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
        let blocks = 102;
        match position {
            SpacePosition::Upper => blocks / 2,
            SpacePosition::Lower => blocks / 2,
        }
    }
}
