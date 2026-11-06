// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;

use crate::app::{Message, Page};
use crate::globals;
use crate::model::ticker::Ticker;
use crate::model::watchlist::Watchlist;
use crate::ui::common::{self, ButtonState, HeaderButtons, ListMode};

use cosmic::Element;
use cosmic::iced::{Alignment, Length};
use cosmic::widget;

pub fn view<'a>(
    watchlists: &'a [Watchlist],
    selected_list: usize,
    quotes: &HashMap<String, Ticker>,
) -> Element<'a, Message> {
    let selected = watchlists.get(selected_list);

    let edit_list_button = widget::button::icon(widget::icon::from_name("edit-symbolic"))
        .label(globals::LABEL_MODIFY_LIST)
        .on_press(Message::OpenPage(Page::EditList))
        .class(common::square_button_class(ButtonState::NotSelected));

    let new_list_button = widget::button::icon(widget::icon::from_name("list-add-symbolic"))
        .label(globals::LABEL_CREATE_LIST)
        .on_press(Message::OpenPage(Page::Create))
        .class(common::square_button_class(ButtonState::NotSelected));

    let action_row = widget::row::with_capacity(2)
        .push(edit_list_button)
        .push(new_list_button)
        .spacing(8);

    let actions_container = widget::container(action_row)
        .width(Length::Fill)
        .align_x(Alignment::End);

    let tab_row = widget::row::with_children(
        watchlists
            .iter()
            .enumerate()
            .map(|(i, l)| {
                widget::container(watchlist_button(
                    &l.name,
                    l.tickers.len(),
                    if i == selected_list {
                        ButtonState::Selected
                    } else {
                        ButtonState::NotSelected
                    },
                    Message::SelectList(i),
                ))
                .width(Length::Fixed(globals::TAB_WIDTH))
                .into()
            })
            .collect::<Vec<_>>(),
    )
    .spacing(8)
    .padding([0, 0, 15, 0]) //handles scrollbar overlap
    .align_y(Alignment::Center);

    let scroller = widget::scrollable::horizontal(tab_row).width(Length::Fill);

    let watchlist_tabs = widget::row::with_capacity(2)
        .push(scroller)
        .spacing(8)
        .align_y(Alignment::Start);

    let tabs = widget::container(watchlist_tabs)
        .padding([24, 0])
        .width(Length::Fill);

    let body = common::generate_ticker_list(selected, quotes, ListMode::ReadOnly);

    let content = widget::column::with_capacity(3)
        .push(actions_container)
        .push(tabs)
        .push(body);

    common::page_layout(globals::TITLE_WATCHLIST, content, HeaderButtons::Settings)
}

pub fn watchlist_button(
    name: &str,
    count: usize,
    button_state: ButtonState,
    on_press: Message,
) -> Element<'_, Message> {
    // label: wraps instead of forcing the button wider
    let label = widget::text::body(name)
        .size(globals::BODY_TEXT_SIZE)
        .width(Length::Fill);

    let badge = widget::container(
        widget::text::caption(count.to_string()).size(globals::CAPTION_TEXT_SIZE),
    )
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
    .class(common::square_button_class(button_state))
    .width(Length::Fill)
    .height(Length::Fixed(48.0))
    .padding([8, 14])
    .on_press(on_press)
    .into()
}
