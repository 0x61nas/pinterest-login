use pinterest_login::config_builder::DefaultBrowserConfigBuilder;
use pinterest_login::login;
use pinterest_login::login_bot::DefaultBrowserLoginBot;
use std::io::{self, Write};
use std::time::Duration;
use std::{env, process};

#[cfg(feature = "log")]
extern crate log;

// #[tokio::main]
#[cfg_attr(all(feature = "__async-std", not(feature = "tokio")), async_std::main)]
#[cfg_attr(feature = "tokio", tokio::main)]
async fn main() {
    let mut headless = true;
    let mut timeout = 3;
    let mut args = env::args().skip(1);
    while let Some(arg) = &args.next() {
        let arg = arg.trim_matches('-');
        match arg {
            "head" => headless = false,
            "t" | "timeout" => {
                timeout = args
                    .next()
                    .unwrap_or_else(|| fail(format!("expected value after `-{arg}`").leak()))
                    .parse::<u64>()
                    .unwrap_or_else(|e| fail(format!("Can't parse `-{arg}` value: {e}").leak()))
                    .to_owned()
            }
            unknown => fail(format!("Unknown argument: `{unknown}`").leak()),
        }
    }

    let Ok((email, password)) = get_auth_info() else {
        fail("Can't get the authentication info")
    };

    let bot = DefaultBrowserLoginBot::new(email.as_str(), password.as_str());

    let config_builder =
        DefaultBrowserConfigBuilder::new(headless, Duration::from_secs(timeout).into(), None);

    #[cfg(feature = "log")]
    pretty_env_logger::init_timed();
    match login(&bot, &config_builder).await {
        Ok(cookies) => {
            println!("{cookies:?}");
        }
        Err(e) => {
            eprintln!("{e}");
        }
    };
}

fn get_auth_info() -> io::Result<(String, String)> {
    if let Ok(email) = env::var("PINTEREST_EMAIL") {
        let Ok(password) = env::var("PINTEREST_PASSWORD") else {
            return Ok((email, rpassword::prompt_password("Account password: ")?));
        };
        Ok((email, password))
    } else {
        print!("Pinterest email/username: ");
        io::stdout().flush()?;
        let mut email = String::new();
        io::stdin().read_line(&mut email)?;
        email.pop(); // rm `\n`
        Ok((email, rpassword::prompt_password("Account password: ")?))
    }
}

#[cold]
fn fail(msg: &'static str) -> ! {
    eprintln!("{msg}");
    process::exit(1)
}
