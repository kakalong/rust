// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type="rlib"]

 #[cfg(rpass1)]
pub struct X {
    pub x: u32
}

#[cfg(rpass2)]
pub struct X {
    pub x: i32
}

pub struct EmbedX {
    pub x: X
}

pub struct Y {
    pub y: char
}
