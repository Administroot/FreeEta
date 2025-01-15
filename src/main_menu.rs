use iced::{
    alignment,
    event::{self, Status},
    mouse::Event::CursorMoved,
    touch::Event::FingerMoved,
    widget::{
        column, container, horizontal_rule, horizontal_space, pick_list, row, svg, svg::Handle,
        text, text::Shaping, vertical_space,
    },
    Color, ContentFit, Element, Event, Length, Point, Subscription, Task,
};

use crate::freeeta_styles;

pub struct FreeEta {
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
    PointUpdated(Point),
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
        }
        Task::none()
    }

    pub fn view(&self) -> Element<MainMenuMessage> {
        column!(
            // Funtion row
            row![
                // File
                pick_list(
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
                )
                .width(Length::Shrink)
                .placeholder("📁 File")
                .text_shaping(Shaping::Advanced)
                .style(freeeta_styles::pick_list_unselected),
                // Graphics
                pick_list(
                    // TODO: Please use container[svg/png]
                    ["⛽ Pop", "🌀 Valve", "Add more..."]
                        .map(|s| s.to_string())
                        .to_vec(),
                    self.file_picklist.clone(),
                    |s| MainMenuMessage::FilePicklistMsg(s),
                )
                .width(Length::Shrink)
                .placeholder("💠 Graphics")
                .text_shaping(Shaping::Advanced)
                .style(freeeta_styles::pick_list_unselected),
                // Analysis
                pick_list(
                    ["🌲 Draw ETA Tree", "📉 Calculate Success & Failure Rates"]
                        .map(|s| s.to_string())
                        .to_vec(),
                    self.file_picklist.clone(),
                    |s| MainMenuMessage::FilePicklistMsg(s),
                )
                .width(Length::Shrink)
                .placeholder("🧭 Analysis")
                .text_shaping(Shaping::Advanced)
                .style(freeeta_styles::pick_list_unselected),
                // Help
                pick_list(
                    [
                        "📔 FreeEta Handbook",
                        "🗣️ Language",
                        "🌏 About FreeEta",
                        "🧊 About ICED"
                    ]
                    .map(|s| s.to_string())
                    .to_vec(),
                    self.file_picklist.clone(),
                    |s| MainMenuMessage::FilePicklistMsg(s),
                )
                .width(Length::Shrink)
                .placeholder(" Help")
                .text_shaping(Shaping::Advanced)
                .style(freeeta_styles::pick_list_unselected),
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
            // TODO: Replace it with canvas view.
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
