use iced::widget::{
    column, horizontal_rule, horizontal_space, row, text, vertical_rule, Column, Rule,
};
use iced::{Color, Length, Theme};

use crate::freeeta_styles;
use crate::main_menu::MainMenuMessage;

pub fn eta_horizontal_branch(content: &str) -> Column<MainMenuMessage, Theme> {
    column![
        row![horizontal_space(), text(content), horizontal_space()],
        horizontal_rule(2.).style(freeeta_styles::eta_horizontal_rule_style)
    ]
    // .height(Length::Fixed(30.))
    .height(Length::Shrink)
    .width(Length::Fill)
    .into()
}

pub fn eta_vertical_branch<'a>() -> Rule<'a, Theme> {
    vertical_rule(2.).style(freeeta_styles::eta_vertical_branch_style)
}

pub fn event_seperate_line<'a>(color: Color) -> Rule<'a, Theme> {
    vertical_rule(0.).style(move |theme| freeeta_styles::event_seperate_line_style(color, theme))
}
