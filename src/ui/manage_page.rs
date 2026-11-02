use crate::app::{Message, Page};
use crate::model::watchlist::Watchlist;
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

    let row = widget::row::with_capacity(2)
        .push(back_button())
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

pub fn edit_list_view<'a>(
    list: Option<&'a Watchlist>,
    ticker_input: &'a str,
) -> Element<'a, Message> {
    let Some(list) = list else {
        return widget::column::with_capacity(2)
            .push(back_button())
            .push(widget::text::body("Create a list first"))
            .spacing(12)
            .padding(16)
            .into();
    };

    let title = widget::container(widget::text::title2(&list.name))
        .width(Length::Fill)
        .align_x(Alignment::Center);

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
        .align_y(Alignment::Center);

    // iter and get existing symbols and generate rows
    // add the delete button
    let symbols: Element<Message> = if list.tickers.is_empty() {
        widget::text::body("No symbols yet").into()
    } else {
        let mut row_acc = widget::list_column();
        for (idx, t) in list.tickers.iter().enumerate() {
            row_acc = row_acc.add(editable_row(idx, t));
        }
        row_acc.into_element()
    };

    //put symbols in a scrall
    let scrollable = widget::container(widget::scrollable::vertical(symbols))
        .width(Length::Fill)
        .height(Length::Fill);

    // puts everything together
    widget::column::with_capacity(4)
        .push(back_button())
        .push(title)
        .push(add_row)
        .push(scrollable)
        .spacing(12)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fixed(600.0))
        .into()
}

pub fn back_button<'a>() -> Element<'a, Message> {
    widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
        .label("Back")
        .on_press(Message::OpenPage(Page::Watchlist))
        .into()
}

fn editable_row<'a>(index: usize, ticker: &'a str) -> Element<'a, Message> {
    let trash =
        widget::button::icon(widget::icon::from_name("edit-delete-symbolic"))
            .extra_small()
            .on_press(Message::RemoveTicker(index));

    let trash_container = widget::container(trash)
        .width(Length::Fill)
        .align_x(Alignment::End);

    let text = widget::text::body(ticker);

    widget::row::with_capacity(2)
        .push(text)
        .push(trash_container)
        // trash can is not the same height as the text, so center them
        .align_y(Alignment::Center)
        .into()
}
