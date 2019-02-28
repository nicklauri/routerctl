use super::*;

fn test_regex() {
    println!("MAC filter: INVALID");
    let macflt_html = include_str!("../Cargo.lock");
    MACFLT_STATUS.captures_iter(macflt_html).for_each(|mat| {
        println!("MACFLT_STATUS: mat: {:?}", mat);
    });

    MACFLT_LIST.captures_iter(macflt_html).for_each(|mat| {
        println!("MACFLT_LIST: mat: {:?}", mat);
    });

    MACFLT_MODE.captures_iter(macflt_html).for_each(|mat| {
        println!("MACFLT_MODE: mat: {:?}", mat);
    });

    println!("\nMAC filter: NO ENTRY");
    let macflt_html = include_str!("../macflt.html");
    MACFLT_STATUS.captures_iter(macflt_html).for_each(|mat| {
        println!("MACFLT_STATUS: mat: {:?}", mat);
    });

    MACFLT_LIST.captures_iter(macflt_html).for_each(|mat| {
        println!("MACFLT_LIST: mat: {:?}", mat);
    });

    MACFLT_MODE.captures_iter(macflt_html).for_each(|mat| {
        println!("MACFLT_MODE: mat: {:?}", mat);
    });

    println!("\nMAC filter: HAS ENTRY");
    let macflt_html = include_str!("../macflt.success.html");
    MACFLT_STATUS.captures_iter(macflt_html).for_each(|mat| {
        println!("MACFLT_STATUS: mat: {:?}", mat);
    });

    MACFLT_LIST.captures_iter(macflt_html).for_each(|mat| {
        println!("MACFLT_LIST: mat: {:?}", mat);
    });

    MACFLT_MODE.captures_iter(macflt_html).for_each(|mat| {
        println!("MACFLT_MODE: mat: {:?}", mat);
    });

    println!("\n MAC validate: correct == {} ; incorrect == {}",
        MAC_VALIDATE.is_match("Ac:5f:04:db:ea:49"),
        MAC_VALIDATE.is_match("hello:world:94859"));
}
