use iced::widget::{svg, text, vertical_rule};
use iced::{
    alignment,
    event::{self, Status},
    mouse::Event::CursorMoved,
    touch::Event::FingerMoved,
    widget::{
        column, container, horizontal_rule, horizontal_space, row, svg::Handle, vertical_space,
    },
    window::Event::Resized,
    Color, ContentFit, Element, Event, Font, Point, Subscription, Task,
};
use iced::{Length, Size};

use crate::freeeta_serial::{self, EtaEntity};
use crate::pages::Pages;
use crate::{freeeta_buttons, freeeta_picklists::functional_picklist, freeeta_styles};

pub struct FreeEta {
    // TODO: Actually, I don't need this member.
    file_picklist: Option<String>,
    pub mouse_point: Point,
    view_bookmark_status: bool,
    eta_bookmark_status: bool,
    developer_bookmark_status: bool,
    export_bookmark_status: bool,
    page: Pages,
    pub eta: EtaEntity,
    pub window_size: Size,
    pub is_dragging: Vec<bool>,
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
    DoNothing,
    WindowSizeUpdated(Size),
    DragStart(usize),
    DragEnded(usize),
    Save(String),
    Sync(String),
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
                page: Pages::MainMenuPage,
                eta: get_eta("default_eta.json"),
                window_size: Size::ZERO,
                is_dragging: vec![false, false, false],
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: MainMenuMessage) -> Task<MainMenuMessage> {
        match message {
            MainMenuMessage::FilePicklistMsg(s) => {
                // TODO: Divide different sections
                if s == "💾 Save" {
                    MainMenuMessage::Save(String::from("default_eta.json"));
                } else if s == "📂 Open.." {
                    MainMenuMessage::Sync(String::from("default_eta.json"));
                } else {
                    self.file_picklist = Some(s);
                }
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
                if s == "🐛Debug" {
                    println!("{:?}", self.eta.nodes)
                } else {
                    self.file_picklist = Some(s)
                };
            }
            MainMenuMessage::ViewBookmarkMsg => {
                self.view_bookmark_status = !self.view_bookmark_status;
                if self.view_bookmark_status {
                    // Close other bookmarks
                    self.eta_bookmark_status = false;
                    self.developer_bookmark_status = false;
                    self.export_bookmark_status = false;
                    // Open corresponding page
                    self.page = Pages::InterfacePage;
                }
            }
            MainMenuMessage::EtaBookmarkMsg => {
                self.eta_bookmark_status = !self.eta_bookmark_status;
                if self.eta_bookmark_status {
                    // Close other bookmarks
                    self.view_bookmark_status = false;
                    self.developer_bookmark_status = false;
                    self.export_bookmark_status = false;
                    // Open corresponding page
                    self.page = Pages::ChartPage;
                }
            }
            MainMenuMessage::DeveloperBookmarkMsg => {
                self.developer_bookmark_status = !self.developer_bookmark_status;
                if self.developer_bookmark_status {
                    // Close other bookmarks
                    self.view_bookmark_status = false;
                    self.eta_bookmark_status = false;
                    self.export_bookmark_status = false;
                    // Open corresponding page
                    self.page = Pages::DeveloperPage;
                }
            }
            MainMenuMessage::ExportBookmarkMsg => {
                self.export_bookmark_status = !self.export_bookmark_status;
                if self.export_bookmark_status {
                    // Close other bookmarks
                    self.view_bookmark_status = false;
                    self.eta_bookmark_status = false;
                    self.developer_bookmark_status = false;
                    // Open corresponding page
                    self.page = Pages::ExportPage;
                }
            }
            MainMenuMessage::PointUpdated(p) => {
                self.mouse_point = p;
            }
            MainMenuMessage::DoNothing => {}
            MainMenuMessage::WindowSizeUpdated(s) => {
                self.window_size = s;
            }
            MainMenuMessage::DragStart(index) => {
                if self.is_dragging.len() < index {
                    self.is_dragging.push(true);
                } else {
                    self.is_dragging[index] = true;
                }
            }
            MainMenuMessage::DragEnded(index) => {
                if self.is_dragging.len() < index {
                    panic!("Internal Error: Vector is_dragging is too short.");
                } else {
                    self.is_dragging[index] = false;
                }
                self.eta =
                    freeeta_serial::read_eta_json("default_eta.json").expect("JSON format error");
            }
            MainMenuMessage::Save(s) => {
                freeeta_serial::update_eta_json(&s, self.eta.clone()).unwrap();
            }
            MainMenuMessage::Sync(s) => {
                self.eta = freeeta_serial::read_eta_json(&s).expect("JSON format error");
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
            [
                "📔 FreeEta Handbook",
                "🌏 About FreeEta",
                "🧊 About ICED",
                "🐛Debug",
            ]
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

        let mouse_position = container(
            text(format!("{:?}", self.mouse_point)).color(Color::from_rgb(0.96, 0.31, 0.64)),
        )
        .padding(3);
        let admonition = container(
            text(
                "  Copyright©Shanghai Justlinking Safety Technology co.,ltd.    \
        Administroot<boli_lemon@foxmail.com>  ",
            )
            .style(freeeta_styles::bottomline_text_unselected)
            .align_x(alignment::Horizontal::Center),
        )
        .padding(3);

        let screen = match self.page {
            Pages::MainMenuPage => self.main_menu_page(),
            Pages::InterfacePage => self.interface_page(),
            Pages::ChartPage => self.chart_page(),
            Pages::DeveloperPage => self.developer_page(),
            Pages::ExportPage => self.export_page(),
        };

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
            vertical_space().height(Length::Shrink),
            // Body
            container(
                row![
                    screen,
                    horizontal_space(),
                    // Bookmarks
                    column![
                        vertical_space(),
                        view_bookmark,
                        eta_bookmark,
                        developer_bookmark,
                        export_bookmark,
                        vertical_space(),
                    ]
                    .spacing(10)
                    .align_x(alignment::Horizontal::Right),
                ]
                .spacing(10)
            )
            .width(Length::Shrink)
            .align_x(alignment::Horizontal::Left),
            vertical_space().height(Length::Shrink),
            // Bottom row
            horizontal_rule(0),
            row![mouse_position, vertical_rule(2.), admonition]
                .align_y(alignment::Vertical::Bottom)
                .height(Length::Shrink)
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
                (Event::Window(Resized(size)), Status::Ignored) => {
                    Some(MainMenuMessage::WindowSizeUpdated(size))
                }
                _ => None
            }
        })
    }
}

/// Get user's customized eta
fn get_eta(path: &str) -> EtaEntity {
    freeeta_serial::read_eta_json(path).expect("JSON syntax error")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_json_test() {
        let freeeta = FreeEta::new();
        assert_ne!(freeeta.0.eta.nodes.is_empty(), true);
    }

    #[test]
    fn read_yaml_test() {
        let freeeta_yml = freeeta_serial::read_freeeta_config().expect("[ERROR] CONFIG ERROR");
        assert_eq!(freeeta_yml, freeeta_serial::FreeEtaConfig::new());
    }
}
