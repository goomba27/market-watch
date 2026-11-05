// SPDX-License-Identifier: MPL-2.0

use crate::config::{AppData, Config, Settings};
use crate::globals;
use crate::model::ticker::Ticker;
use crate::model::watchlist::Watchlist;
use crate::provider;
use crate::ui;
use cosmic::cosmic_config::{self, ConfigGet, CosmicConfigEntry};
use cosmic::iced::platform_specific::shell::wayland::commands::popup;
use cosmic::iced::window::Id;
use cosmic::iced::{Limits, Subscription, time};
use cosmic::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use yfinance_rs::YfClient;

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

    /// Latest quote for each symbol, from the last successful fetch.
    quotes: HashMap<String, Ticker>,

    /// One Yahoo client, reused so its login cookie is reused.
    yf_client: YfClient,

    /// Config handle for app data
    data_handle: Option<cosmic_config::Config>,

    selected_list: usize,
    page: Page,
    list_name: String,
    ticker_input: String,

    /// True while a quote fetch is running, so ticks don't stack requests.
    in_flight: bool,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    UpdateConfig(Config),

    //Watchlist
    CreateWatchlist,
    SelectList(usize),
    NewListNameChanged(String),
    TickerInputChanged(String),
    AddTicker(String),
    RemoveTicker(String),

    //Prices
    QuotesFetched(Result<Vec<Ticker>, String>),
    Tick,

    //Settings
    SetRefreshSecs(usize),

    //Navigation
    OpenPage(Page),
}

#[derive(Debug, Clone, Default)]
pub enum Page {
    #[default]
    Watchlist,
    Create,
    EditList,
    Settings,
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
        let config_handle = cosmic_config::Config::new(Self::APP_ID, Config::VERSION).ok();

        let config = match &config_handle {
            Some(h) => match Config::get_entry(h) {
                Ok(config) => {
                    //first run: nothing on disk, write defaults
                    if matches!(
                        h.get_local::<Settings>("settings"),
                        Err(cosmic_config::Error::NotFound)
                    ) && let Err(err) = config.write_entry(h)
                    {
                        eprintln!("failed to write default settings: {err}");
                    }
                    config
                }
                // File exists but can't be read: use defaults, don't overwrite it.
                Err((errors, config)) => {
                    eprintln!("failed to load settings: {errors:?}");
                    config
                }
            },
            None => Config::default(),
        };

        // App data: ~/.local/share/cosmic/<APP_ID>/v1/
        let data_handle = dirs::data_dir().and_then(|dir| {
            cosmic_config::Config::with_custom_path(Self::APP_ID, AppData::VERSION, dir).ok()
        });

        let data = data_handle
            .as_ref()
            .map(|h| AppData::get_entry(h).unwrap_or_else(|(_errors, d)| d))
            .unwrap_or_default();

        let mut app = AppModel {
            core,
            config,
            config_handle,
            data,
            data_handle,
            ..Default::default()
        };

        let fetch = app.fetch_all_quotes();
        (app, fetch)
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let refresh_secs = self.config.settings.refresh_secs as u64;
        let refresh = if refresh_secs == 0 {
            //0 would panic treat it as off
            Subscription::none()
        } else {
            time::every(Duration::from_secs(refresh_secs)).map(|_| Message::Tick)
        };

        Subscription::batch(vec![
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config)),
            refresh,
        ])
    }

    //update only changes states, doesnt draw anything
    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::UpdateConfig(config) => {
                self.config = config;
            }
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    popup::destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(globals::POPUP_WIDTH)
                        .min_width(globals::POPUP_WIDTH)
                        .min_height(globals::POPUP_HEIGHT)
                        .max_height(globals::POPUP_HEIGHT);
                    popup::get_popup(popup_settings)
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
            Message::TickerInputChanged(text) => self.ticker_input = text,
            Message::AddTicker(symbol) => {
                let symbol = symbol.trim().to_uppercase();
                if !symbol.is_empty()
                    && let Some(list) = self.data.watchlists.get_mut(self.selected_list)
                    && !list.tickers.contains(&symbol)
                {
                    list.tickers.push(symbol);
                    self.save_data();
                    self.ticker_input.clear();
                }
            }
            Message::RemoveTicker(ticker) => {
                if let Some(list) = self.data.watchlists.get_mut(self.selected_list) {
                    let starting_len = list.tickers.len();
                    list.tickers.retain(|t| *t != ticker);

                    if starting_len != list.tickers.len() {
                        self.save_data();
                    } else {
                        eprintln!("RemoveTicker({ticker}): symbol was not found");
                    }
                } else {
                    eprintln!(
                        "RemoveTicker({ticker}): no list at selected index {}",
                        self.selected_list
                    );
                }
            }
            Message::SetRefreshSecs(secs) => {
                let mut settings = self.config.settings.clone();
                eprintln!("refresh_secs changed to {secs}");
                settings.refresh_secs = secs;
                self.save_settings(settings);
            }
            Message::QuotesFetched(result) => {
                self.in_flight = false;

                eprintln!("Quotes Fetched: {result:?}");
                match result {
                    Ok(tickers) => {
                        for t in tickers {
                            self.quotes.insert(t.symbol.clone(), t);
                        }
                    }
                    Err(err) => {
                        eprintln!("failed to fetch quotes: {err}");
                    }
                }
            }
            Message::Tick => {
                eprintln!("tick, in_flight={}", self.in_flight);
                if self.in_flight {
                    return Task::none();
                }
                return self.fetch_all_quotes();
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
            Page::Create => ui::manage_page::create_new_list_view(&self.list_name),
            Page::Watchlist => {
                ui::watchlist_page::view(&self.data.watchlists, self.selected_list, &self.quotes)
            }
            Page::EditList => ui::manage_page::edit_list_view(
                self.data.watchlists.get(self.selected_list),
                &self.ticker_input,
                &self.quotes,
            ),
            Page::Settings => ui::settings_page::settings_page_view(&self.config.settings),
        };

        self.core
            .applet
            .popup_container(page)
            .limits(
                Limits::NONE
                    .min_width(globals::POPUP_WIDTH)
                    .max_width(globals::POPUP_WIDTH)
                    .min_height(globals::POPUP_HEIGHT)
                    .max_height(globals::POPUP_HEIGHT),
            )
            .into()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

impl AppModel {
    fn save_data(&self) {
        // write_entry is generated by #[derive(CosmicConfigEntry)] on AppData
        if let Some(handle) = &self.data_handle
            && let Err(err) = self.data.write_entry(handle)
        {
            eprintln!("failed to save the app data: {err}");
        }
    }

    fn save_settings(&mut self, settings: Settings) {
        // set_settings is generated by #[derive(CosmicConfigEntry)] on Config:
        // it updates self.config and writes the file, only if the value changed.
        if let Some(handle) = &self.config_handle
            && let Err(err) = self.config.set_settings(handle, settings)
        {
            eprintln!("failed to save settings: {err}");
        }
    }

    /// Starts one fetch for every symbol in every watchlist.
    ///
    /// Returns no task when there are no symbols, since Yahoo rejects an empty request.
    fn fetch_all_quotes(&mut self) -> Task<cosmic::Action<Message>> {
        let symbols = provider::all_symbols(&self.data.watchlists);
        if symbols.is_empty() {
            return Task::none();
        }

        self.in_flight = true;

        let client = self.yf_client.clone();
        cosmic::task::future(async move {
            Message::QuotesFetched(
                provider::fetch_quotes(client, &symbols)
                    .await
                    .map_err(|e| e.to_string()),
            )
        })
    }
}
