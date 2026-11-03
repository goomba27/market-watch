// SPDX-License-Identifier: MPL-2.0

use crate::app::Message;
use crate::config::Settings;
use crate::globals;
use crate::ui::common::page_layout;
use cosmic::Element;
use cosmic::iced::{Alignment, Length};
use cosmic::widget;

pub fn settings_page_view<'a>(
    settings: &'a Settings,
    api_key_input: &'a str,
    show_api_key: bool,
) -> Element<'a, Message> {
    //-------  API key: a hidden text field plus a Save button
    let key_field = widget::secure_input(
        "Paste your Massive API key",
        api_key_input,
        Some(Message::ToggleShowApiKey),
        !show_api_key,
    )
    .on_input(Message::ApiKeyInputChanged)
    .on_submit(|_| Message::SaveApiKey)
    .width(Length::Fill);

    let save_button = widget::button::suggested("Save").on_press_maybe(
        (api_key_input.trim() != settings.api_key)
            .then_some(Message::SaveApiKey),
    );

    let key_row = widget::row::with_capacity(2)
        .push(key_field)
        .push(save_button)
        .spacing(8)
        .align_y(Alignment::Center);

    let api_section = widget::settings::section()
        .title("Massive API")
        .add(key_row);
    //-------  ------------------------------------

    //------ Refresh: one button per choice, the current one highlighted
    let mut buttons =
        widget::row::with_capacity(globals::REFRESH_CHOICES.len()).spacing(8);
    for (secs, label) in globals::REFRESH_CHOICES {
        let button = if settings.refresh_secs == secs {
            widget::button::suggested(label)
        } else {
            widget::button::standard(label)
        };
        buttons = buttons.push(button.on_press(Message::SetRefreshSecs(secs)));
    }

    let refresh_section = widget::settings::section()
        .title("Refresh")
        .add(widget::settings::item("Update prices every", buttons));
    //------------------------------------------

    let content = widget::column::with_capacity(2)
        .push(api_section)
        .push(refresh_section)
        .spacing(12);

    page_layout("Settings", content)
}
