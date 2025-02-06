use iced::{
    alignment,
    widget::{column, container, row, text, vertical_space, Row},
    Color, Element, Font, Length,
};

use crate::freeeta_buttons::eta_event_header;
use crate::freeeta_rules::{
    eta_horizontal_branch, eta_output_branch, eta_vertical_branch, event_seperate_line,
};
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
        // TODO: Reverse it when all function is completed
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

// Default ETA tree
fn default_chart_page<'a>() -> Element<'a, MainMenuMessage> {
    let page = default_init_event();
    page.into()
}

fn default_init_event<'a>() -> Row<'a, MainMenuMessage> {
    let color_0 = freeeta_styles::get_a_color(0);
    let color_1 = freeeta_styles::get_a_color(1);
    let color_2 = freeeta_styles::get_a_color(2);
    let color_3 = freeeta_styles::get_a_color(3);
    let color_4 = freeeta_styles::get_a_color(4);
    let color_5 = freeeta_styles::get_a_color(5);

    let init_event = row![
        column![
            eta_event_header(
                MainMenuMessage::DoNothing,
                Font::DEFAULT,
                "Initiating Event",
                color_0,
            )
            .width(Length::FillPortion(1)),
            column![
                vertical_space().height(Length::FillPortion(15)),
                eta_horizontal_branch("Initiating Event(IE)", true),
                vertical_space().height(Length::FillPortion(4)),
            ]
            .width(Length::FillPortion(1))
        ],
        event_seperate_line(color_0),
    ];

    let event_1 = default_event_1(init_event, color_1);
    let event_2 = default_event_2(event_1, color_2);
    let event_3 = default_event_3(event_2, color_3);
    let event_4 = default_event_4(event_3, color_4);
    let event_5 = default_outcome(event_4, color_5);
    event_5
}

fn default_event_1<'a>(
    prev_event: Row<'a, MainMenuMessage>,
    color: Color,
) -> Row<'a, MainMenuMessage> {
    row![
        prev_event.width(Length::FillPortion(1)),
        column![
            eta_event_header(
                MainMenuMessage::DoNothing,
                Font::default(),
                "Event 1",
                color
            ),
            column![
                vertical_space().height(Length::FillPortion(1)),
                eta_horizontal_branch("Success(Valve 1)", true),
                eta_vertical_branch(1),
                eta_horizontal_branch("Failure(Valve 1)", false),
            ],
        ]
        .width(Length::FillPortion(1)),
        event_seperate_line(color),
    ]
}

fn default_event_2<'a>(
    prev_event: Row<'a, MainMenuMessage>,
    color: Color,
) -> Row<'a, MainMenuMessage> {
    row![
        prev_event.width(Length::FillPortion(2)),
        column![
            eta_event_header(
                MainMenuMessage::DoNothing,
                Font::default(),
                "Event 2",
                color
            ),
            column![
                vertical_space().height(Length::FillPortion(2)),
                eta_horizontal_branch("Success(Valve 2)", true),
                eta_vertical_branch(5),
                eta_horizontal_branch("Failure(Valve 2)", false),
                vertical_space().height(Length::FillPortion(2)),
                eta_horizontal_branch("Failure(Valve 1)", true),
            ],
        ]
        .width(Length::FillPortion(1)),
        event_seperate_line(color),
    ]
}

fn default_event_3<'a>(
    prev_event: Row<'a, MainMenuMessage>,
    color: Color,
) -> Row<'a, MainMenuMessage> {
    row![
        prev_event.width(Length::FillPortion(3)),
        column![
            eta_event_header(
                MainMenuMessage::DoNothing,
                Font::default(),
                "Event 3",
                color
            ),
            column![
                vertical_space().height(Length::FillPortion(1)),
                eta_horizontal_branch("Success(Valve 3)", true),
                eta_vertical_branch(3),
                eta_horizontal_branch("Failure(Valve 3)", false),
                vertical_space().height(Length::FillPortion(3)),
                eta_horizontal_branch("Success(Valve 3)", true),
                eta_vertical_branch(3),
                eta_horizontal_branch("Failure(Valve 3)", false),
                vertical_space().height(Length::FillPortion(1)),
                eta_horizontal_branch("Failure(Valve 1)", true)
            ],
        ]
        .width(Length::FillPortion(1)),
        event_seperate_line(color),
    ]
}

fn default_event_4<'a>(
    prev_event: Row<'a, MainMenuMessage>,
    color: Color,
) -> Row<'a, MainMenuMessage> {
    row![
        prev_event.width(Length::FillPortion(4)),
        column![
            eta_event_header(
                MainMenuMessage::DoNothing,
                Font::default(),
                "Event 4",
                color
            ),
            eta_horizontal_branch("Success(Valve 4)", true),
            eta_vertical_branch(1),
            eta_horizontal_branch("Failure(Valve 4)", false),
            vertical_space().height(Length::FillPortion(1)),
            eta_horizontal_branch("Success(Valve 4)", true),
            eta_vertical_branch(1),
            eta_horizontal_branch("Failure(Valve 4)", false),
            vertical_space().height(Length::FillPortion(1)),
            eta_horizontal_branch("Success(Valve 4)", true),
            eta_vertical_branch(1),
            eta_horizontal_branch("Failure(Valve 4)", false),
            vertical_space().height(Length::FillPortion(2)),
            eta_horizontal_branch("Failure(Valve 3)", true),
            vertical_space().height(Length::FillPortion(1)),
            eta_horizontal_branch("Failure(Valve 1)", true),
        ]
        .width(Length::FillPortion(1)),
    ]
}

fn default_outcome<'a>(
    prev_event: Row<'a, MainMenuMessage>,
    color: Color,
) -> Row<'a, MainMenuMessage> {
    row![
        prev_event
            .width(Length::FillPortion(4))
            .height(Length::FillPortion(1)),
        column![
            eta_event_header(MainMenuMessage::DoNothing, Font::default(), "Output", color),
            eta_output_branch("Success Outcome A"),
            vertical_space().height(Length::FillPortion(1)),
            eta_output_branch("Failure Outcome B"),
            vertical_space().height(Length::FillPortion(1)),
            eta_output_branch("Success Outcome C"),
            vertical_space().height(Length::FillPortion(1)),
            eta_output_branch("Failure Outcome D"),
            vertical_space().height(Length::FillPortion(1)),
            eta_output_branch("Success Outcome E"),
            vertical_space().height(Length::FillPortion(1)),
            eta_output_branch("Failure Outcome F"),
            vertical_space().height(Length::FillPortion(2)),
            eta_output_branch("Failure Outcome G"),
            vertical_space().height(Length::FillPortion(1)),
            eta_output_branch("Failure Outcome H")
        ]
        .width(Length::FillPortion(1)),
    ]
}

// Default interface page
fn default_interface_page<'a>() -> Element<'a, MainMenuMessage> {
    let page = default_init_event();
    page.into()
}
