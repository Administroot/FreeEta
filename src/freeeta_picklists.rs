use std::borrow::Borrow;

use iced::widget::pick_list;
use iced::{
    widget::{text::Shaping, PickList},
    Length,
};

/// Picklist to select functions.
pub fn functional_picklist<'a, T, L, V, Message, Theme, Renderer>(
    options: L,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
    placeholder: &str,
) -> PickList<'a, T, L, V, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone,
    Theme: iced::widget::pick_list::Catalog + iced::widget::overlay::menu::Catalog,
    Renderer: iced::advanced::text::Renderer,
{
    pick_list::<'a, T, L, V, Message, Theme, Renderer>(options, selected, on_selected)
        .width(Length::Shrink)
        .placeholder(placeholder)
        .text_shaping(Shaping::Advanced)
}
