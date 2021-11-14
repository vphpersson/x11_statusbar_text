use std::{fs, thread, time, ffi::CString, ptr, cmp};

use x11::xlib::{XDefaultScreenOfDisplay, XFlush, XOpenDisplay, XRootWindowOfScreen, XStoreName};
use chrono::Local;

fn main() {
    unsafe {
        let display = XOpenDisplay(ptr::null_mut());
        let screen = XDefaultScreenOfDisplay(display);
        let root = XRootWindowOfScreen(screen);

        let sleep_time = time::Duration::from_millis(50);

        loop {
            let date_str = Local::now().format("%A %Y-%m-%d %H:%M").to_string();
            let battery_level = cmp::min(
                fs::read_to_string("/sys/class/power_supply/BAT0/capacity").expect("Battery status could not be read.").trim().parse::<i32>().expect("Could not parse battery level as an integer."),
                100
            );

            XStoreName(display, root, CString::new(format!(" {} | {: >3}% ", date_str, battery_level)).unwrap().as_ptr() as *const i8);
            XFlush(display);

            thread::sleep(sleep_time);
        }
    }
}
