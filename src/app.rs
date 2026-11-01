// SPDX-License-Identifier: MPL-2.0

use crate::config::{AppData, Config};
use crate::model::watchlist::Watchlist;
use crate::ui::{self, watchlist_page};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::platform_specific::shell::wayland::commands::popup::{
    destroy_popup, get_popup,
};
use cosmic::iced::{Limits, Subscription, futures, window::Id};
use cosmic::prelude::*;
use futures::SinkExt;

#[derive(Default)]
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: cosmic::Core,
    /// The popup id.
    popup: Option<Id>,
    /// Configuration data that persists between application runs.
    config: Config,

    /// Config handle for settings (needed to write them).
    config_handle: Option<cosmic_config::Config>,

    /// Watchlists and anything else the user creates.
    data: AppData,

    /// Config handle for app data
    data_handle: Option<cosmic_config::Config>,

    selected_list: usize,
    page: Page,
    list_name: String,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    SubscriptionChannel,
    UpdateConfig(Config),

    //Watchlist
    CreateWatchlist,
    SelectList(usize),
    NewListNameChanged(String),
    AddTicker(String),
    SetRefreshSecs(usize),

    //Navigation
    OpenPage(Page),
}

#[derive(Debug, Clone, Default)]
pub enum Page {
    #[default]
    Watchlist,
    Manage,
}

/// Create a COSMIC application from the app model
impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "com.github.goomba27.market-watch";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        // Settings: ~/.config/cosmic/<APP_ID>/v1/
        let config_handle =
            cosmic_config::Config::new(Self::APP_ID, Config::VERSION).ok();

        let config = config_handle
            .as_ref()
            .map(|h| Config::get_entry(h).unwrap_or_else(|(_errors, c)| c))
            .unwrap_or_default();

        // App data: ~/.local/share/cosmic/<APP_ID>/v1/
        let data_handle = dirs::data_dir().and_then(|dir| {
            cosmic_config::Config::with_custom_path(
                Self::APP_ID,
                AppData::VERSION,
                dir,
            )
            .ok()
        });

        let data = data_handle
            .as_ref()
            .map(|h| AppData::get_entry(h).unwrap_or_else(|(_errors, d)| d))
            .unwrap_or_default();

        let app = AppModel {
            core,
            config,
            config_handle,
            data,
            data_handle,
            ..Default::default()
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        struct MySubscription;

        Subscription::batch(vec![
            // Create a subscription which emits updates through a channel.
            Subscription::run(|| {
                cosmic::iced::stream::channel(
                    4,
                    move |mut channel: futures::channel::mpsc::Sender<_>| async move {
                        _ = channel.send(Message::SubscriptionChannel).await;

                        futures::future::pending().await
                    },
                )
            }),
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| {
                    // for why in update.errors {
                    //     tracing::error!(?why, "app config error");
                    // }

                    Message::UpdateConfig(update.config)
                }),
        ])
    }

    //update only changes states, doesnt draw anything
    fn update(
        &mut self,
        message: Self::Message,
    ) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::SubscriptionChannel => {
                // For example purposes only.
            }
            Message::UpdateConfig(config) => {
                self.config = config;
            }
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings =
                        self.core.applet.get_popup_settings(
                            self.core.main_window_id().unwrap(),
                            new_id,
                            None,
                            None,
                            None,
                        );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(600.0)
                        .min_width(600.0)
                        .min_height(600.0)
                        .max_height(1080.0);
                    get_popup(popup_settings)
                };
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::CreateWatchlist => {
                let name = self.list_name.trim().to_string();
                if name.is_empty() {
                    return Task::none();
                }
                self.data.watchlists.push(Watchlist::new(name));
                self.save_data();
                self.selected_list = self.data.watchlists.len() - 1;
                self.list_name.clear();
                self.page = Page::Watchlist;
            }
            Message::SelectList(i) => self.selected_list = i,
            Message::OpenPage(page) => self.page = page,
            Message::NewListNameChanged(name) => self.list_name = name,
            Message::AddTicker(symbol) => {
                let symbol = symbol.trim().to_uppercase();
                if let Some(list) =
                    self.data.watchlists.get_mut(self.selected_list)
                {
                    if !symbol.is_empty() && !list.tickers.contains(&symbol) {
                        list.tickers.push(symbol);
                        self.save_data();
                    }
                }
            }
            Message::SetRefreshSecs(secs) => {
                if let Some(handle) = &self.config_handle {
                    let mut settings = self.config.settings.clone();
                    settings.refresh_secs = secs;
                    if let Err(err) = self.config.set_settings(handle, settings)
                    {
                        eprintln!("failed to save settings: {err}");
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        self.core
            .applet
            .icon_button("display-symbolic")
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let page = match self.page {
            Page::Manage => ui::manage_page::view(&self.list_name),
            Page::Watchlist => ui::watchlist_page::view(
                &self.data.watchlists,
                self.selected_list,
            ),
        };

        self.core
            .applet
            .popup_container(page)
            .limits(
                Limits::NONE
                    .min_width(600.0)
                    .max_width(600.0)
                    .min_height(600.0)
                    .max_height(600.0),
            )
            .into()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

impl AppModel {
    fn save_data(&self) {
        if let Some(handle) = &self.data_handle {
            if let Err(err) = self.data.write_entry(handle) {
                eprintln!("failed to save the app data: {err}");
            }
        }
    }
}
