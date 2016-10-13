// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// this file defines CGFloat, as well as stubbed data types.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw;

#[cfg(target_pointer_width = "64")]
pub type CGFloat = f64;
#[cfg(not(target_pointer_width = "64"))]
pub type CGFloat = f32;

#[cfg(any(target_arch = "x86",
          target_arch = "arm",
          target_arch = "aarch64"))]
pub type boolean_t = raw::c_int;
#[cfg(target_arch = "x86_64")]
pub type boolean_t = raw::c_uint;

pub type CGError = i32;

pub const kCGImageAlphaNone: u32 = 0;
pub const kCGImageAlphaPremultipliedLast: u32 = 1;
pub const kCGImageAlphaPremultipliedFirst: u32 = 2;
pub const kCGImageAlphaLast: u32 = 3;
pub const kCGImageAlphaFirst: u32 = 4;
pub const kCGImageAlphaNoneSkipLast: u32 = 5;
pub const kCGImageAlphaNoneSkipFirst: u32 = 6;
