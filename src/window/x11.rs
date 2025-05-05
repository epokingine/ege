use crate::log::error;
use std::{
    ffi::{c_int, c_ulong, CString},
    ptr,
};
use x11::xlib::*;

use super::color::RGBColor;

pub struct X11Window {
    window_addr: u64,
    x11_display: *mut _XDisplay,
    x11_context: *mut _XGC,
}

impl X11Window {
    pub fn new(width: u32, height: u32, name: &str) -> Result<Self, ()> {
        unsafe {
            let display = XOpenDisplay(ptr::null());
            if display.is_null() {
                error!("Could not open X display");
                return Err(());
            }

            let screen = XDefaultScreen(display);
            let root_window = XRootWindow(display, screen);

            let window_addr = XCreateSimpleWindow(
                display,
                root_window,
                0,
                0,
                width,
                height,
                1,
                XBlackPixel(display, screen),
                XWhitePixel(display, screen),
            );

            let title = CString::new(name).unwrap();
            XStoreName(display, window_addr, title.as_ptr());

            let r = XMapWindow(display, window_addr);
            println!("map win: {}", r);

            let context = XCreateGC(display, window_addr, 0, ptr::null_mut());

            XSetForeground(display, context, 0xffffff);
            return Ok(X11Window {
                window_addr: window_addr,
                x11_display: display,
                x11_context: context,
            });
        }
    }

    pub fn draw_pixel(&self, color: RGBColor, x: u32, y: u32) {
        self.set_color(color.as_decimal());
        unsafe {
            XDrawPoint(
                self.x11_display,
                self.window_addr,
                self.x11_context,
                x as c_int,
                y as c_int,
            );
            XFlush(self.x11_display);
        }
    }

    pub fn draw_rect(&self, color: RGBColor, x: u32, y: u32, w: u32, h: u32) {
        self.set_color(color.as_decimal());
        for ry in 0..h {
            for rx in 0..w {
                unsafe {
                    XDrawPoint(
                        self.x11_display,
                        self.window_addr,
                        self.x11_context,
                        (x + rx) as c_int,
                        (y + ry) as c_int,
                    );
                }
            }
        }

        unsafe {
            XFlush(self.x11_display);
        }
    }

    fn set_color(&self, color: u32) {
        unsafe {
            XSetForeground(self.x11_display, self.x11_context, color as c_ulong);
        }
    }

    pub fn clear_window(&self) {
        unsafe {
            XClearWindow(self.x11_display, self.window_addr);
        }
    }

    pub fn close_window(&self) {
        unsafe {
            XFreeGC(self.x11_display, self.x11_context);
            XDestroyWindow(self.x11_display, self.window_addr);
            XCloseDisplay(self.x11_display);
        }
    }
}
