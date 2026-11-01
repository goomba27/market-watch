use crate::app::{Message, Page};
use cosmic::Element;
use cosmic::iced::Length;
use cosmic::widget;

pub fn view<'a>(list_name: &'a str) -> Element<'a, Message> {
    let back_button =
        widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
            .label("Back")
            .on_press(Message::OpenPage(Page::Watchlist));

    let save_list_button = widget::button::icon(widget::icon::from_name(
        "document-save-as-symbolic",
    ))
    .label("Save List")
    .on_press(Message::CreateWatchlist);

    let row = widget::row::with_capacity(2)
        .push(back_button)
        .push(save_list_button)
        .spacing(12)
        .padding(16)
        .width(Length::Fill);

    let title = widget::text::title2("New List");

    let name_field = widget::text_input("List name", list_name)
        .on_input(Message::NewListNameChanged)
        .width(Length::Fill);

    widget::column::with_capacity(3)
        .push(row)
        .push(title)
        .push(name_field)
        .spacing(12)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fixed(600.0))
        .into()
}
