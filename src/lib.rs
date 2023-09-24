//! Simple crate to login to Pinterest and get the cookies via Chromiumoxide to simulate a browser (open a real browser actually), to use the Pinterest API without needing a developer account or an API key or anything that costs money :).
//!
//! [![crates.io](https://img.shields.io/crates/v/pinterest-login.svg)](https://crates.io/crates/pinterest-login)
//! [![docs.rs](https://docs.rs/pinterest-login/badge.svg)](https://docs.rs/pinterest-login)
//! [![downloads](https://img.shields.io/crates/d/pinterest-login.svg)](https://crates.io/crates/pinterest-login)
//! [![license](https://img.shields.io/crates/l/pinterest-login.svg)][mit]
//!
//! Asynchronous, and uses async-std as the runtime by default (you can use tokio if you want)
//!
//! >  WARNING: This project isn't officially supported by Pinterest, and it's not affiliated with Pinterest in any way.
//!
//! # Examples
//!
//! ## With the `async-std` runtime
//!
//! ```ignore
//! use pinterest_login::config_builder::DefaultBrowserConfigBuilder;
//! use pinterest_login::login;
//! use pinterest_login::login_bot::DefaultBrowserLoginBot;
//!
//! #[async_std::main]
//! async fn main() {
//!     let email = std::env::var("PINTEREST_EMAIL").unwrap();
//!     let password = std::env::var("PINTEREST_PASSWORD").unwrap();
//!
//!     let bot = DefaultBrowserLoginBot::new(email.as_str(), password.as_str());
//!     let config_builder = DefaultBrowserConfigBuilder::default();
//!
//!     match login(&bot, &config_builder).await {
//!         Ok(cookies) => {
//!             // Store the cookies in a file or something, and do whatever you want with them
//!             // I like the cookies bay the way
//!             // ...
//!             println!("{}", cookies.len());
//!             println!("{:?}", cookies);
//!         }
//!         Err(e) => {
//!             // The login was unsuccessful
//!             eprintln!("The login was unsuccessful: {}", e);
//!         }
//!     };
//! }
//! ```
//! ```ignore
//! use pinterest_login::config_builder::DefaultBrowserConfigBuilder;
//! use pinterest_login::login;
//! use pinterest_login::login_bot::DefaultBrowserLoginBot;
//! use std::time::Duration;
//!
//! #[async_std::main]
//! async fn main() {
//!     let email = std::env::var("PINTEREST_EMAIL").unwrap();
//!     let password = std::env::var("PINTEREST_PASSWORD").unwrap();
//!
//!     let bot = DefaultBrowserLoginBot::new(email.as_str(), password.as_str());
//!
//!    // Show the browser, and set the request timeout to 2 seconds
//!     let config_builder = DefaultBrowserConfigBuilder::new(false, Duration::from_secs(2).into(), None);
//!
//!     match login(&bot, &config_builder).await {
//!         Ok(cookies) => {
//!             // ...
//!         }
//!         Err(e) => {
//!             // The login was unsuccessful
//!             eprintln!("The login was unsuccessful: {}", e);
//!         }
//!     };
//! }
//! ```
//!
//! ## With `tokio` runtime
//! ```ignore
//! use pinterest_login::config_builder::DefaultBrowserConfigBuilder;
//! use pinterest_login::login;
//! use pinterest_login::login_bot::DefaultBrowserLoginBot;
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!     let email = std::env::var("PINTEREST_EMAIL").unwrap();
//!     let password = std::env::var("PINTEREST_PASSWORD").unwrap();
//!
//!    let bot = DefaultBrowserLoginBot::new(email.as_str(), password.as_str());
//!
//!   // Show the browser, and set the request timeout to 2 seconds
//!    let config_builder = DefaultBrowserConfigBuilder::new(false, Duration::from_secs(2).into(), None);
//!
//!     match login(&bot, &config_builder).await {
//!         Ok(cookies) => {
//!             // ...
//!         }
//!         Err(e) => {
//!             // The login was unsuccessful
//!             eprintln!("The login was unsuccessful: {}", e);
//!         }
//!     };
//! }
//! ```
//!
//! # Features
//! * `async-std-runtime`: Use the async-std runtime instead of tokio (enabled by default)
//! * `tokio-runtime`: Use the tokio runtime instead of async-std
//! * `debug`: Enable debug logging
//!
//!
//! # Contributing
//! I'm happy to accept any contributions, just consider reading the [CONTRIBUTING.md](https://github.com/0x61nas/pinterest-login/blob/aurora/CONTRIBUTING.md) guide first. to avoid waste waste our time on some unnecessary things.
//!
//! > the main keywords are: **signed commits**, **conventional commits**, **no emojis**, **linear history**, **the PR shouldn't have more than tree commits most of the time**
//!
//! # License
//! This project is licensed under ether the [MIT license][mit] or the [Unlicense license][unlicense], you can choose which one you want.
//!
//! [mit]: https://github.com/0x61nas/pinterest-login/blob/aurora/LICENSE
//! [unlicense]: https://github.com/0x61nas/pinterest-login/blob/aurora/LICENSE-UNLICENSE
//!
//!
//! > This project is part of the [pinterest-rs](https://github.com/0x61nas/pinterest-rs) project
//!
#![deny(missing_docs, clippy::all)]

/// The chromiumoxide browser config builder
pub mod config_builder;
/// The pinterest login bot
pub mod login_bot;

use std::collections::HashMap;
// #[cfg(feature = "async-std-runtime")]
// use async_std::prelude::StreamExt;
use crate::config_builder::BrowserConfigBuilder;
use crate::login_bot::BrowserLoginBot;
use chromiumoxide::Browser;
use futures::StreamExt;
#[cfg(feature = "debug")]
use log::{debug, info, trace};

/// The pinterest login url
pub const PINTEREST_LOGIN_URL: &str = "https://pinterest.com/login";

/// Pinterest login error type
#[derive(Debug, thiserror::Error)]
pub enum PinterestLoginError {
    /// Chromiumoxide error, returned when chromiumoxide fails to connect to the browser or when the browser fails to load the login page or when the timeout is reached
    /// See [chromiumoxide::error::CdpError](https://docs.rs/chromiumoxide/latest/chromiumoxide/error/enum.CdpError.htm) for more details
    #[error("{0}")]
    CdpError(#[from] chromiumoxide::error::CdpError),
    /// The browser config builder failed to build the browser config
    #[error("{0}")]
    BrowserConfigBuildError(String),
    /// The login bot failed to fill or submit the login form, or the authentication is incorrect
    #[error("Authentication error: The email or password you entered is incorrect.")]
    AuthenticationError,
}

/// A type alias for `Result<T, PinterestLoginError>`
pub type Result<T> = std::result::Result<T, PinterestLoginError>;

/// Logs into Pinterest and returns the cookies as a HashMap
///
/// # Arguments
/// * `login_bot` - The login bot to use to fill and submit the login form
/// * `browser_config_builder` - The browser config builder to use to build the browser config
///
/// # Example
/// ```ignore
/// # use std::collections::HashMap;
/// # use pinterest_login::config_builder::DefaultBrowserConfigBuilder;
/// # use pinterest_login::login;
/// # use pinterest_login::login_bot::DefaultBrowserLoginBot;
///
/// async fn login_to_pinterest(email: &str, password: &str) -> pinterest_login::Result<HashMap<String, String>> {
///     let browser_config_builder = DefaultBrowserConfigBuilder::default();
///     let bot = DefaultBrowserLoginBot::new(email, password);
///
///     let cookies = login(&bot, &browser_config_builder).await?;
///     Ok(cookies)
/// }
/// ```
///
/// # Errors
/// * `CdpError` - If there is an error with chromiumoxide (like launching timeout, or request timeout, network error, etc.)  see [chromiumoxide::error::CdpError](https://docs.rs/chromiumoxide/latest/chromiumoxide/error/enum.CdpError.html) to see all the errors
/// * `BrowserConfigBuildError` - If there is an error building the browser config
/// * `AuthenticationError` - If the email or password is incorrect
///
#[inline]
pub async fn login(
    login_bot: &dyn BrowserLoginBot,
    config_builder: &dyn BrowserConfigBuilder,
) -> Result<HashMap<String, String>> {
    #[cfg(feature = "debug")]
    info!("Launching the browser");

    let (browser, mut handler) = Browser::launch(config_builder.build_browser_config()?).await?;

    #[cfg(feature = "debug")]
    info!(
        "The browser has been launched\nBrowser version: {:?}",
        browser.version().await?
    );

    #[cfg(feature = "async-std-runtime")]
    let handle = async_std::task::spawn(async move {
        loop {
            let _event = handler.next().await;
        }
    });

    #[cfg(all(feature = "tokio-runtime", not(feature = "async-std-runtime")))]
    let handle = tokio::spawn(async move {
        loop {
            let _event = handler.next().await;
        }
    });

    #[cfg(feature = "debug")]
    info!("Navigating to the login page: {}", PINTEREST_LOGIN_URL);

    let page = browser.new_page(PINTEREST_LOGIN_URL).await?;
    page.wait_for_navigation().await?;

    #[cfg(feature = "debug")]
    {
        info!("The login page has been loaded");
        trace!("The login page content: {}", page.content().await?);
        debug!("The login page cookies: {:?}", page.get_cookies().await?);
        info!("Filling the login form");
    }
    // Fill the login form
    login_bot.fill_login_form(&page).await?;
    #[cfg(feature = "debug")]
    info!("Submitting the login form");
    // Click the login button
    login_bot.submit_login_form(&page).await?;

    #[cfg(feature = "debug")]
    {
        info!("The login form has been submitted");
        info!("Waiting for the login to complete, and checking if the login was successful");
    }
    // Check if the login was successful
    login_bot.check_login(&page).await?;

    let mut cookies = HashMap::with_capacity(5);

    #[cfg(feature = "debug")]
    info!("The login was successful, getting the cookies");
    // Get the cookies
    let c = page.get_cookies().await?;

    #[cfg(feature = "debug")]
    {
        info!("The cookies have been retrieved");
        debug!("The cookies: {c:?}");
        debug!("The cookies length: {}", c.len());
    }

    #[cfg(feature = "debug")]
    info!("Collecting the cookies values and names into a HashMap");
    for cookie in c {
        #[cfg(feature = "debug")]
        trace!("Inserting the cookie: {} : {}", cookie.name, cookie.value);

        cookies.insert(cookie.name, cookie.value);
    }

    #[cfg(feature = "debug")]
    info!("Canceling the event handler");
    #[cfg(feature = "async-std-runtime")]
    // Cancel the event handler
    handle.cancel().await;
    #[cfg(all(feature = "tokio-runtime", not(feature = "async-std-runtime")))]
    // Cancel the event handler
    handle.abort();

    // #[cfg(feature = "debug")]
    // info!("Closing the browser");
    // Close the browser
    // browser.close().await?;

    #[cfg(feature = "debug")]
    trace!("The cookies: {cookies:?}");

    Ok(cookies)
}
