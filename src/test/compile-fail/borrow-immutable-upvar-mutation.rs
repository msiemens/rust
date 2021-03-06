// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(unboxed_closures)]

// Tests that we can't assign to or mutably borrow upvars from `Fn`
// closures (issue #17780)

fn set(x: &mut usize) { *x = 5; }

fn to_fn<A,F:Fn<A>>(f: F) -> F { f }
fn to_fn_mut<A,F:FnMut<A>>(f: F) -> F { f }

fn main() {
    // By-ref captures
    {
        let mut x = 0us;
        let _f = to_fn(|| x = 42); //~ ERROR cannot assign

        let mut y = 0us;
        let _g = to_fn(|| set(&mut y)); //~ ERROR cannot borrow

        let mut z = 0us;
        let _h = to_fn_mut(|| { set(&mut z); to_fn(|| z = 42); }); //~ ERROR cannot assign
    }

    // By-value captures
    {
        let mut x = 0us;
        let _f = to_fn(move || x = 42); //~ ERROR cannot assign

        let mut y = 0us;
        let _g = to_fn(move || set(&mut y)); //~ ERROR cannot borrow

        let mut z = 0us;
        let _h = to_fn_mut(move || { set(&mut z); to_fn(move || z = 42); }); //~ ERROR cannot assign
    }
}
