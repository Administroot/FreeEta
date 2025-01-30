use iced::{
    alignment,
    widget::{column, container, row, text, vertical_space, Row},
    Color, Element, Font, Length,
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
    pub fn main_menu_page(&self) -> Element<MainMenuMessage> {
        container(text("Hello, FreeEta!").size(80))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    pub fn interface_page(&self) -> Element<MainMenuMessage> {
        container(text("Hello, interface page!").size(70))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    pub fn chart_page(&self) -> Element<MainMenuMessage> {
        if !self.eta.nodes.is_empty() {
            // TODO: Start ETA analysis!
            container(text("Hello, chart page!").size(65))
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .into()
        } else {
            // Display default ETA
            default_chart_page()
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

fn default_chart_page<'a>() -> Element<'a, MainMenuMessage> {
    let page = default_init_event();
    page.into()
}

fn default_init_event<'a>() -> Row<'a, MainMenuMessage> {
    let color_0 = freeeta_styles::get_a_color(0);
    let color_1 = freeeta_styles::get_a_color(1);

    let init_event = row![
        column![
            freeeta_buttons::eta_event_header(
                MainMenuMessage::DoNothing,
                Font::DEFAULT,
                "Initiating Event",
                color_0,
            )
            .width(Length::FillPortion(1)),
            vertical_space(),
            freeeta_rules::eta_horizontal_branch("Initiating Event(IE)")
                .width(Length::FillPortion(1)),
            vertical_space(),
        ]
        .width(Length::Fill),
        freeeta_rules::event_seperate_line(color_0),
    ]
    .height(Length::Fill);

    let event_1 = default_event_1(init_event, color_1);
    event_1
}

fn default_event_1<'a>(
    prev_event: Row<'a, MainMenuMessage>,
    color: Color,
) -> Row<'a, MainMenuMessage> {
    row![
        prev_event.width(Length::FillPortion(1)),
        column![
            freeeta_buttons::eta_event_header(
                MainMenuMessage::DoNothing,
                Font::default(),
                "Event 1",
                color
            ),
            column![
                column![vertical_space(), freeeta_rules::eta_horizontal_branch("Success(Valve 1)")].height(Length::FillPortion(1)).padding(0),
                freeeta_rules::eta_vertical_branch(),
                column![freeeta_rules::eta_horizontal_branch("Failure(Valve 1)")].height(Length::FillPortion(1)).padding(0),
            ],
        ]
        .width(Length::FillPortion(1)),
        freeeta_rules::event_seperate_line(color),
    ]
}
