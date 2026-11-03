use crate::app::Message;
use crate::config::Settings;
use crate::globals;
use crate::ui::common::build_header;
use cosmic::Element;
use cosmic::iced::Length;
use cosmic::widget;

pub fn settings_page_view<'a>(settings: &'a Settings) -> Element<'a, Message> {
    let header = build_header("Settings");
    let content = widget::column::with_capacity(1)
        .push(header)
        .width(Length::Fill)
        .height(Length::Fixed(globals::POPUP_HEIGHT));

    content.into()
}
