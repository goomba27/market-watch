// SPDX-License-Identifier: MPL-2.0

use crate::config::Config;
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
    /// Example row toggler.
    example_row: bool,
    example_text: String,
    watchlists: Vec<Watchlist>,
    selected_list: usize,
    page: Page,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    SubscriptionChannel,
    UpdateConfig(Config),
    ToggleExampleRow(bool),

    //Watchlist
    CreateWatchlist,
    SelectList(usize),

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
        let app = AppModel {
            example_text: String::from("Init Values"),
            core,
            selected_list: 0,
            config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                .map(|context| match Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((_errors, config)) => config,
                })
                .unwrap_or_default(),
            watchlists: vec![
                Watchlist {
                    name: "List 1".to_string(),
                    tickers: vec!["NVDA".to_string()],
                    count: 1,
                },
                Watchlist {
                    name: "List 2".to_string(),
                    tickers: vec!["NVDA".to_string(), "AMD".to_string()],
                    count: 2,
                },
                Watchlist {
                    name: "List 3".to_string(),
                    tickers: vec![
                        "NVDA".to_string(),
                        "AMD".to_string(),
                        "GOOG".to_string(),
                    ],
                    count: 3,
                },
                Watchlist {
                    name: "List 4".to_string(),
                    tickers: vec![
                        "NVDA".to_string(),
                        "AMD".to_string(),
                        "GOOG".to_string(),
                        "APPL".to_string(),
                    ],
                    count: 4,
                },
            ],
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
            Message::ToggleExampleRow(toggled) => {
                self.example_row = toggled;
                self.example_text = format!("Toggle is {}", toggled)
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
            Message::CreateWatchlist => todo!(),
            Message::SelectList(i) => self.selected_list = i,
            Message::OpenPage(page) => self.page = page,
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
            Page::Manage => ui::manage_page::view(),
            Page::Watchlist => {
                ui::watchlist_page::view(&self.watchlists, self.selected_list)
            }
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
