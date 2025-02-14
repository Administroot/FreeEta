use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, row, text, vertical_rule,
    vertical_space, Column, Container, Rule,
};
use iced::{alignment, Color, Length, Theme};

use crate::freeeta_styles;
use crate::main_menu::MainMenuMessage;

/// Horizontal branche of ETA tree
/// - `content`: the content of the branch
/// - `is_frontal`: whether it is the front one relative to the parent branch
pub fn eta_horizontal_branch(content: &str, is_frontal: bool) -> Column<MainMenuMessage, Theme> {
    let vertical_compensatory_line = if !is_frontal {
        eta_vertical_branch(1)
    } else {
        container("").height(Length::Fill)
    };
    column![
        row![
            vertical_compensatory_line,
            horizontal_space(),
            text(content)
                .size(20.)
                .width(Length::Shrink)
                .height(Length::Shrink),
            horizontal_space()
        ]
        .align_y(alignment::Vertical::Bottom),
        horizontal_rule(2.).style(freeeta_styles::eta_horizontal_rule_style)
    ]
    .height(Length::FillPortion(1))
    .width(Length::Fill)
    .into()
}

/// Vertical branch of ETA tree
/// height: the height of the branch, relative to Length::FillPortion()
pub fn eta_vertical_branch<'a>(height: u16) -> Container<'a, MainMenuMessage, Theme> {
    container(vertical_rule(2.).style(freeeta_styles::eta_vertical_branch_style))
        .height(Length::FillPortion(height))
}

pub fn event_seperate_line<'a>(color: Color) -> Rule<'a, Theme> {
    vertical_rule(0.).style(move |theme| freeeta_styles::event_seperate_line_style(color, theme))
}

pub fn eta_output_branch(content: &str) -> Column<MainMenuMessage, Theme> {
    column![
        button(
            row![
                vertical_space().height(Length::FillPortion(1)),
                text(content)
                    .size(20.)
                    .width(Length::Shrink)
                    .height(Length::Shrink),
                horizontal_space()
            ]
            .align_y(alignment::Vertical::Bottom)
        )
        .style(freeeta_styles::invisiable_button_style)
        .on_press(MainMenuMessage::DoNothing),
        horizontal_rule(2.).style(freeeta_styles::invisiable_rule_style),
    ]
    .height(Length::FillPortion(1))
    .width(Length::Fill)
    .into()
}
