// MIT License

// Copyright (c) 2019 Nick Lauri

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// #![allow(warnings)]
use std::io::Error;

const VERSION: &str = "0.1";
const AUTHOR: & str = "Nick Lauri <khoanta.96@gmail.com>";
const ROUTER_DEFAULT_ADDR: &str = "192.168.1.1:80";

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
