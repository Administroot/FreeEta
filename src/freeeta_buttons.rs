use crate::freeeta_styles;
use crate::main_menu::MainMenuMessage;
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{button, Button, Text, Tooltip};
use iced::{Alignment, Color, Font, Length, Pixels, Theme};

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
    background_color: Color,
    is_actived: bool,
) -> Tooltip<'a, MainMenuMessage, Theme> {
    let mut button = button(
        Text::new(inner_text.to_string())
            .font(font)
            .align_y(Alignment::Center)
            .align_x(Alignment::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .style(move |theme, status| {
        freeeta_styles::bookmark_style(theme, status, background_color, is_actived)
    })
    .height(40)
    .on_press(message);

    // Lengthen and hignlight the button when activated
    button = match is_actived {
        true => button.width(80),
        false => button.width(60),
    };

    let bookmark = Tooltip::new(
        button,
        Text::new("Press to active / deactive")
            .font(font)
            .size(Pixels { 0: 15f32 }),
        Position::Left,
    )
    .gap(5);
    return bookmark;
}

pub fn eta_event_header<'a>(
    message: MainMenuMessage,
    font: Font,
    inner_text: &str,
    background_color: Color,
) -> Button<'a, MainMenuMessage, Theme> {
    button(
        Text::new(inner_text.to_string())
            .size(20.)
            .font(font)
            .align_y(Alignment::Center)
            .align_x(Alignment::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .style(move |theme, status| {
        freeeta_styles::eta_event_header_style(theme, status, background_color)
    })
    .width(Length::Shrink)
    .height(30)
    .padding(5)
    .on_press(message)
}
