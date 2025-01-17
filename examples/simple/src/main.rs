use std::{
    ffi::{CStr, CString},
    os::raw::c_void,
    ptr::null,
};

use libc::{c_char, c_int};
use libmpv_rs::safe::*;

fn check(error: Option<MpvError>) {
    if let Some(error) = error {
        panic!("mpv API error: {}", error.get_error_string());
    }
}

fn main() {
    let Some(mut ctx) = MpvHandle::new() else {
        panic!("Cannot initialize mpv handle");
    };
    println!("Handle inited");
    check(ctx.set_option("input-default-bindings".into(), Node::String("yes".into())));
    println!("keyboard");
    ctx.set_option("input-vo-keyboard".into(), Node::String("yes".into()));
    println!("vo");
    check(ctx.set_option("osc".into(), Node::Flag(true)));
    println!("osc");
    check(ctx.initialize());
    println!("init");
    let args = std::env::args().collect::<Vec<String>>();

    let res = ctx.command(vec!["loadfile".into(), args[1].clone()], false);
    if let Err(err) = res {
        check(Some(err));
    }

    loop {
        let Some(event) = ctx.wait_event(10000.) else {
            continue;
        };
        println!("event: {}", event.get_event_string());
        if let Event::Shutdown = event {
            break;
        }
    }
}
