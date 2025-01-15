mod freeeta_styles;
mod freeeta_yml;
mod main_menu;

use iced;

fn main() -> iced::Result {
    freeeta_yml::read_freeeta_config().expect("[ERROR] CONFIG ERROR");
    iced::application(
        "FreeEta",
        main_menu::FreeEta::update,
        main_menu::FreeEta::view,
    )
    .theme(main_menu::FreeEta::theme)
    .run()
}
