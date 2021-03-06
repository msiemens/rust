// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//ignore-android
//ignore-freebsd
//ignore-ios
//ignore-dragonfly

#![feature(asm)]

use std::old_io::process::Command;
use std::os;
use std::thread::Thread;

// lifted from the test module
// Inlining to avoid llvm turning the recursive functions into tail calls,
// which doesn't consume stack.
#[inline(always)]
#[no_stack_check]
pub fn black_box<T>(dummy: T) { unsafe { asm!("" : : "r"(&dummy)) } }

#[no_stack_check]
fn recurse() {
    let buf = [0; 10];
    black_box(buf);
    recurse();
}

fn main() {
    let args = os::args();
    let args = args;
    if args.len() > 1 && args[1] == "recurse" {
        let _t = Thread::scoped(recurse);
    } else {
        let recurse = Command::new(&args[0]).arg("recurse").output().unwrap();
        assert!(!recurse.status.success());
        let error = String::from_utf8_lossy(&recurse.error);
        println!("wut");
        println!("`{}`", error);
        assert!(error.contains("has overflowed its stack"));
    }
}
