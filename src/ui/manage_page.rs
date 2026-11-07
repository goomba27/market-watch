// SPDX-License-Identifier: MPL-2.0

use crate::app::Message;
use crate::globals;
use crate::model::ticker::Ticker;
use crate::model::watchlist::Watchlist;
use crate::ui::common::{self, ButtonState, HeaderButtons, ListMode};
use cosmic::Element;
use cosmic::iced::{Alignment, Length};
use cosmic::widget;
use std::collections::HashMap;

pub fn create_new_list_view<'a>(list_name: &'a str) -> Element<'a, Message> {
    let save_list_button = widget::container(
        widget::button::icon(widget::icon::from_name("document-save-as-symbolic"))
            .label(globals::LABEL_SAVE_LIST)
            .on_press(Message::CreateWatchlist)
            .class(common::square_button_class(ButtonState::NotSelected)),
    )
    .width(Length::Fill)
    .align_x(Alignment::End);

    let name_field = widget::text_input(globals::PLACEHOLDER_LIST_NAME, list_name)
        .size(globals::BODY_TEXT_SIZE)
        .on_input(Message::NewListNameChanged)
        .width(Length::Fill);

    let content = widget::column::with_capacity(2)
        .push(save_list_button)
        .push(name_field)
        .spacing(8);

    common::page_layout(globals::TITLE_CREATE_LIST, content, HeaderButtons::All)
}

pub fn edit_list_view<'a>(
    list: Option<&'a Watchlist>,
    ticker_input: &'a str,
    rename_input: &'a str,
    quotes: &HashMap<String, Ticker>,
) -> Element<'a, Message> {
    let Some(current) = list else {
        // No list yet
        return common::page_layout(
            globals::TITLE_EDIT_LIST,
            widget::container(
                widget::text::body(globals::EMPTY_NO_LIST).size(globals::BODY_TEXT_SIZE),
            )
            .padding([16, 0]),
            HeaderButtons::All,
        );
    };

    let list_name_textbox = widget::text_input(&current.name, rename_input)
        .size(globals::BODY_TEXT_SIZE)
        .on_input(Message::RenameInputChanged)
        .on_submit(Message::SaveListName)
        .width(Length::Fill)
        .label(globals::FIELD_CHANGE_LIST_NAME);

    let save_new_name_button =
        widget::button::icon(widget::icon::from_name("document-save-symbolic"))
            .label(globals::LABEL_SAVE_NAME)
            .on_press(Message::SaveListName(rename_input.to_string()))
            .class(common::square_button_class(ButtonState::NotSelected));

    let change_name_row = widget::row::with_capacity(2)
        .push(list_name_textbox)
        .push(save_new_name_button)
        .spacing(8)
        // the field label sits above the box, so line the button up with the bottom
        .align_y(Alignment::End);

    let symbol_input_field = widget::text_input(globals::PLACEHOLDER_SYMBOL, ticker_input)
        .size(globals::BODY_TEXT_SIZE)
        .on_input(Message::TickerInputChanged)
        .on_submit(Message::AddTicker)
        .width(Length::Fill)
        .label(globals::FIELD_NEW_SYMBOL);

    let add_button = widget::button::icon(widget::icon::from_name("list-add-symbolic"))
        .label(globals::LABEL_ADD_TO_LIST)
        .on_press(Message::AddTicker(ticker_input.to_string()))
        .class(common::square_button_class(ButtonState::NotSelected));

    let add_symbol_row = widget::row::with_capacity(2)
        .push(symbol_input_field)
        .push(add_button)
        .spacing(8)
        // the field label sits above the box, so line the button up with the bottom
        .align_y(Alignment::End);

    let body = common::generate_ticker_list(list, quotes, ListMode::Editable);

    let content = widget::column::with_capacity(3)
        .push(change_name_row)
        .push(add_symbol_row)
        .push(body)
        .spacing(24);

    common::page_layout(globals::TITLE_EDIT_LIST, content, HeaderButtons::All)
}
