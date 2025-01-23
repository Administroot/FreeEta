use iced::{
    border::Radius,
    widget::{button, pick_list, text},
    Background, Border, Color, Theme,
};

// TODO: Read Theme from const in the future.
pub fn functional_picklist_style(_theme: &Theme, _status: pick_list::Status) -> pick_list::Style {
    pick_list::Style {
        text_color: Color::from_rgb(0.09, 0.02, 0.08),
        placeholder_color: Color::from_rgb(0.09, 0.02, 0.08),
        handle_color: Color::from_rgb(0.36, 0.40, 0.67),
        background: Background::Color(Color::from_rgb(0.64, 0.84, 0.87)),
        border: Border {
            color: Color::from_rgb(0.47, 0.46, 0.45),
            width: 2.,
            radius: Radius {
                top_left: 5.,
                top_right: 5.,
                bottom_right: 5.,
                bottom_left: 5.,
            },
        },
    }
}

pub fn bottomline_text_unselected(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(Color::from_rgb(0.35, 0.35, 0.34)),
    }
}

/// Returns the average of two colors; color intensity is fixed to 100%
fn mix_colors(color_1: Color, color_2: Color) -> Color {
    Color {
        r: (color_1.r + color_2.r) / 2.0,
        g: (color_1.g + color_2.g) / 2.0,
        b: (color_1.b + color_2.b) / 2.0,
        a: 1.0,
    }
}

/// Half transparency of the given color.
fn half_transparency(color: Color) -> Color {
    Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a / 2.0,
    }
}

pub fn bookmark_style(
    _theme: &Theme,
    status: button::Status,
    color: Color,
    is_actived: bool,
) -> button::Style {
    let text_color = Color::BLACK;
    let border = Border {
        color,
        width: 2.,
        radius: Radius {
            top_left: 0.,
            top_right: 5.,
            bottom_right: 5.,
            bottom_left: 0.,
        },
    };

    match (status, is_actived) {
        (button::Status::Active, true) => button::Style {
            background: Some(Background::Color(color)),
            text_color,
            border,
            ..Default::default()
        },
        (button::Status::Hovered, _) => button::Style {
            background: Some(Background::Color(mix_colors(Color::WHITE, color))),
            text_color,
            border,
            ..Default::default()
        },
        (button::Status::Pressed, _) => button::Style {
            background: Some(Background::Color(color)),
            text_color,
            border,
            ..Default::default()
        },
        (_, _) => button::Style {
            background: Some(Background::Color(half_transparency(color))),
            text_color,
            border,
            ..Default::default()
        },
    }
}