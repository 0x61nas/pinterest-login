use crate::PinterestLoginError;
use chromiumoxide::BrowserConfig;
#[cfg(feature = "log")]
use log::{debug, info, trace};

/// The browser config builder trait, that provides a method to build a chromiumoxide browser config
/// You can implement this trait for your own struct, and use it to build a chromiumoxide browser config
///
/// # Example
/// ```
/// use chromiumoxide::BrowserConfig;
/// use pinterest_login::config_builder::BrowserConfigBuilder;
/// use pinterest_login::PinterestLoginError;
///
/// struct MyBrowserConfigBuilder;
///
/// impl BrowserConfigBuilder for MyBrowserConfigBuilder {
///    fn build_browser_config(&self) -> pinterest_login::Result<BrowserConfig> {
///       let mut browser_config = BrowserConfig::builder();
///
///      // Do whatever you want with the browser config
///      browser_config = browser_config.with_head(); // For example, set the browser to head mode
///
///     // Build the browser config and return it
///      browser_config.build().map_err(PinterestLoginError::BrowserConfigBuildError)
///   }
/// }
/// ```
pub trait BrowserConfigBuilder {
    /// Builds a chromiumoxide browser config
    fn build_browser_config(&self) -> crate::Result<BrowserConfig>;
}

/// The default browser config builder, that provides a method to build a chromiumoxide browser config
/// This builder enables you to set the headless mode, the request timeout and the launch timeout
///
/// # Example
/// ```
/// # use pinterest_login::config_builder::{BrowserConfigBuilder, DefaultBrowserConfigBuilder};
/// use std::time::Duration;
///
/// let browser_config_builder = DefaultBrowserConfigBuilder::new(true, Duration::from_secs(3).into(), None);
/// let browser_config = browser_config_builder.build_browser_config().unwrap();
/// ```
pub struct DefaultBrowserConfigBuilder {
    headless: bool,
    request_timeout: Option<std::time::Duration>,
    launch_timeout: Option<std::time::Duration>,
}

impl DefaultBrowserConfigBuilder {
    /// Creates a new default browser config builder
    ///
    /// # Arguments
    /// * `headless` - Whether to launch the browser in headless mode or not (you probably want this to be true)
    /// * `request_timeout` - The timeout for requests, the default is no timeout (you probably want to set this unless you want to wait forever if you take the internet from potato)
    /// * `lunch_timeout` - The timeout for launching the browser, the default is no timeout
    pub fn new(
        headless: bool,
        request_timeout: Option<std::time::Duration>,
        launch_timeout: Option<std::time::Duration>,
    ) -> Self {
        Self {
            headless,
            request_timeout,
            launch_timeout,
        }
    }
}

impl BrowserConfigBuilder for DefaultBrowserConfigBuilder {
    #[inline(always)]
    fn build_browser_config(&self) -> crate::Result<BrowserConfig> {
        #[cfg(feature = "log")]
        {
            debug!("Building browser config");
            trace!("Headless: {}", self.headless);
            trace!("Request timeout: {:?}", self.request_timeout);
            trace!("Launch timeout: {:?}", self.launch_timeout);
        }
        let mut browser_config_builder = if self.headless {
            BrowserConfig::builder()
        } else {
            BrowserConfig::builder().with_head()
        };

        if let Some(timeout) = self.request_timeout {
            #[cfg(feature = "log")]
            {
                trace!("Setting request timeout to {:?}", timeout);
            }
            browser_config_builder = browser_config_builder.request_timeout(timeout);
        }

        if let Some(timeout) = self.launch_timeout {
            #[cfg(feature = "log")]
            {
                trace!("Setting launch timeout to {:?}", timeout);
            }
            browser_config_builder = browser_config_builder.launch_timeout(timeout);
        }

        #[cfg(feature = "log")]
        {
            info!("Built browser config");
            trace!("Browser config: {:?}", browser_config_builder);
        }

        browser_config_builder
            .build()
            .map_err(PinterestLoginError::BrowserConfigBuildError)
    }
}

impl Default for DefaultBrowserConfigBuilder {
    /// Creates a new default browser config builder, with the following values:
    /// * `headless` - true
    /// * `request_timeout` - 5 seconds
    /// * `lunch_timeout` - None
    fn default() -> Self {
        Self::new(true, Some(std::time::Duration::from_secs(5)), None)
    }
}
