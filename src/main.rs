// #![allow(warnings)]
use std::io::Error;

const VERSION: &'static str = "0.1";
const AUTHOR: &'static str = "Nick Lauri <khoanta.96@gmail.com>";
const ROUTER_DEFAULT_ADDR: &'static str = "192.168.1.1:80";

mod client;
mod commandline;
mod router;

use commandline::{ARGS, PARSER};

#[macro_export]
macro_rules! verbose {
    ($( $x:expr ),*) => (
        if crate::commandline::ARGS.verbose {
            print!("routerctl: ");
            $( $x )*
        }
    )
}

fn start() -> Result<(), Error> {
    if ARGS.login_only {
        return router::login();
    }

    if ARGS.get_clients_only {
        router::login()?;
        router::active_clients()?;
        return router::logout();
    }

    if ARGS.enable_macflt {
        router::login()?;
        router::macflt_enable()?;
        return router::logout();
    }

    if ARGS.disable_macflt {
        router::login()?;
        router::macflt_disable()?;
        return router::logout();
    }

    if ARGS.show_status {
        router::login()?;
        router::macflt_status()?;
        router::active_clients()?;
        return router::logout();
    }

    if ARGS.logout {
        return router::logout();
    }

    if ARGS.reboot {
        router::login()?;
        return router::reboot();
    }

    Ok(())
}

fn main() {
    if let Err(e) = start() {
        eprintln!("routerctl: ERR: {}", e.to_string());
    }

    if std::env::args().count() == 1 {
        println!("{}", PARSER.usage());
    }
}
