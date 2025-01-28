use iced::{
    alignment,
    widget::{column, container, row, text, vertical_space, Container},
    Font, Length,
};

use crate::freeeta_buttons;
use crate::freeeta_rules;
use crate::{
    freeeta_styles,
    main_menu::{FreeEta, MainMenuMessage},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pages {
    #[default]
    MainMenuPage,
    InterfacePage,
    ChartPage,
    DeveloperPage,
    ExportPage,
}

impl FreeEta {
    pub fn main_menu_page(&self) -> Container<MainMenuMessage> {
        container(text("Hello, FreeEta!").size(80))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    pub fn interface_page(&self) -> Container<MainMenuMessage> {
        container(text("Hello, interface page!").size(70))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    pub fn chart_page(&self) -> Container<MainMenuMessage> {
        if !self.eta.nodes.is_empty() {
            // TODO: Start ETA analysis!
            container(text("Hello, chart page!").size(65))
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .into()
        } else {
            // Display default ETA
            self.default_chart_page()
        }
    }

    pub fn developer_page(&self) -> Container<MainMenuMessage> {
        container(text("Hello, developer page!").size(60))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    pub fn export_page(&self) -> Container<MainMenuMessage> {
        container(text("Hello, export page!").size(50))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    // Default
    fn default_chart_page(&self) -> Container<MainMenuMessage> {
        let color_0 = freeeta_styles::get_a_color(0);
        container(row![
            column![
                freeeta_buttons::eta_event_header(
                    MainMenuMessage::DoNothing,
                    Font::DEFAULT,
                    "Initiating Event",
                    color_0,
                ),
                vertical_space(),
                freeeta_rules::eta_branch("Initiating Event(IE)"),
                vertical_space(),
            ]
            .width(Length::Shrink),
            freeeta_rules::event_seperate_line(color_0),
            // freeeta_rules::event_seperate_line(Color::BLACK),
            column![
                freeeta_buttons::eta_event_header(
                    MainMenuMessage::DoNothing,
                    Font::DEFAULT,
                    "Event 1",
                    freeeta_styles::get_a_color(1)
                ),
                vertical_space(),
            ],
            freeeta_buttons::eta_event_header(
                MainMenuMessage::DoNothing,
                Font::DEFAULT,
                "Event 2",
                freeeta_styles::get_a_color(2)
            ),
            freeeta_buttons::eta_event_header(
                MainMenuMessage::DoNothing,
                Font::DEFAULT,
                "Event 3",
                freeeta_styles::get_a_color(3)
            ),
            freeeta_buttons::eta_event_header(
                MainMenuMessage::DoNothing,
                Font::DEFAULT,
                "Event 4",
                freeeta_styles::get_a_color(4)
            ),
            freeeta_buttons::eta_event_header(
                MainMenuMessage::DoNothing,
                Font::DEFAULT,
                "Outcome",
                freeeta_styles::get_a_color(5)
            ),
        ])
    }
}
