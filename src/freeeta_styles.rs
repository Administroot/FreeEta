use iced::{
    border::{rounded, Radius},
    widget::{button, container, pick_list, text},
    Background, Border, Color, Shadow, Theme, Vector,
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

pub fn shadowed_container(_theme: &Theme) -> container::Style {
    container::Style {
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector { x: 0., y: 0. },
            blur_radius: 8.,
        },
        background: Some(Background::Color(Color::BLACK)),
        border: rounded(20.),
        ..Default::default()
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

pub fn bookmark_style(_theme: &Theme, status: button::Status, color: Color) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(color)),
            text_color: Color::BLACK,
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(mix_colors(Color::WHITE, color))),
            text_color: Color::BLACK,
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(color)),
            text_color: Color::BLACK,
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(mix_colors(Color::TRANSPARENT, color))),
            text_color: Color::BLACK,
            ..Default::default()
        },
    }
}
