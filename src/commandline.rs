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

use std::sync::Arc;
use std::path::Path;

use clap::{ArgMatches, clap_app};
use lazy_static::lazy_static;

#[derive(Debug, Default)]
pub struct Args {
    pub router: &'static str,
    pub password: &'static str,
    pub login_only: bool,
    pub get_clients_only: bool,
    pub get_macs_only: bool,
    pub show_status: bool,
    pub enable_macflt: bool,
    pub disable_macflt: bool,
    pub add_white_list: Option<Vec<&'static str>>,
    pub add_black_list: Option<Vec<&'static str>>,
    pub black_list_file: Option<Vec<String>>,
    pub show_white_list: bool,
    pub show_black_list: bool,
    pub logout: bool,
    pub reboot: bool,
    pub verbose: bool,
}

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
    pub static ref PARSER: Arc<ArgMatches<'static>> = Arc::new(Args::initial());
    pub static ref DEFAULT_BLACK_LIST: String = {
            // this static var will not check if this path is a valid file.
            let mut current_dir = std::env::current_exe().unwrap();
            current_dir.pop();
            current_dir.push("macs.txt");
            current_dir.to_str().unwrap().to_string()
        };
}

impl Args {
    pub fn parse() -> Self {
        let mut args: Args = Default::default();
        args.password = PARSER.value_of("password").unwrap_or("admin");
        args.router = PARSER.value_of("router").unwrap_or(super::ROUTER_DEFAULT_ADDR);

        // iter.find returns Option<Self::Item> => Option.map to get it out
        args.add_white_list = PARSER.values_of("add_white_list").map(|iter| {
            iter.clone().find(|m| !super::router::MAC_VALIDATE.is_match(m))
                .map(|m| panic!("routerctl::Args::parse: ERR: invalid MAC: '{}'", m));
            iter.collect()
        });

        args.add_black_list = PARSER.values_of("add_black_list").map(|iter| {
            iter.clone().find(|m| !super::router::MAC_VALIDATE.is_match(m))
                .map(|m| panic!("routerctl::Args::parse: ERR: invalid MAC: '{}'", m));
            iter.collect()
        });


        // can use unwrap_or, but whatever, iduncare
        let mut __use_default_value: bool = false;
        if let Some(f) = PARSER.value_of("black_list_file")
            .or_else(|| { __use_default_value=true; Some(&*DEFAULT_BLACK_LIST) })
        {
            dbg!(f);
            if Path::new(f).is_file() {
                // str.split_whitespace > str.lines + str.trim
                // iter.filter is simple but less details
                let mut list = vec![];
                String::from_utf8_lossy(&std::fs::read(f).unwrap_or_else(|_| panic!("can't read '{}'", f)))
                    .split_whitespace().for_each(|l| {
                        if super::router::MAC_VALIDATE.is_match(l) {
                            list.push(l.to_string());
                        }
                        else {
                            println!("routerctl: ERR: entry({}) is not a valid MAC.", l);
                        }
                    });
                args.black_list_file = Some(list);
            }
            else if !__use_default_value {
                println!("routerctl: ERR: '{}' is not a file.", f);
            }
        }

        if PARSER.is_present("login_only") { args.login_only = true; }
        if PARSER.is_present("get_clients_only") { args.get_clients_only = true; }
        if PARSER.is_present("get_macs_only") { args.get_macs_only = true; }
        if PARSER.is_present("show_status") { args.show_status = true; }
        if PARSER.is_present("show_white_list") { args.show_white_list = true; }
        if PARSER.is_present("show_black_list") { args.show_black_list = true; }
        if PARSER.is_present("enable_macflt") { args.enable_macflt = true; }
        if PARSER.is_present("disable_macflt") { args.disable_macflt = true; }
        if PARSER.is_present("logout") { args.logout = true; }
        if PARSER.is_present("reboot") { args.reboot = true; }
        if PARSER.is_present("verbose") { args.verbose = true; }
        
        args
    }

    fn initial() -> ArgMatches<'static> {
        clap_app!(routerctl => 
            (version: super::VERSION)
            (author: super::AUTHOR)
            (about: "Router controler for router model GPON G-93RG1")
            (@arg router: -r --router +takes_value "Set router address, default: 192.168.1.1:80")
            (@arg password: -p --password +takes_value "Set password for user admin")
            (@arg login_only: -l --login "Perform login action then exit")
            (@arg get_clients_only: -c --clients "Get all active clients")
            (@arg get_macs_only: -m --macs "Get all active clients' mac")
            (@arg show_status: -s --status "Get MAC filter status and active clients")
            (@arg enable_macflt: -e --enable "Enable MACs filter")
            (@arg disable_macflt: -d --disable "Disable MACs filter")
            (@arg add_white_list: -w --white +takes_value +multiple "Add MACs into white list")
            (@arg add_black_list: -b --black +takes_value +multiple "Add MACs into black list, default is <current_exe>/macs.txt")
            (@arg black_list_file: -f +takes_value "Get black list from file")
            (@arg show_white_list: -W --wl "Show MACs from white list")
            (@arg show_black_list: -B --bl "Show MACs from black list")
            (@arg logout: -o --logout "Perform logout action then exit")
            (@arg reboot: -t --reboot "Perform reboot action then exit")
            (@arg verbose: -v --verbose "Output more details")
        ).get_matches()
    }
}
