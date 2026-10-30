use cosmic::iced::Background;
use cosmic::widget::button::Style as ButtonStyle;
use cosmic::{widget,theme};
use cosmic::iced::{Alignment, Length};
use cosmic::Element;
use crate::app::Message;
use crate::palette;


pub fn header_button(
    name: &str,
    count: u32,
    selected: bool,
    on_press: Message,) -> Element<'_, Message> {

        // label: let the text take the spare space so it wraps instead of forcing the button wider
        let label = widget::text::body(name)
            .width(Length::Fill)
            .class(theme::Text::Color(if selected {
                palette::PRIMARY
            } else {
                palette::ON_SURFACE
            }));

   let badge = widget::container(widget::text::caption(count.to_string()))
           .padding([1, 6])
           .class(theme::Container::custom(|_| {
               cosmic::iced::widget::container::Style {
                   background: Some(cosmic::iced::Background::Color(
                       palette::SURFACE_CONTAINER_HIGHEST,
                   )),
                   border: cosmic::iced::Border {
                       radius: 0.0.into(), ..Default::default()
                   },
                   ..Default::default()
               }
           }));

   // button: share the row equally
   widget::button::custom(
       widget::row::with_capacity(2)
           .push(label)
           .push(badge)
           .spacing(8)
           .align_y(Alignment::Center)
           .height(Length::Fill),
   )
   .width(Length::Fill)          // <-- add
   .height(Length::Fixed(48.0))
       .padding([8, 14])
   .class(square_tab(selected))
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
    .class(square_tab(false))
    .on_press(Message::AddWatchlist)
    .into()
}
pub fn  ticker_list<'a>() -> Element<'a, Message>
{
   let text = widget::text::text("Ticker");

    text.into()
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
