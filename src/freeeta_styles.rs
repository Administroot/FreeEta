use iced::{border::Radius, widget::pick_list, Background, Border, Color, Theme};

// pub fn radio_selected(_theme: &Theme, _status: radio::Status) -> radio::Style {
//     radio::Style {
//         text_color: Some(Color::from_rgb(0., 0., 1.)),
//         background: Background::Color(Color::from_rgb(1., 1., 1.)),
//         dot_color: Color::from_rgb(0., 0., 1.),
//         border_width: 1.0,
//         border_color: Color::from_rgb(0., 0., 1.),
//     }
// }

// pub fn radio_unselected(_theme: &Theme, _status: radio::Status) -> radio::Style {
//     radio::Style {
//         text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
//         background: Background::Color(Color::from_rgb(1., 1., 1.)),
//         dot_color: Color::from_rgb(0., 0., 1.),
//         border_width: 1.0,
//         border_color: Color::from_rgb(0., 0., 1.),
//     }
// }

// TODO: Read Theme from const in the future.
pub fn pick_list_unselected(_theme: &Theme, _status: pick_list::Status) -> pick_list::Style {
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
