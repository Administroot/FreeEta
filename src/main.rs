mod freeeta_styles;
mod main_menu;

use iced;

fn main() -> iced::Result {
    iced::application(
        "FreeEta",
        main_menu::FreeEta::update,
        main_menu::FreeEta::view,
    )
    .run()
}
