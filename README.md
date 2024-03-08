Simple crate to login to Pinterest and get the cookies via Chromiumoxide to simulate a browser (open a real browser actually), to use the Pinterest API without needing a developer account or an API key or anything that costs money :).

[![crates.io](https://img.shields.io/crates/v/pinterest-login.svg)](https://crates.io/crates/pinterest-login)
[![docs.rs](https://docs.rs/pinterest-login/badge.svg)](https://docs.rs/pinterest-login)
[![downloads](https://img.shields.io/crates/d/pinterest-login.svg)](https://crates.io/crates/pinterest-login)
[![license](https://img.shields.io/crates/l/pinterest-login.svg)][mit]

Asynchronous, and uses async-std as the runtime by default (you can use tokio if you want)

>  WARNING: This project isn't officially supported by Pinterest, and it's not affiliated with Pinterest in any way.

## Examples

### With the `async-std` runtime

```rust
use pinterest_login::config_builder::DefaultBrowserConfigBuilder;
use pinterest_login::login;
use pinterest_login::login_bot::DefaultBrowserLoginBot;

#[async_std::main]
async fn main() {
    let email = std::env::var("PINTEREST_EMAIL").unwrap();
    let password = std::env::var("PINTEREST_PASSWORD").unwrap();

    let bot = DefaultBrowserLoginBot::new(email.as_str(), password.as_str());
    let config_builder = DefaultBrowserConfigBuilder::default();

    match login(&bot, &config_builder).await {
        Ok(cookies) => {
            // Store the cookies in a file or something, and do whatever you want with them
            // I like the cookies bay the way
            // ...
            println!("{}", cookies.len());
            println!("{:?}", cookies);
        }
        Err(e) => {
            // The login was unsuccessful
            eprintln!("The login was unsuccessful: {}", e);
        }
    };
}
```
```rust
use pinterest_login::config_builder::DefaultBrowserConfigBuilder;
use pinterest_login::login;
use pinterest_login::login_bot::DefaultBrowserLoginBot;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let email = std::env::var("PINTEREST_EMAIL").unwrap();
    let password = std::env::var("PINTEREST_PASSWORD").unwrap();

    let bot = DefaultBrowserLoginBot::new(email.as_str(), password.as_str());

   // Show the browser, and set the request timeout to 2 seconds
    let config_builder = DefaultBrowserConfigBuilder::new(false, Duration::from_secs(2).into(), None);

    match login(&bot, &config_builder).await {
        Ok(cookies) => {
            // ...
        }
        Err(e) => {
            // The login was unsuccessful
            eprintln!("The login was unsuccessful: {}", e);
        }
    };
}
```

### With `tokio` runtime
```rust
use pinterest_login::config_builder::DefaultBrowserConfigBuilder;
use pinterest_login::login;
use pinterest_login::login_bot::DefaultBrowserLoginBot;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let email = std::env::var("PINTEREST_EMAIL").unwrap();
    let password = std::env::var("PINTEREST_PASSWORD").unwrap();

   let bot = DefaultBrowserLoginBot::new(email.as_str(), password.as_str());

  // Show the browser, and set the request timeout to 2 seconds
   let config_builder = DefaultBrowserConfigBuilder::new(false, Duration::from_secs(2).into(), None);

    match login(&bot, &config_builder).await {
        Ok(cookies) => {
            // ...
        }
        Err(e) => {
            // The login was unsuccessful
            eprintln!("The login was unsuccessful: {}", e);
        }
    };
}
```

## Features
* `async-std-runtime`: Use the async-std runtime instead of tokio (enabled by default)
* `tokio-runtime`: Use the tokio runtime instead of async-std
* `debug`: Enable debug logging


## Contributing
I'm happy to accept any contributions, just consider reading the [CONTRIBUTING.md](https://github.com/0x61nas/pinterest-login/blob/aurora/CONTRIBUTING.md) guide first. to avoid waste waste our time on some unnecessary things.

> the main keywords are: **signed commits**, **conventional commits**, **no emojis**, **linear history**, **try to compine the commits if posoble**

## License
This project is licensed under ether the [MIT license][mit] or the [Unlicense license][unlicense], you can choose which one you want.

[mit]: https://github.com/0x61nas/pinterest-login/blob/aurora/LICENSE
[unlicense]: https://github.com/0x61nas/pinterest-login/blob/aurora/LICENSE-UNLICENSE


> This project is part of the [pinterest-rs](https://github.com/0x61nas/pinterest-rs) project


## Dependencies graph

![deps graph](./_deps.png)

> Generated with [cargo-depgraph](https://crates.io/crates/cargo-depgraph)
