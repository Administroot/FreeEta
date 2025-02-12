use iced::{
    alignment,
    widget::{container, text},
    Element,
};

use crate::main_menu::{FreeEta, MainMenuMessage};

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
    pub fn main_menu_page(&self) -> Element<MainMenuMessage> {
        container(text("Hello, FreeEta!").size(80))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    pub fn interface_page(&self) -> Element<MainMenuMessage> {
        if self.eta.nodes.is_empty() {
            container(text("Hello, interface page!").size(70))
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .into()
        } else {
            // Display default interface page
            self.default_interface_page()
        }
    }

    pub fn developer_page(&self) -> Element<MainMenuMessage> {
        container(text("Hello, developer page!").size(60))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    pub fn export_page(&self) -> Element<MainMenuMessage> {
        container(text("Hello, export page!").size(50))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }
}
