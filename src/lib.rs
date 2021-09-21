#![feature(asm, panic_info_message, fn_align, alloc_error_handler)]
#![no_std]
// #![no_main]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

extern crate hashbrown;
extern crate libm;
extern crate rusttype;
extern crate tiny_skia;

pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod render;
pub mod style;

#[cfg(feature = "no-std")]
pub mod allocator;
#[cfg(feature = "no-std")]
pub mod float;
#[cfg(feature = "no-std")]
pub mod syscall;
#[cfg(feature = "no-std")]
pub mod writer;

#[cfg(feature = "no-std")]
pub use writer::*;

pub static HTML: &str = include_str!("../resources/index.html");
pub static CSS: &str = include_str!("../resources/style.css");
pub static MPLUS_FONT: &[u8] = include_bytes!("../resources/MPLUS1p-Medium.ttf");
