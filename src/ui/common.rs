// SPDX-License-Identifier: MPL-2.0

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::app::{Message, Page};
use crate::globals;
use crate::model::ticker::Ticker;
use crate::model::watchlist::Watchlist;
use cosmic::Element;
use cosmic::iced::border::Radius;
use cosmic::iced::{Alignment, Length};
use cosmic::theme;
use cosmic::widget;
use cosmic::widget::button::Catalog;
use cosmic::widget::button::Style;

use yfinance_rs::Decimal;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonState {
    Selected,
    NotSelected,
}

pub fn generate_ticker_list<'a>(
    selected: Option<&'a Watchlist>,
    quotes: &HashMap<String, Ticker>,
    mode: ListMode,
) -> Element<'a, Message> {
    let list: Element<Message> = match selected {
        Some(w) if !w.tickers.is_empty() => {
            let mut row_acc = widget::list_column();
            for t in &w.tickers {
                row_acc = match mode {
                    ListMode::ReadOnly => row_acc.add(ticker_row(t, quotes.get(t))),
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

fn ticker_row<'a>(symbol: &'a str, quote: Option<&Ticker>) -> Element<'a, Message> {
    let price_area = match quote {
        Some(q) => {
            let style = match q.change_percent.cmp(&Decimal::from(0)) {
                Ordering::Greater => theme::Text::Color(globals::GAIN_COLOR),
                Ordering::Less => theme::Text::Color(globals::LOSS_COLOR),
                Ordering::Equal => theme::Text::Color(globals::DEFAULT_TEXT_COLOR),
            };

            let price = widget::text(format!("${}", q.price.round_dp(2))).size(18);

            let change = widget::text(format!("{}%", q.change_percent.round_dp(2)))
                .class(style)
                .size(16);

            widget::column::with_capacity(2)
                .push(price)
                .push(change)
                .align_x(Alignment::End)
        }
        None => widget::column::with_capacity(2)
            .push(widget::text(globals::NO_PRICE).size(18))
            .push(widget::text(globals::NO_PRICE).size(16))
            .align_x(Alignment::End),
    };

    widget::row::with_capacity(2)
        .push(widget::text::body(symbol).width(Length::Fill))
        .push(price_area)
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

fn square_rounded_corners(mut style: Style) -> Style {
    style.border_radius = Radius::from(globals::TAB_CORNER_RADIUS);
    style
}

fn base(button_state: ButtonState) -> theme::Button {
    match button_state {
        ButtonState::Selected => theme::Button::Suggested,
        ButtonState::NotSelected => theme::Button::Standard,
    }
}

/// The stock button look (accent when selected), with squarer rounded corners.
pub fn tab_style(button_state: ButtonState) -> theme::Button {
    theme::Button::Custom {
        active: Box::new(move |focused, t| {
            square_rounded_corners(t.active(focused, false, &base(button_state)))
        }),
        disabled: Box::new(move |t| square_rounded_corners(t.disabled(&base(button_state)))),
        hovered: Box::new(move |focused, t| {
            square_rounded_corners(t.hovered(focused, false, &base(button_state)))
        }),
        pressed: Box::new(move |focused, t| {
            square_rounded_corners(t.pressed(focused, false, &base(button_state)))
        }),
    }
}
