use std::time::Duration;

use async_std::task::sleep;
use chromiumoxide::{layout::BoundingBox, Element, Page};
#[cfg(feature = "log")]
use log::{debug, info, trace};

use crate::PinterestLoginError;

/// Trait for login bots, which are used to fill and submit the login form in the browser
///
/// # Example
/// ```ignore
/// use chromiumoxide::Page;
/// # use pinterest_login::login_bot::BrowserLoginBot;
/// # use pinterest_login::Result;
///
/// struct MyLoginBot;
///
/// #[async_trait::async_trait]
/// impl BrowserLoginBot for MyLoginBot {
///    async fn fill_login_form(&self, page: &Page) -> Result<()> {
///        // ...
///    }
///
///    async fn submit_login_form(&self, page: &Page) -> Result<()> {
///        // ...
///    }
///
///    async fn check_login(&self, page: &Page) -> Result<()> {
///        // ...
///    }
/// }
/// ```
#[async_trait::async_trait]
pub trait BrowserLoginBot {
    /// Fills the login form fields with the required data
    async fn fill_login_form(&self, page: &Page) -> crate::Result<()>;
    /// Submits the login form
    async fn submit_login_form(&self, page: &Page) -> crate::Result<()>;
    /// Checks if the login was successful
    async fn check_login(&self, page: &Page) -> crate::Result<()>;
}

/// The default login bot, that provides methods to fill and submit the login form in the browser
/// This login bot enables you to login to pinterest with an email and password
///
/// # Example
/// ```ignore
/// # use pinterest_login::login_bot::{BrowserLoginBot, DefaultBrowserLoginBot};
///
/// let login_bot = DefaultBrowserLoginBot::new("email", "password");
///
/// // ...
/// ```
///
/// U don't need to use the login bot directly, it is used by the login function,
/// you just send it to the login function and it will use it to fill and submit the login form
pub struct DefaultBrowserLoginBot<'a> {
    email: &'a str,
    password: &'a str,
}

const EMAIL_INPUT_SELECTOR: &str = "input#email";
const PASSWORD_INPUT_SELECTOR: &str = "input#password";
const LOGIN_BUTTON_SELECTOR: &str = "//*[contains(text(), 'Log in')]";
const WAIT_DELAY: u64 = 20;

impl<'a> DefaultBrowserLoginBot<'a> {
    /// Creates a new default login bot
    ///
    /// # Arguments
    /// * `email` - The email to login with
    /// * `password` - The password to login with
    pub fn new(email: &'a str, password: &'a str) -> Self {
        Self { email, password }
    }
}

#[async_trait::async_trait]
impl BrowserLoginBot for DefaultBrowserLoginBot<'_> {
    #[inline]
    async fn fill_login_form(&self, page: &Page) -> crate::Result<()> {
        #[cfg(feature = "log")]
        {
            trace!(
                "Filling the login form with the email: {} and password: {}",
                self.email,
                self.password
            );
            debug!("entering the email");
            trace!(
                "Finding the email input field with the selector: {}",
                EMAIL_INPUT_SELECTOR
            );
        }
        // Wait for the page to load, and then find the email input field and fill it
        let e = loop {
            let Ok(e) = page.find_element(EMAIL_INPUT_SELECTOR).await else {
                sleep(Duration::from_millis(WAIT_DELAY)).await;
                continue;
            };
            break e;
        };

        e.type_str(self.email).await?;

        #[cfg(feature = "log")]
        {
            debug!("Email entered successfully, entering the password");
            trace!(
                "Finding the password input field with the selector: {}",
                PASSWORD_INPUT_SELECTOR
            );
        }

        // Find the password input field and fill it
        page.find_element(PASSWORD_INPUT_SELECTOR)
            .await?
            .focus()
            .await?
            .type_str(self.password)
            .await?;

        #[cfg(feature = "log")]
        debug!("Password entered successfully");

        Ok(())
    }

    #[inline]
    async fn submit_login_form(&self, page: &Page) -> crate::Result<()> {
        #[cfg(feature = "log")]
        {
            debug!("Submitting the login form");
            info!("Finding the submit button and clicking it");
            trace!(
                "Finding the submit button with the selector: {}",
                LOGIN_BUTTON_SELECTOR
            );
        }
        let mut buttons = Vec::with_capacity(2);
        let mut old_bounds = Vec::with_capacity(2);
        // Find the submit button and click it
        for e in page.find_xpaths(LOGIN_BUTTON_SELECTOR).await? {
            e.click().await?;
            old_bounds.push(e.bounding_box().await?);
            buttons.push(e);
        }

        while page.find_element(EMAIL_INPUT_SELECTOR).await.is_ok() && {
            // We need this in case if user enters an invalid authentication data.
            // because in this case pinterest will not change the page and just show up a little tooltip
            // under the wrong box, and we don't have any way to handle that case besid this _wanky_ sloution
            // TODO: find a better way.
            async fn cheack(buttons: &[Element], ob: &[BoundingBox]) -> crate::Result<bool> {
                for (i, b) in ob.iter().enumerate() {
                    let nb = buttons[i].bounding_box().await?;
                    if nb.x != b.x || nb.y != b.y {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            cheack(&buttons, &old_bounds).await?
        } {
            sleep(Duration::from_millis(WAIT_DELAY)).await;
        }

        #[cfg(feature = "log")]
        debug!("Login form submitted successfully");

        Ok(())
    }

    #[inline]
    async fn check_login(&self, page: &Page) -> crate::Result<()> {
        use lazy_regex::regex;
        #[cfg(feature = "log")]
        debug!("Checking if the login was successful");
        // Wait for the page to load, and then check if the login was successful
        match page.wait_for_navigation().await?.url().await? {
            None => {
                #[cfg(feature = "log")]
                debug!("Couldn't get the url, the login was unsuccessful");
                // If we can't get the url, then the login was unsuccessful
                Err(PinterestLoginError::AuthenticationError)
            }
            Some(url) => {
                #[cfg(feature = "log")]
                {
                    debug!("Got the url: {}", url);
                    info!("Checking if the url is the same as the login url");
                }
                let regex = regex!(r#"^(https?:\/\/(?:www\.)?pinterest\.com\/login).*$"#);
                if regex.is_match(&url) {
                    #[cfg(feature = "log")]
                    debug!("The url is the same as the login url, the login was unsuccessful");
                    // If the url is the same as the login url, then the login was unsuccessful
                    Err(PinterestLoginError::AuthenticationError)
                } else {
                    #[cfg(feature = "log")]
                    info!("The url is not the same as the login url, the login was successful");
                    Ok(())
                }
            }
        }
    }
}
