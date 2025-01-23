use iced::{alignment, widget::{container, text, Container}};

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
    pub fn main_menu_page(&self) -> Container<MainMenuMessage>{
        container(
            text("Hello, FreeEta!").size(80),
        ).align_x(alignment::Horizontal::Center).align_y(alignment::Vertical::Center).into()
    }

    pub fn interface_page(&self) -> Container<MainMenuMessage>{
        container(
            text("Hello, interface page!").size(70),
        ).align_x(alignment::Horizontal::Center).align_y(alignment::Vertical::Center).into()
    }

    pub fn chart_page(&self) -> Container<MainMenuMessage>{
        container(
            text("Hello, chart page!").size(65),
        ).align_x(alignment::Horizontal::Center).align_y(alignment::Vertical::Center).into()
    }

    pub fn developer_page(&self) -> Container<MainMenuMessage>{
        container(
            text("Hello, developer page!").size(60),
        ).align_x(alignment::Horizontal::Center).align_y(alignment::Vertical::Center).into()
    }

    pub fn export_page(&self) -> Container<MainMenuMessage>{
        container(
            text("Hello, export page!").size(50),
        ).align_x(alignment::Horizontal::Center).align_y(alignment::Vertical::Center).into()
    }
}