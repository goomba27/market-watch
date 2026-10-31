use crate::app::{Message, Page};
use cosmic::Element;
use cosmic::iced::Length;
use cosmic::widget;

pub fn view<'a>() -> Element<'a, Message> {
    let back_button =
        widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
            .label("Back")
            .on_press(Message::OpenPage(Page::Watchlist));

    let title = widget::text::title2("New List");

    let name_field = widget::text_input("List name", "").width(Length::Fill);

    widget::column::with_capacity(3)
        .push(back_button)
        .push(title)
        .push(name_field)
        .spacing(12)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fixed(600.0))
        .into()
}
