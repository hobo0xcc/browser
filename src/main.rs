#![feature(asm, panic_info_message, fn_align)]
#![no_std]
#![cfg_attr(feature = "no-std", no_main)]

extern crate alloc;
extern crate browser;
extern crate libm;
extern crate rusttype;

use alloc::string::ToString;
#[cfg(feature = "no-std")]
use browser::syscall::*;
use browser::*;
use browser::{css, html, layout, style};
use core::slice;

#[cfg(feature = "std")]
use fltk::{prelude::*, *};

#[cfg(feature = "std")]
fn main() {
    let app = app::App::default();

    let width: i32 = 1000;
    let height: i32 = 700;
    let mut my_window = window::Window::new(100, 100, width, height, "My Window");
    my_window.set_color(enums::Color::White);

    let mut frame = frame::Frame::default().with_size(width, height).center_of(&my_window);

    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = width as f32;
    viewport.content.height = height as f32;

    let root_node = html::parse(HTML.to_string());
    let stylesheet = css::parse(CSS.to_string());
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);

    let size = ((width * height) * 4) as usize;
    let layout = Layout::from_size_align(size, 8).unwrap();
    let buffer = unsafe { alloc_zeroed(layout) };
    let slice = unsafe { slice::from_raw_parts_mut(buffer, size) };
    let mut renderer = render::Renderer::new(slice, width as u32, height as u32);

    renderer.render(&layout_root);
    draw::draw_rgba(&mut frame, renderer.pixmap.data_mut()).unwrap();

    my_window.end();
    my_window.show();
    app.run().unwrap();
}

#[repr(align(4096))]
#[no_mangle]
#[cfg(feature = "no-std")]
pub unsafe extern "C" fn _entry() {
    browser::allocator::init();
    let w_width = 600_usize;
    let w_height = 400_usize;
    let frame_addr = 0x10000000;
    let window_title = "browser";
    let wid = sys_create_window(
        window_title.as_bytes().as_ptr(),
        window_title.len(),
        100,
        100,
        w_width,
        w_height,
    );
    sys_map_window(wid, frame_addr);
    let frame_ptr = frame_addr as *mut u32;
    for y in 0..w_height {
        for x in 0..w_width {
            frame_ptr.add(y * w_width + x).write(0xffffffff);
        }
    }

    let buffer_slice =
        slice::from_raw_parts_mut(frame_ptr as *mut u8, (w_width * w_height * 4) as usize);

    sys_sync_window(wid);

    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = w_width as f32;
    viewport.content.height = w_height as f32;

    let root_node = html::parse(HTML.to_string());
    let stylesheet = css::parse(CSS.to_string());
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);
    let mut renderer = render::Renderer::new(buffer_slice, w_width as u32, w_height as u32);
    renderer.render(&layout_root);
    sys_sync_window(wid);
    println!("Hello");
}

#[panic_handler]
#[cfg(feature = "no-std")]
unsafe fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Aborting: ");
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        println!("no information available.");
    }

    loop {}
}
