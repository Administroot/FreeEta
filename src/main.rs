mod freeeta_styles;
mod freeeta_yml;
mod main_menu;

use iced;

fn main() -> iced::Result {
    let freeeta_yml = freeeta_yml::read_freeeta_config().expect("[ERROR] CONFIG ERROR");
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
