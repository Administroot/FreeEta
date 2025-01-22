use iced::widget::{svg, text};
use iced::{
    alignment,
    event::{self, Status},
    mouse::Event::CursorMoved,
    touch::Event::FingerMoved,
    widget::{
        column, container, horizontal_rule, horizontal_space, row, svg::Handle, vertical_space,
    },
    Color, ContentFit, Element, Event, Font, Point, Subscription, Task,
};

use crate::{freeeta_buttons, freeeta_picklists::functional_picklist, freeeta_styles};

pub struct FreeEta {
    // TODO: Actually, I don't need this member.
    file_picklist: Option<String>,
    mouse_point: Point,
}

impl Default for FreeEta {
    fn default() -> Self {
        FreeEta::new().0
    }
}

#[derive(Debug, Clone)]
pub enum MainMenuMessage {
    FilePicklistMsg(String),
    GraphicsPicklistMsg(String),
    AnalysisPicklistMsg(String),
    SettingsPicklistMsg(String),
    HelpPicklistMsg(String),
    PointUpdated(Point),
    DoNothing,
}

impl FreeEta {
    pub fn new() -> (Self, Task<MainMenuMessage>) {
        (
            Self {
                file_picklist: None,
                mouse_point: Point::ORIGIN,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: MainMenuMessage) -> Task<MainMenuMessage> {
        match message {
            MainMenuMessage::FilePicklistMsg(s) => {
                // TODO: Divide different sections
                self.file_picklist = Some(s);
            }
            MainMenuMessage::PointUpdated(p) => {
                self.mouse_point = p;
            }
            MainMenuMessage::GraphicsPicklistMsg(s) => {
                // TODO: Divide different sections
                self.file_picklist = Some(s);
            }
            MainMenuMessage::AnalysisPicklistMsg(s) => {
                // TODO: Divide different sections
                self.file_picklist = Some(s);
            }
            MainMenuMessage::SettingsPicklistMsg(s) => {
                // TODO: Divide different sections
                self.file_picklist = Some(s);
            }
            MainMenuMessage::HelpPicklistMsg(s) => {
                // TODO: Divide different sections
                self.file_picklist = Some(s);
            }
            MainMenuMessage::DoNothing => {}
        }
        Task::none()
    }

    pub fn view(&self) -> Element<MainMenuMessage> {
        column!(
            // Funtion row
            row![
                // File
                functional_picklist(
                    [
                        "🆕 New.. ",
                        "📂 Open..",
                        "🔎 Open Recent",
                        "💾 Save",
                        "📤 Export"
                    ]
                    .map(|s| s.to_string())
                    .to_vec(),
                    self.file_picklist.clone(),
                    |s| MainMenuMessage::FilePicklistMsg(s),
                    "📁 File"
                )
                .style(freeeta_styles::functional_picklist_style),
                // Graphics
                functional_picklist(
                    // FIXME: Better use container[svg/png]
                    ["⛽ Pop", "🌀 Valve", "➕ Add more..."]
                        .map(|s| s.to_string())
                        .to_vec(),
                    self.file_picklist.clone(),
                    |s| MainMenuMessage::GraphicsPicklistMsg(s),
                    "💠 Graphics"
                )
                .style(freeeta_styles::functional_picklist_style),
                // Analysis
                functional_picklist(
                    ["🌲 Draw ETA Tree", "📉 Calculate Success & Failure Rates"]
                        .map(|s| s.to_string())
                        .to_vec(),
                    self.file_picklist.clone(),
                    |s| MainMenuMessage::AnalysisPicklistMsg(s),
                    "🧭 Analysis"
                )
                .style(freeeta_styles::functional_picklist_style),
                // Settings
                functional_picklist(
                    ["🔮 Themes", "🗣️ Languages"]
                        .map(|s| s.to_string())
                        .to_vec(),
                    self.file_picklist.clone(),
                    |s| MainMenuMessage::SettingsPicklistMsg(s),
                    "⚙️ Settings"
                )
                .style(freeeta_styles::functional_picklist_style),
                // Help
                functional_picklist(
                    ["📔 FreeEta Handbook", "🌏 About FreeEta", "🧊 About ICED"]
                        .map(|s| s.to_string())
                        .to_vec(),
                    self.file_picklist.clone(),
                    |s| MainMenuMessage::HelpPicklistMsg(s),
                    "🤝 Help"
                )
                .style(freeeta_styles::functional_picklist_style),
                // Space[ ]
                horizontal_space(),
                // FreeEta logo
                container(
                    svg(Handle::from_path("static/svg/logo.svg"))
                        .width(32.)
                        .height(32.)
                        .content_fit(ContentFit::ScaleDown)
                )
                .padding(0u16)
                .style(container::rounded_box)
            ],
            horizontal_rule(0),
            vertical_space(),
            // Body
            // TODO: Replace it with canvas view.
            container(
                column![
                    freeeta_buttons::bookmark(
                        MainMenuMessage::DoNothing,
                        Font::default(),
                        "VIEW",
                        Color::from_rgb(0.92, 0.21, 0.36)
                    ),
                    freeeta_buttons::bookmark(
                        MainMenuMessage::DoNothing,
                        Font::default(),
                        "ETA",
                        Color::from_rgb(0., 0.64, 0.51)
                    ),
                    freeeta_buttons::bookmark(
                        MainMenuMessage::DoNothing,
                        Font::default(),
                        "Export",
                        Color::from_rgb(0.95, 0.6, 0.)
                    ),
                ]
                .spacing(10.)
                .align_x(alignment::Horizontal::Left)
            )
            .align_x(alignment::Horizontal::Left),
            vertical_space(),
            // Bottom row
            horizontal_rule(0),
            row![
                text(format!("{:?}", self.mouse_point)).color(Color::from_rgb(0.96, 0.31, 0.64)),
                text(
                    "|  Copyright©Shanghai Justlinking Safety Technology co.,ltd.    \
                Administroot<boli_lemon@foxmail.com>  "
                )
                .style(freeeta_styles::bottomline_text_unselected)
                .align_x(alignment::Horizontal::Center)
            ]
            .align_y(alignment::Vertical::Bottom)
        )
        .into()
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Light
    }

    pub fn subscription(&self) -> Subscription<MainMenuMessage> {
        event::listen_with(|event, status, _window| {
            match (event, status) {
                // Using mouse
                (Event::Mouse(CursorMoved { position }), Status::Ignored)
                // Or using touchboard
                | (Event::Touch(FingerMoved {position, ..}), Status::Ignored) => {
                    Some(MainMenuMessage::PointUpdated(position))
                }
                _ => None
            }
        })
    }
}
