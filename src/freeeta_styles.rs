use iced::{
    border::{radius, Radius},
    color,
    widget::{button, pick_list, rule, text},
    Background, Border, Color, Pixels, Theme,
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
            top_left: 5.,
            top_right: 0.,
            bottom_right: 0.,
            bottom_left: 5.,
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

pub fn eta_event_header_style(
    _theme: &Theme,
    status: button::Status,
    color: Color,
) -> button::Style {
    let text_color = Color::BLACK;
    let border = Border {
        color,
        width: 1.,
        radius: radius(Pixels { 0: 2. }),
    };

    match status {
        button::Status::Active | button::Status::Pressed => button::Style {
            background: Some(Background::Color(color)),
            text_color,
            border,
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(mix_colors(Color::WHITE, color))),
            text_color,
            border,
            ..Default::default()
        },
        _ => button::Style {
            background: Some(Background::Color(half_transparency(color))),
            text_color,
            border,
            ..Default::default()
        },
    }
}

/// Pick a color from color palette
/// num: color selected, range [0, +∞)
pub fn get_a_color(num: usize) -> Color {
    let palette: Vec<Color> = vec![
        color!(0x2ecc71),
        color!(0x3498db),
        color!(0x9b59b6),
        color!(0xf1c40f),
        color!(0xe67e22),
        color!(0xe74c3c),
        color!(0xfd79a8),
        color!(0xba79b1),
        color!(0x7efff5),
    ];

    let color_number = num % palette.len();
    return *palette.get(color_number).unwrap();
}

pub fn eta_horizontal_rule_style(_theme: &Theme) -> rule::Style {
    rule::Style {
        width: 2,
        fill_mode: rule::FillMode::Full,
        color: Color::BLACK,
        radius: radius(Pixels::ZERO),
    }
}

pub fn invisiable_rule_style(_theme: &Theme) -> rule::Style {
    rule::Style {
        width: 2,
        fill_mode: rule::FillMode::Full,
        color: Color::TRANSPARENT,
        radius: radius(Pixels::ZERO),
    }
}

pub fn event_seperate_line_style(color: Color, _theme: &Theme) -> rule::Style {
    rule::Style {
        width: 1,
        fill_mode: rule::FillMode::Padded(2),
        color,
        radius: radius(Pixels::ZERO),
    }
}

pub fn eta_vertical_branch_style(_theme: &Theme) -> rule::Style {
    rule::Style {
        width: 2,
        fill_mode: rule::FillMode::Full,
        color: Color::BLACK,
        radius: radius(Pixels::ZERO),
    }
}

pub fn eta_output_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(half_transparency(Color::from_rgb(
                0.63, 0.62, 0.62,
            )))),
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.63, 0.62, 0.62))),
            ..Default::default()
        },
        button::Status::Disabled | button::Status::Active => button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        },
    }
}
