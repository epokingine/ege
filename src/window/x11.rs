use crate::log::error;
use std::{
    ffi::{c_int, c_ulong, CString},
    ptr,
};
use image::{imageops::FilterType, DynamicImage, GenericImageView};
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

            let context = XCreateGC(display, window_addr, 0, ptr::null_mut());

            XSetForeground(display, context, 0xffffff);
            return Ok(X11Window {
                window_addr: window_addr,
                x11_display: display,
                x11_context: context,
            });
        }
    }

    pub fn draw_pixel(&self, color: &RGBColor, x: u32, y: u32) {
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

    pub fn draw_circ(&self, color: &RGBColor, x: i32, y: i32, radius: i32) {
        let mut r = radius;

        while r != 0 {
            let mut dx = 0;
            let mut dy = r;
            let mut p = 1 - r;

            while dx <= dy {
                self.draw_pixel(color, (x + dx) as u32, (y + dy) as u32);
                self.draw_pixel(color, (x + dy) as u32, (y + dx) as u32);
                self.draw_pixel(color, (x - dx) as u32, (y + dy) as u32);
                self.draw_pixel(color, (x - dy) as u32, (y + dx) as u32);

                self.draw_pixel(color, (x + dx) as u32, (y - dy) as u32);
                self.draw_pixel(color, (x + dy) as u32, (y - dx) as u32);
                self.draw_pixel(color, (x - dx) as u32, (y - dy) as u32);
                self.draw_pixel(color, (x - dy) as u32, (y - dx) as u32);

                if p < 0 {
                    p += 2 * dx + 3;
                } else {
                    p += 2 * (dx - dy) + 5;
                    dy -= 1;
                }

                dx += 1;
            }
            r -= 1;
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

    /// Draws an image, ``sx`` and ``sy`` sets the size of the image, if ``sx`` and ``sy`` are set to ``0`` the size of the image will not change
    pub fn draw_image(&self, x: u32, y: u32, sx: u32, sy: u32, image: DynamicImage) {
        let image = if sx == 0 && sy == 0 {
            image
        } else {
            image.resize(sx, sy, FilterType::Nearest)
        };
        let image_size = image.dimensions();

        for dy in 0..image_size.1 {
            for dx in 0..image_size.0 {
                let pixel_color = image.get_pixel(dx, dy);
                let color = RGBColor(pixel_color[0], pixel_color[1], pixel_color[2]);
                self.draw_pixel(&color, x + dx, y + dy);
            }
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
