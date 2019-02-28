
use std::io::{Error, ErrorKind, Write};

use regex::Regex;
use lazy_static::lazy_static;
use prettytable::{Cell, Row, Table, row, cell, format};

use crate::commandline::ARGS;
use crate::client;

macro_rules! onoff {
    ($x:expr) => (if $x { "on" } else { "off" })
}

lazy_static! {
    pub static ref LOGIN_VALIDATE: Regex= Regex::new(r"login.html").unwrap();
    pub static ref MAC_VALIDATE: Regex = Regex::new(r"^([a-fA-F0-9]{2}:){5}[a-fA-F0-9]{2}$").unwrap();

    pub static ref CLIENTS_LIST: Regex = Regex::new(r"client_list\[\d+\]='([^']+)';").unwrap();

    pub static ref MACFLT_STATUS: Regex = Regex::new(r"MacFltEnable=([01]);").unwrap();
    pub static ref MACFLT_MODE: Regex = Regex::new(r"MacFltMode=([01]);").unwrap();
    pub static ref MACFLT_LIST: Regex = Regex::new(r#"Mac:"([^"]+)""#).unwrap();
}

pub fn login() -> Result<(), Error> {
    if LOGIN_VALIDATE.is_match(&client::post("/GponForm/LoginForm",
        &("XWebPageName=index&username=admin&password=".to_string() + ARGS.password))?)
    {
        return Err(Error::new(ErrorKind::Other, "wrong password or someone logged in"));
    }
    Ok(())
}

pub fn logout() -> Result<(), Error> {
    client::get("/logout.html")?;
    Ok(())
}

pub fn reboot() -> Result<(), Error> {
    client::post("/GponForm/reboot_XForm", "XWebPageName=reboot&admin_action=reboot")?;
    Ok(())
}

pub fn macflt_enable() -> Result<(), Error> {
    // entries at the most is couples dozen, doesn't need to optimize so much
    // if it's ok if 2 same macs
    let mut list: Vec<&str> = Vec::new();
    if let Some(l) = &ARGS.black_list_file {
        l.iter().for_each(|m| {
            list.push(m);
        });
    }

    if let Some(l) = &ARGS.add_black_list {
        l.iter().for_each(|m| {
            list.push(&m);
        });
    }

    if list.is_empty() {
        eprintln!("routerctl: WARN: no MAC given.");
        return Ok(());
    }
    
    list.sort_unstable();
    list.dedup();

    if ARGS.verbose {
        println!("routerctl: INFO: MACs are going to be sent:");
        list.iter().for_each(|m| {
            println!(" - {}", m);
        });
        print!("routerctl: INFO: confirm all the MACs, Ctrl-C to cancel.");
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut String::new());
    }

    client::post("/GponForm/mac_filter_XForm",
        &format!("macfltenable=on&macfltlist=0-{}&macfltmode=0&XWebPageName=mac_filter",
            list.join("-.-0-")))?;

    if ARGS.verbose {
        println!("routerctl: INFO: sent successfully.");
    }
    Ok(())
}

pub fn macflt_disable() -> Result<(), Error> {
    client::post("/GponForm/mac_filter_XForm", "macfltlist=&macfltmode=0&XWebPageName=mac_filter")?;
    Ok(())
}

pub fn macflt_status() -> Result<(), Error> {
    let macflt_html = client::get("/mac_filter.html")?;

    let status = match MACFLT_STATUS.captures(&macflt_html) {
        Some(thing) => if &thing[1] == "1" { true } else { false },
        None => return Err(Error::new(ErrorKind::Other, "match MACFLT_STATUS failed"))
    };

    let mode = match MACFLT_MODE.captures(&macflt_html) {
        Some(thing) => if &thing[1] == "1" { true } else { false },
        None => return Err(Error::new(ErrorKind::Other, "match MACFLT_MODE failed"))
    };

    // assume that router will always work correctly
    let list =  MACFLT_LIST.captures_iter(&macflt_html)
        .map(|c| { c[1].to_string() }).collect::<Vec<String>>();

    print!(concat!("routerctl: mac filter status:\n",
        "  - status: {}\n",
        "  - mode  : {}\n",
        "  - list  :"), onoff!(status),
        if !mode { "black-list" } else { "white-list" });

    if list.is_empty() {
        println!(" <empty>");
    }
    else {
        println!();
        list.iter().for_each(|m| println!("     {}", m));
    }
    Ok(())
}

pub fn active_clients() -> Result<(), Error> {
    let laninfo_html = client::get("/laninfo.html")?;

    let list: Vec<Vec<Cell>> = CLIENTS_LIST.captures_iter(&laninfo_html).map(|c| {
        c[1].to_string().split("|").map(Cell::new).collect()
    }).collect();

    let mut table = Table::new();
    table.add_row(row!["ID", "Hostname", "MAC Address", "IP Address", "Conn-Type", "Uptime"]);

    list.iter().for_each(|vc| {
        table.add_row(Row::new(vc.to_vec()));
    });

    table.set_format(*format::consts::FORMAT_CLEAN);
    table.printstd();
    Ok(())
}
