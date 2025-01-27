mod freeeta_buttons;
mod freeeta_picklists;
mod freeeta_serial;
mod freeeta_styles;
mod main_menu;
mod pages;

use iced;

fn main() -> iced::Result {
    let freeeta_yml = freeeta_serial::read_freeeta_config().expect("[ERROR] CONFIG ERROR");
    // TODO: Deal with the config struct.
    println!("{:?}", freeeta_yml);
    iced::application(
        "FreeEta",
        main_menu::FreeEta::update,
        main_menu::FreeEta::view,
    )
    .theme(main_menu::FreeEta::theme)
    .subscription(main_menu::FreeEta::subscription)
    .run()
}
