use crate::app::Message;
use crate::app::Page;
use crate::model::watchlist::Watchlist;

use cosmic::Element;
use cosmic::iced::{Alignment, Length};
use cosmic::widget;

const TAB_WIDTH: f32 = 150.0;

pub fn view(
    watchlists: &[Watchlist],
    selected_list: usize,
) -> Element<'_, Message> {
    let selected = watchlists.get(selected_list);

    let edit_list_button =
        widget::button::icon(widget::icon::from_name("edit-symbolic"))
            .label("Modify List")
            .on_press(Message::OpenPage(Page::EditList));

    let new_list_button =
        widget::button::icon(widget::icon::from_name("list-add-symbolic"))
            .label("Create List")
            .on_press(Message::OpenPage(Page::Create));

    let action_row = widget::row::with_capacity(2)
        .push(edit_list_button)
        .push(new_list_button)
        .spacing(8);

    let actions_container = widget::container(action_row)
        .width(Length::Fill)
        .align_x(Alignment::End)
        .padding([0, 8]);

    let list: Element<Message> = match selected {
        Some(w) if !w.tickers.is_empty() => w
            .tickers
            .iter()
            .fold(widget::list_column(), |row_acc, t| {
                row_acc.add(ticker_row(t))
            })
            .into_element(),
        _ => widget::text::body("No ticker yet").into(),
    };

    let body = widget::container(widget::scrollable::vertical(list))
        .padding(8)
        .width(Length::Fill)
        .height(Length::Fill);

    let title = widget::container(widget::text::title2("Market Watch"))
        .padding(8)
        .width(Length::Fill)
        .align_x(Alignment::Center);

    let tab_row = widget::row::with_children(
        watchlists
            .iter()
            .enumerate()
            .map(|(i, l)| {
                widget::container(watchlist_button(
                    &l.name,
                    l.tickers.len(),
                    Message::SelectList(i),
                ))
                .width(Length::Fixed(TAB_WIDTH))
                .into()
            })
            .collect::<Vec<_>>(),
    )
    .spacing(8)
    .padding([0, 0, 15, 0]) //handles scrollbar overlap
    .align_y(Alignment::Center);

    let scroller = widget::scrollable::horizontal(tab_row).width(Length::Fill);

    let header = widget::row::with_capacity(2)
        .push(scroller)
        .spacing(8)
        .align_y(Alignment::Start);

    let tabs = widget::container(header).padding(8).width(Length::Fill);

    let content = widget::column::with_capacity(4)
        .push(title)
        .push(actions_container)
        .push(tabs)
        .push(body) // new
        .height(Length::Fixed(600.0));

    content.into()
}

pub fn watchlist_button(
    name: &str,
    count: usize,
    on_press: Message,
) -> Element<'_, Message> {
    // label: wraps instead of forcing the button wider
    let label = widget::text::body(name).width(Length::Fill);

    let badge = widget::container(widget::text::caption(count.to_string()))
        .padding([1, 6]);

    // button: share the row equally
    widget::button::custom(
        widget::row::with_capacity(2)
            .push(label)
            .push(badge)
            .spacing(8)
            .align_y(Alignment::Center)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fixed(48.0))
    .padding([8, 14])
    .on_press(on_press)
    .into()
}

pub fn ticker_row(symbol: &str) -> Element<'_, Message> {
    widget::row::with_capacity(2)
        .push(widget::text::body(symbol).width(Length::Fill))
        .push(widget::text("--"))
        .align_y(Alignment::Center)
        .into()
}
