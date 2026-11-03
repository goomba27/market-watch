use crate::app::Message;
use crate::globals;
use crate::model::watchlist::Watchlist;
use crate::ui::common::{build_header, generate_ticker_list};
use cosmic::Element;
use cosmic::iced::{Alignment, Length};
use cosmic::widget;

pub fn create_new_list_view<'a>(list_name: &'a str) -> Element<'a, Message> {
    let save_list_button = widget::container(
        widget::button::icon(widget::icon::from_name(
            "document-save-as-symbolic",
        ))
        .label("Save List")
        .on_press(Message::CreateWatchlist),
    )
    .width(Length::Fill)
    .align_x(Alignment::End)
    .padding([0, 8]);

    let name_field = widget::text_input("List name", list_name)
        .on_input(Message::NewListNameChanged)
        .width(Length::Fill);

    widget::column::with_capacity(3)
        .push(build_header("Create List"))
        .push(save_list_button)
        .push(name_field)
        .width(Length::Fill)
        .height(Length::Fixed(globals::POPUP_HEIGHT))
        .into()
}

pub fn edit_list_view<'a>(
    list: Option<&'a Watchlist>,
    ticker_input: &'a str,
) -> Element<'a, Message> {
    //No list yet
    if list.is_none() {
        return widget::column::with_capacity(2)
            .push(build_header("Edit List"))
            .push(
                widget::container(widget::text::body("Create a list first"))
                    .padding(16),
            )
            .width(Length::Fill)
            .height(Length::Fixed(globals::POPUP_HEIGHT))
            .into();
    }

    let symbol_field = widget::text_input("Symbol, e.g AAPL", ticker_input)
        .on_input(Message::TickerInputChanged)
        .on_submit(Message::AddTicker)
        .width(Length::Fill);

    let add_button =
        widget::button::icon(widget::icon::from_name("list-add-symbolic"))
            .label("Add to List")
            .on_press(Message::AddTicker(ticker_input.to_string()));

    let add_row = widget::row::with_capacity(2)
        .push(symbol_field)
        .push(add_button)
        .spacing(8)
        .padding(12)
        .align_y(Alignment::Center);

    let body = generate_ticker_list(list, true);

    // puts everything together
    widget::column::with_capacity(4)
        .push(build_header("Edit List"))
        .push(add_row)
        .push(body)
        .width(Length::Fill)
        .height(Length::Fixed(globals::POPUP_HEIGHT))
        .into()
}
