mod freeeta_buttons;
mod freeeta_picklists;
mod freeeta_rules;
mod freeeta_serial;
mod freeeta_styles;
mod main_menu;
mod pages;

use iced;

fn main() -> iced::Result {
    iced::application(
        "FreeEta",
        main_menu::FreeEta::update,
        main_menu::FreeEta::view,
    )
    .theme(main_menu::FreeEta::theme)
    .subscription(main_menu::FreeEta::subscription)
    .run()
}
