#![allow(unused_variables)]

use std::thread;
use std::thread::Thread;

fn main(){
    let outer_scope = 412;

    let join_handle = thread::spawn( move || {
        outer_scope * 2
    });

    let result = join_handle.join();
    match result {
        Ok(value) => {
            println!("{}", value);
        }
        Err(_) => {}
    }
}
