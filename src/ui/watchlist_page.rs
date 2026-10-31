use crate::app::Message;
use crate::app::Page;
use crate::model::watchlist::Watchlist;
use crate::palette;
use cosmic::Element;
use cosmic::iced::Background;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::button::Style as ButtonStyle;
use cosmic::{theme, widget};

const TAB_WIDTH: f32 = 150.0;

pub fn view(watchlists: &[Watchlist], selected_list: usize) -> Element<'_, Message> {
    let selected = watchlists.get(selected_list);

    let list: Element<Message> = match selected {
        Some(w) if !w.tickers.is_empty() => w
            .tickers
            .iter()
            .fold(widget::list_column(), |col, t| col.add(ticker_row(t)))
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
                widget::container(watchlist_buttons(
                    &l.name,
                    l.count,
                    i == selected_list,
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

    let new_list_button = widget::container(new_list_button())
        .width(Length::Fill) //fill fully then allign to the right/end
        .align_x(Alignment::End)
        .padding([0, 8]);

    let tabs = widget::container(header).padding(8).width(Length::Fill);

    let content = widget::column::with_capacity(3)
        .push(title)
        .push(new_list_button)
        .push(tabs)
        .push(body) // new
        .height(Length::Fixed(600.0));

    content.into()
}

pub fn watchlist_buttons(name: &str, count: usize, selected: bool, on_press: Message,) -> Element<'_, Message> {
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
    .width(Length::Fill)
    .height(Length::Fixed(48.0))
    .padding([8, 14])
    .on_press(on_press)
    .into()
}

pub fn new_list_button<'a>() -> Element<'a, Message> {
    widget::button::custom(
        widget::row::with_capacity(2)
            .push(widget::icon::from_name("list-add-symbolic").size(14))
            .push(widget::text::caption("New\nList"))
            .spacing(6)
            .height(Length::Fill)
            .align_y(Alignment::Center),
    )
    .padding([8, 12])
    .height(Length::Fixed(48.0))
    // .class(square_tab(false))
    .on_press(Message::OpenPage(Page::Manage))
    .into()
}

pub fn ticker_row(symbol: &str) -> Element<'_, Message> {
    widget::row::with_capacity(2)
        .push(widget::text::body(symbol).width(Length::Fill))
        .push(widget::text("--"))
        .align_y(Alignment::Center)
        .into()
}

/// Square tab style. `selected` tints the background and uses the accent color.
fn square_tab(selected: bool) -> theme::Button {
    fn style(_theme: &cosmic::Theme, selected: bool, hover: bool) -> ButtonStyle {
        let mut s = ButtonStyle::new();

        let bg = if selected {
            palette::SURFACE_CONTAINER
        } else if hover {
            palette::SURFACE_CONTAINER_HIGH
        } else {
            palette::with_alpha(palette::SURFACE_CONTAINER, 0.0)
        };
        s.background = Some(Background::Color(bg));
        s.text_color = Some(if selected {
            palette::PRIMARY
        } else {
            palette::ON_SURFACE
        });

        s.border_width = 1.0;
        s.border_color = if selected {
            palette::PRIMARY
        } else {
            palette::OUTLINE_VARIANT
        };
        s
    }

    theme::Button::Custom {
        active: Box::new(move |_focused, t| style(t, selected, false)),
        hovered: Box::new(move |_focused, t| style(t, selected, true)),
        pressed: Box::new(move |_focused, t| style(t, selected, true)),
        disabled: Box::new(move |t| style(t, selected, false)),
    }
}
