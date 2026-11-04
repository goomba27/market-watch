// SPDX-License-Identifier: MPL-2.0

use crate::app::Message;
use crate::config::Settings;
use crate::globals;
use crate::ui::common;
use cosmic::Element;
use cosmic::widget;

pub fn settings_page_view(settings: &Settings) -> Element<'_, Message> {
    //------ Refresh: one button per choice, the current one highlighted
    let mut buttons = widget::row::with_capacity(globals::REFRESH_CHOICES.len()).spacing(8);
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

    common::page_layout("Settings", refresh_section)
}
