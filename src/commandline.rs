
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
    pub static ref ARGS: Arc<Args> = Arc::new(Args::parse());
    pub static ref PARSER: Arc<ArgMatches<'static>> = Arc::new(Args::initial());
    pub static ref DEFAULT_BLACK_LIST: String = {
            let mut current_dir = std::env::current_exe().unwrap();
            current_dir.pop();
            current_dir.push("macs.txt");
            if current_dir.is_file() {
                current_dir.to_str().unwrap().to_string()
            }
            else {
                String::new()
            }
        };
}

impl Args {
    pub fn parse() -> Self {
        let mut args: Args = Default::default();
        if let Some(p) = PARSER.value_of("password") {
            args.password = p;
        }
        else {
            args.password = "admin";
        }

        if let Some(iter) = PARSER.values_of("add_white_list") {
            iter.clone().for_each(|m| {
                if !super::router::MAC_VALIDATE.is_match(&m) {
                    panic!(format!("routerctl::Args::parse: ERR: invalid MAC: '{}'", m));
                }
            });

            args.add_white_list = Some(iter.collect());
        }

        if let Some(iter) = PARSER.values_of("add_black_list") {
            iter.clone().for_each(|m| {
                if !super::router::MAC_VALIDATE.is_match(&m) {
                    panic!(format!("routerctl::Args::parse: ERR: invalid MAC: '{}'", m));
                }
            });

            args.add_black_list = Some(iter.collect());
        }

        if let Some(f) = PARSER.value_of("black_list_file").or(Some(&*DEFAULT_BLACK_LIST)) {
            if Path::new(f).is_file() {
                // str.split_whitespace > str.lines + str.trim
                // iter.filter is simple but less details
                let mut list = vec![];
                String::from_utf8_lossy(&std::fs::read(f).expect(&format!("can't read '{}'", f)))
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
            else {
                println!("routerctl: ERR: '{}' is not a file.", f);
            }
        }

        if let Some(v) = PARSER.value_of("router") {
            args.router = v;
        }
        else {
            args.router = super::ROUTER_DEFAULT_ADDR;
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
