use iced::{
    widget::{column, container, horizontal_space, pick_list, row, svg, svg::Handle},
    ContentFit, Element, Task,
};

use crate::freeeta_styles;

pub struct FreeEta {
    file_picklist: Option<String>,
}

impl Default for FreeEta {
    fn default() -> Self {
        FreeEta::new().0
    }
}

#[derive(Debug, Clone)]
pub enum MainMenuMessage {
    DoNothing,
}

impl FreeEta {
    pub fn new() -> (Self, Task<MainMenuMessage>) {
        (
            Self {
                file_picklist: None,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, _message: MainMenuMessage) -> Task<MainMenuMessage> {
        Task::none()
    }

    pub fn view(&self) -> Element<MainMenuMessage> {
        column!(
            // Funtion row
            row![
                // File
                pick_list(
                    ["Other choices 1", "Other choices 2"]
                        .map(|s| s.to_string())
                        .to_vec(),
                    self.file_picklist.clone(),
                    |_| MainMenuMessage::DoNothing,
                )
                .placeholder("File")
                .style(freeeta_styles::pick_list_unselected),
                // Graphics
                pick_list(
                    ["Other choices 1", "Other choices 2"]
                        .map(|s| s.to_string())
                        .to_vec(),
                    self.file_picklist.clone(),
                    |_| MainMenuMessage::DoNothing,
                )
                .placeholder("Graphics")
                .style(freeeta_styles::pick_list_unselected),
                // Tools
                pick_list(
                    ["Other choices 1", "Other choices 2"]
                        .map(|s| s.to_string())
                        .to_vec(),
                    self.file_picklist.clone(),
                    |_| MainMenuMessage::DoNothing,
                )
                .placeholder("Tools")
                .style(freeeta_styles::pick_list_unselected),
                // Helps
                pick_list(
                    ["Other choices 1", "Other choices 2"]
                        .map(|s| s.to_string())
                        .to_vec(),
                    self.file_picklist.clone(),
                    |_| MainMenuMessage::DoNothing,
                )
                .placeholder("Helps")
                .style(freeeta_styles::pick_list_unselected),
                // Space[ ]
                horizontal_space(),
                // FreeEta logo
                container(
                    svg(Handle::from_path("assets/logo.svg"))
                        .width(32.)
                        .height(32.)
                        .content_fit(ContentFit::ScaleDown)
                )
                .padding(0u16)
                .style(container::rounded_box)
            ]
        )
        .into()
    }
}
