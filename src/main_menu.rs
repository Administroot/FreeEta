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
    view_bookmark_status: bool,
    eta_bookmark_status: bool,
    developer_bookmark_status: bool,
    export_bookmark_status: bool,
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
    ViewBookmarkMsg,
    EtaBookmarkMsg,
    DeveloperBookmarkMsg,
    ExportBookmarkMsg,
}

impl FreeEta {
    pub fn new() -> (Self, Task<MainMenuMessage>) {
        (
            Self {
                file_picklist: None,
                mouse_point: Point::ORIGIN,
                view_bookmark_status: false,
                eta_bookmark_status: false,
                developer_bookmark_status: false,
                export_bookmark_status: false,
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
            MainMenuMessage::ViewBookmarkMsg => {
                self.view_bookmark_status = !self.view_bookmark_status;
            }
            MainMenuMessage::EtaBookmarkMsg => {
                self.eta_bookmark_status = !self.eta_bookmark_status;
            }
            MainMenuMessage::DeveloperBookmarkMsg => {
                self.developer_bookmark_status = !self.developer_bookmark_status;
            }
            MainMenuMessage::ExportBookmarkMsg => {
                self.export_bookmark_status = !self.export_bookmark_status;
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<MainMenuMessage> {
        let file_picklist = functional_picklist(
            [
                "🆕 New.. ",
                "📂 Open..",
                "🔎 Open Recent",
                "💾 Save",
                "📤 Export",
            ]
            .map(|s| s.to_string())
            .to_vec(),
            self.file_picklist.clone(),
            |s| MainMenuMessage::FilePicklistMsg(s),
            "📁 File",
        )
        .style(freeeta_styles::functional_picklist_style);
        let graphics_picklist = functional_picklist(
            // FIXME: Better use container[svg/png]
            ["⛽ Pop", "🌀 Valve", "➕ Add more..."]
                .map(|s| s.to_string())
                .to_vec(),
            self.file_picklist.clone(),
            |s| MainMenuMessage::GraphicsPicklistMsg(s),
            "💠 Graphics",
        )
        .style(freeeta_styles::functional_picklist_style);
        let analysis_picklist = functional_picklist(
            ["🌲 Draw ETA Tree", "📉 Calculate Success & Failure Rates"]
                .map(|s| s.to_string())
                .to_vec(),
            self.file_picklist.clone(),
            |s| MainMenuMessage::AnalysisPicklistMsg(s),
            "🧭 Analysis",
        )
        .style(freeeta_styles::functional_picklist_style);
        let settings_picklist = functional_picklist(
            ["🔮 Themes", "🗣️ Languages"]
                .map(|s| s.to_string())
                .to_vec(),
            self.file_picklist.clone(),
            |s| MainMenuMessage::SettingsPicklistMsg(s),
            "⚙️ Settings",
        )
        .style(freeeta_styles::functional_picklist_style);
        let help_picklist = functional_picklist(
            ["📔 FreeEta Handbook", "🌏 About FreeEta", "🧊 About ICED"]
                .map(|s| s.to_string())
                .to_vec(),
            self.file_picklist.clone(),
            |s| MainMenuMessage::HelpPicklistMsg(s),
            "🤝 Help",
        )
        .style(freeeta_styles::functional_picklist_style);

        let view_bookmark = freeeta_buttons::bookmark(
            MainMenuMessage::ViewBookmarkMsg,
            Font::default(),
            "View",
            Color::from_rgb(0.92, 0.21, 0.36),
            self.view_bookmark_status,
        );
        let eta_bookmark = freeeta_buttons::bookmark(
            MainMenuMessage::EtaBookmarkMsg,
            Font::default(),
            "ETA",
            Color::from_rgb(0., 0.64, 0.51),
            self.eta_bookmark_status,
        );
        let developer_bookmark = freeeta_buttons::bookmark(
            MainMenuMessage::DeveloperBookmarkMsg,
            Font::default(),
            "Developer",
            Color::from_rgb(0.88, 0.6, 0.71),
            self.developer_bookmark_status,
        );
        let export_bookmark = freeeta_buttons::bookmark(
            MainMenuMessage::ExportBookmarkMsg,
            Font::default(),
            "Export",
            Color::from_rgb(0.95, 0.6, 0.),
            self.export_bookmark_status,
        );

        let logo = container(
            svg(Handle::from_path("static/svg/logo.svg"))
                .width(32.)
                .height(32.)
                .content_fit(ContentFit::ScaleDown),
        )
        .padding(0u16)
        .style(container::rounded_box);

        let mouse_position =
            text(format!("{:?}", self.mouse_point)).color(Color::from_rgb(0.96, 0.31, 0.64));
        let admonition = text(
            "|  Copyright©Shanghai Justlinking Safety Technology co.,ltd.    \
        Administroot<boli_lemon@foxmail.com>  ",
        )
        .style(freeeta_styles::bottomline_text_unselected)
        .align_x(alignment::Horizontal::Center);

        column!(
            // Functional row
            row![
                file_picklist,
                graphics_picklist,
                analysis_picklist,
                settings_picklist,
                help_picklist,
                horizontal_space(),
                logo,
            ],
            horizontal_rule(0),
            vertical_space(),
            // Body
            container(
                column![
                    view_bookmark,
                    eta_bookmark,
                    developer_bookmark,
                    export_bookmark,
                ]
                .spacing(10.)
                .align_x(alignment::Horizontal::Left)
            )
            .align_x(alignment::Horizontal::Left),
            vertical_space(),
            // Bottom row
            horizontal_rule(0),
            row![mouse_position, admonition].align_y(alignment::Vertical::Bottom)
        )
        .into()
    }

    pub fn theme(&self) -> iced::Theme {
        // TODO: Read from config struct var.
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
