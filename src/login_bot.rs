use chromiumoxide::Page;
#[cfg(feature = "debug")]
use log::{info, trace, debug};

use crate::{PINTEREST_LOGIN_URL, PinterestLoginError};

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

impl<'a> DefaultBrowserLoginBot<'a> {
    /// Creates a new default login bot
    ///
    /// # Arguments
    /// * `email` - The email to login with
    /// * `password` - The password to login with
    pub fn new(email: &'a str, password: &'a str) -> Self {
        Self {
            email,
            password,
        }
    }
}

#[async_trait::async_trait]
impl BrowserLoginBot for DefaultBrowserLoginBot<'_> {
    #[inline(always)]
    async fn fill_login_form(&self, page: &Page) -> crate::Result<()> {
        const EMAIL_INPUT_SELECTOR: &str = "input#email";
        const PASSWORD_INPUT_SELECTOR: &str = "input#password";

        #[cfg(feature = "debug")] {
            trace!("Filling the login form with the email: {} and password: {}", self.email, self.password);
            debug!("entering the email");
            trace!("Finding the email input field with the selector: {}", EMAIL_INPUT_SELECTOR);
        }
        // Wait for the page to load, and then find the email input field and fill it
        page.find_element(EMAIL_INPUT_SELECTOR).await?
            .click().await?
            .type_str(self.email).await?;

        #[cfg(feature = "debug")] {
            debug!("Email entered successfully, entering the password");
            trace!("Finding the password input field with the selector: {}", PASSWORD_INPUT_SELECTOR);
        }

        // Find the password input field and fill it
        page.find_element(PASSWORD_INPUT_SELECTOR).await?
            .click().await?
            .type_str(self.password).await?;

        #[cfg(feature = "debug")]
        debug!("Password entered successfully");

        Ok(())
    }

    #[inline(always)]
    async fn submit_login_form(&self, page: &Page) -> crate::Result<()> {
        const LOGIN_BUTTON_SELECTOR: &str = "button[type='submit']";

        #[cfg(feature = "debug")] {
            debug!("Submitting the login form");
            info!("Finding the submit button and clicking it");
            trace!("Finding the submit button with the selector: {}", LOGIN_BUTTON_SELECTOR);
        }
        // Find the submit button and click it
        page.find_element(LOGIN_BUTTON_SELECTOR).await?
            .click().await?;

        #[cfg(feature = "debug")]
        debug!("Login form submitted successfully");

        Ok(())
    }

    #[inline(always)]
    async fn check_login(&self, page: &Page) -> crate::Result<()> {
        #[cfg(feature = "debug")]
        debug!("Checking if the login was successful");
        // Wait for the page to load, and then check if the login was successful
        match page.wait_for_navigation().await?.url().await? {
            None => {
                #[cfg(feature = "debug")]
                debug!("Couldn't get the url, the login was unsuccessful");
                // If we can't get the url, then the login was unsuccessful
                Err(PinterestLoginError::AuthenticationError)
            }
            Some(url) => {
                #[cfg(feature = "debug")] {
                    debug!("Got the url: {}", url);
                    info!("Checking if the url is the same as the login url");
                }
                if url == PINTEREST_LOGIN_URL {
                    #[cfg(feature = "debug")]
                    debug!("The url is the same as the login url, the login was unsuccessful");
                    // If the url is the same as the login url, then the login was unsuccessful
                    Err(PinterestLoginError::AuthenticationError)
                } else {
                    #[cfg(feature = "debug")]
                    info!("The url is not the same as the login url, the login was successful");
                    Ok(())
                }
            }
        }
    }
}
