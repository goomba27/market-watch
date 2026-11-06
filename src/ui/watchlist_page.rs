// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;

use crate::app::{Message, Page};
use crate::globals;
use crate::model::ticker::Ticker;
use crate::model::watchlist::Watchlist;
use crate::ui::common::{self, ListMode};

use cosmic::Element;
use cosmic::iced::{Alignment, Length};
use cosmic::theme;
use cosmic::widget;
use cosmic::widget::button::Style;

pub fn view<'a>(
    watchlists: &'a [Watchlist],
    selected_list: usize,
    quotes: &HashMap<String, Ticker>,
) -> Element<'a, Message> {
    let selected = watchlists.get(selected_list);

    let edit_list_button = widget::button::icon(widget::icon::from_name("edit-symbolic"))
        .label(globals::LABEL_MODIFY_LIST)
        .on_press(Message::OpenPage(Page::EditList));

    let new_list_button = widget::button::icon(widget::icon::from_name("list-add-symbolic"))
        .label(globals::LABEL_CREATE_LIST)
        .on_press(Message::OpenPage(Page::Create));

    let action_row = widget::row::with_capacity(2)
        .push(edit_list_button)
        .push(new_list_button)
        .spacing(8);

    let actions_container = widget::container(action_row)
        .width(Length::Fill)
        .align_x(Alignment::End);

    let body = common::generate_ticker_list(selected, ListMode::ReadOnly, quotes);

    let tab_row = widget::row::with_children(
        watchlists
            .iter()
            .enumerate()
            .map(|(i, l)| {
                widget::container(watchlist_button(
                    &l.name,
                    l.tickers.len(),
                    i == selected_list,
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
        .padding([8, 0])
        .width(Length::Fill);

    let content = widget::column::with_capacity(3)
        .push(actions_container)
        .push(tabs)
        .push(body);

    common::page_layout(globals::TITLE_WATCHLIST, content)
}

pub fn watchlist_button(
    name: &str,
    count: usize,
    selected: bool,
    on_press: Message,
) -> Element<'_, Message> {
    // label: wraps instead of forcing the button wider
    let label = widget::text::body(name).width(Length::Fill);

    let badge = widget::container(widget::text::caption(count.to_string())).padding([1, 6]);

    // button: share the row equally
    widget::button::custom(
        widget::row::with_capacity(2)
            .push(label)
            .push(badge)
            .spacing(8)
            .align_y(Alignment::Center)
            .height(Length::Fill),
    )
    .class(tab_class(selected))
    .width(Length::Fill)
    .height(Length::Fixed(48.0))
    .padding([8, 14])
    .on_press(on_press)
    .into()
}

/// Button class for a watchlist tab: the stock `Suggested` (selected) or `Standard` look, but
/// with the theme's small corner radius instead of the default pill shape.
fn tab_class(selected: bool) -> theme::Button {
    use cosmic::widget::button::Catalog;

    // `theme::Button` is not `Clone`, so each closure below rebuilds its own base class.
    fn base(selected: bool) -> theme::Button {
        if selected {
            theme::Button::Suggested
        } else {
            theme::Button::Standard
        }
    }

    fn square(mut style: Style, theme: &cosmic::Theme) -> Style {
        style.border_radius = theme.cosmic().corner_radii.radius_xs.into();
        style
    }

    theme::Button::Custom {
        active: Box::new(move |focused, theme| {
            square(theme.active(focused, false, &base(selected)), theme)
        }),
        disabled: Box::new(move |theme| square(theme.disabled(&base(selected)), theme)),
        hovered: Box::new(move |focused, theme| {
            square(theme.hovered(focused, false, &base(selected)), theme)
        }),
        pressed: Box::new(move |focused, theme| {
            square(theme.pressed(focused, false, &base(selected)), theme)
        }),
    }
}
