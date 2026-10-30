// SPDX-License-Identifier: MPL-2.0

use crate::applet::*;
use crate::config::Config;
use crate::palette;
use crate::watchlist::Watchlist;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::platform_specific::shell::wayland::commands::popup::{
    destroy_popup, get_popup,
};
use cosmic::iced::{Alignment, Length};
use cosmic::iced::{Background, Border};
use cosmic::iced::{Limits, Subscription, futures, window::Id};
use cosmic::prelude::*;
use cosmic::theme;
use cosmic::widget;
use futures::SinkExt;

const TAB_WIDTH: f32 = 150.0;
/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
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
    AddWatchlist,
    SelectList(usize),
    SelectPopup(usize),
}

/// Create a COSMIC application from the app model
impl cosmic::Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "com.github.goomba27.market-watch";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        // Construct the app model with the runtime's core.
        let app = AppModel {
            example_text: String::from("Init Values"),
            core,
            selected_list: 0,
            config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                .map(|context| match Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((_errors, config)) => {
                        // for why in errors {
                        //     tracing::error!(%why, "error loading app config");
                        // }

                        config
                    }
                })
                .unwrap_or_default(),
            watchlists: vec![
                Watchlist {
                    index: 0,
                    name: "List 1".to_string(),
                    tickers: vec!["NVDA".to_string(), "AMD".to_string()],
                    count: 2,
                },
                Watchlist {
                    index: 1,
                    name: "List 2".to_string(),
                    tickers: vec!["NVDA".to_string(), "AMD".to_string()],
                    count: 2,
                },
                Watchlist {
                    index: 2,
                    name: "List 3".to_string(),
                    tickers: vec![
                        "NVDA".to_string(),
                        "AMD".to_string(),
                        "GOOG".to_string(),
                    ],
                    count: 2,
                },
                Watchlist {
                    index: 3,
                    name: "List 4".to_string(),
                    tickers: vec![
                        "NVDA".to_string(),
                        "AMD".to_string(),
                        "GOOG".to_string(),
                    ],
                    count: 2,
                },
            ],
            ..Default::default()
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-lived async tasks running in the background which
    /// emit messages to the application through a channel. They may be conditionally
    /// activated by selectively appending to the subscription batch, and will
    /// continue to execute for the duration that they remain in the batch.
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

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime. The application will not exit until all
    /// tasks are finished.
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
            Message::AddWatchlist => todo!(),
            Message::SelectList(_) => todo!(),
            Message::SelectPopup(_) => {}
        }
        Task::none()
    }

    /// Describes the interface based on the current state of the application model.
    ///
    /// The applet's button in the panel will be drawn using the main view method.
    /// This view should emit messages to toggle the applet's popup window, which will
    /// be drawn using the `view_window` method.
    fn view(&self) -> Element<'_, Self::Message> {
        self.core
            .applet
            .icon_button("display-symbolic")
            .on_press(Message::TogglePopup)
            .into()
    }

    /// The applet's popup window will be drawn using this view method. If there are
    /// multiple poups, you may match the id parameter to determine which popup to
    /// create a view for.
    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        // let content_list = widget::list_column()
        //     .add(widget::text(&self.example_text))
        //     .add(widget::settings::item(
        //         fl!("example-row"),
        //         widget::toggler(self.example_row).on_toggle(Message::ToggleExampleRow),
        //     ));

        let title = widget::container(
             widget::text::title2("Market Watch"))
            .padding(8)
            .width(Length::Fill)
            .align_x(Alignment::Center);

        let tab_row = widget::row::with_children(
            self.watchlists
                .iter()
                .enumerate()
                .map(|(i, l)| {
                    widget::container(
                        watchlist_buttons(
                            &l.name,
                            l.count,
                            i == self.selected_list,
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

        // Column 2: New List button, pinned to the right.
        let header = widget::row::with_capacity(2)
            .push(scroller)
            .push(new_list_button())
            .spacing(8)
            .align_y(Alignment::Start); //start at the top to match the others

        let tabs = widget::container(header)
            .padding(8)
            .width(Length::Fill)
            
            .class(theme::Container::custom(|_| {
                cosmic::iced::widget::container::Style {
                    background: Some(Background::Color(
                        palette::SURFACE_CONTAINER_LOW,
                    )),
                    border: Border {
                        radius: 0.0.into(),
                        width: 1.0,
                        color: palette::OUTLINE_VARIANT,
                    },
                    ..Default::default()
                }
            }));

        //Set to fill for height
        let content = widget::column::with_capacity(2)
            .push(title)
            .push(tabs)
            .height(Length::Fixed(600.0));

        self.core
            .applet
            .popup_container(content)
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
