use iced::widget::{column, horizontal_rule, text, vertical_rule, Column, Rule};
use iced::{Color, Theme};

use crate::freeeta_styles;
use crate::main_menu::MainMenuMessage;

pub fn eta_branch(content: &str) -> Column<MainMenuMessage, Theme> {
    column![
        text(content),
        horizontal_rule(0.).style(freeeta_styles::eta_branch_style)
    ]
    .into()
}

pub fn event_seperate_line<'a>(color: Color) -> Rule<'a, Theme> {
    vertical_rule(0.).style(move |theme| freeeta_styles::event_seperate_line_style(color, theme))
}
