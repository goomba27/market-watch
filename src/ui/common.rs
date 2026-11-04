// SPDX-License-Identifier: MPL-2.0

use crate::globals;
use cosmic::Element;
use cosmic::iced::{Alignment, Length};
use cosmic::widget;

use crate::app::{Message, Page};
use crate::model::watchlist::Watchlist;

//TODO: add param to not display black on watchlist page show_back: bool
//TODO: add param to not display settings on settings page show_settings: bool
//TODO: title is off center

pub fn build_header<'a>(page_title: &'a str) -> Element<'a, Message> {
    let title = widget::container(widget::text::title2(page_title))
        .width(Length::Fill) //this two are needed for centered text with the container
        .align_x(Alignment::Center);

    let settings_button =
        widget::button::icon(widget::icon::from_name("preferences-system-symbolic"))
            .label(globals::LABEL_SETTINGS)
            .on_press(Message::OpenPage(Page::Settings));

    widget::row::with_capacity(3)
        .push(back_button())
        .push(title)
        .push(settings_button)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .spacing(12)
        .padding(globals::INSET)
        .into()
}

// for now all pages back button go to Watchlist
pub fn back_button<'a>() -> Element<'a, Message> {
    widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
        .label(globals::LABEL_BACK)
        .on_press(Message::OpenPage(Page::Watchlist))
        .into()
}

#[derive(Clone, Copy)]
pub enum ListMode {
    ReadOnly,
    Editable,
}

pub fn generate_ticker_list<'a>(
    selected: Option<&'a Watchlist>,
    mode: ListMode,
) -> Element<'a, Message> {
    let list: Element<Message> = match selected {
        Some(w) if !w.tickers.is_empty() => {
            let mut row_acc = widget::list_column();
            for t in &w.tickers {
                row_acc = match mode {
                    ListMode::ReadOnly => row_acc.add(ticker_row(t)),
                    ListMode::Editable => row_acc.add(editable_row(t)),
                };
            }
            row_acc.into_element()
        }
        _ => widget::text::body(globals::EMPTY_TICKER_LIST).into(),
    };

    widget::container(widget::scrollable::vertical(list))
        .padding([8, 0])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn ticker_row(symbol: &str) -> Element<'_, Message> {
    widget::row::with_capacity(2)
        .push(widget::text::body(symbol).width(Length::Fill))
        .push(widget::text(globals::NO_PRICE))
        .align_y(Alignment::Center)
        .into()
}

fn editable_row<'a>(ticker: &'a str) -> Element<'a, Message> {
    let trash = widget::button::icon(widget::icon::from_name("edit-delete-symbolic"))
        .extra_small()
        .on_press(Message::RemoveTicker(ticker.to_string()));

    let trash_container = widget::container(trash)
        .width(Length::Fill)
        .align_x(Alignment::End);

    let text = widget::text::body(ticker);

    widget::row::with_capacity(2)
        .push(text)
        .push(trash_container)
        // trash can is not the same height as the text, so center them
        .align_y(Alignment::Center)
        .spacing(8)
        .into()
}

/// Header + content with the same side inset on every page.
pub fn page_layout<'a>(
    title: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let body = widget::container(content)
        .padding([0, globals::INSET])
        .width(Length::Fill)
        .height(Length::Fill);

    widget::column::with_capacity(2)
        .push(build_header(title))
        .push(body)
        .width(Length::Fill)
        .height(Length::Fixed(globals::POPUP_HEIGHT))
        .into()
}
