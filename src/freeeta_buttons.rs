use crate::freeeta_styles;
use crate::main_menu::MainMenuMessage;
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{button, Text, Tooltip};
use iced::{Alignment, Color, Font, Pixels, Theme};

#[allow(dead_code)]
/// Button '×'
pub fn button_hide<'a>(
    message: MainMenuMessage,
    font: Font,
) -> Tooltip<'a, MainMenuMessage, Theme> {
    Tooltip::new(
        button(
            Text::new("×")
                .font(font)
                .align_y(Alignment::Center)
                .align_x(Alignment::Center)
                .size(15)
                .line_height(LineHeight::Relative(1.0)),
        )
        .padding(2)
        .height(20)
        .width(20)
        .on_press(message),
        Text::new("Cancel").font(font),
        Position::Right,
    )
    .gap(5)
}

/// Bookmark
pub fn bookmark<'a>(
    message: MainMenuMessage,
    font: Font,
    inner_text: &str,
    color: Color,
) -> Tooltip<'a, MainMenuMessage, Theme> {
    let bookmark = Tooltip::new(
        button(
            Text::new(inner_text.to_string())
                .font(font)
                .align_y(Alignment::Center)
                .align_x(Alignment::Center)
                // .size(15)
                .line_height(LineHeight::Relative(1.0)),
        )
        .style(move |theme, status| freeeta_styles::bookmark_style(theme, status, color))
        .padding(2)
        .height(40)
        .width(80)
        .on_press(message),
        Text::new("Press to active / deactive")
            .font(font)
            .size(Pixels { 0: 15f32 }),
        Position::Right,
    )
    .gap(5);
    return bookmark;
}
