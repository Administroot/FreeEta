use iced::{
    widget::{column, horizontal_rule, image, row, vertical_space, Column},
    Element, Length,
};

use crate::main_menu::{FreeEta, MainMenuMessage};

impl FreeEta {
    // Default interface page
    pub fn default_interface_page<'a>(&self) -> Element<'a, MainMenuMessage> {
        let page = self.default_pump_widget("static/png/Pump.png");
        page.into()
    }

    fn default_pump_widget<'a>(&self, pic_path: &str) -> Column<'a, MainMenuMessage> {
        column![
            vertical_space().height(Length::FillPortion(20)),
            row![
                horizontal_rule(1),
                image(pic_path),
                image("static/png/Switch.png")
            ],
            vertical_space().height(Length::FillPortion(10)),
        ]
    }
}
