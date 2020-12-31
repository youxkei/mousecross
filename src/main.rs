extern crate gtk;

use cairo::{RectangleInt, Region};
use gio::prelude::*;
use gtk::prelude::*;

use x11::xlib::*; // TODO remove asterisk

use std::cell::RefCell;
use std::env::{args, var};
use std::option::Option;
use std::ptr::null;
use std::rc::Rc;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let radius = 128;

    let app =
        gtk::Application::new(Some("io.github.youxkei.mousecross"), Default::default()).unwrap();

    glib::MainContext::default().acquire();
    let (cast_tx, cast_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let (screen_width, screen_height) = unsafe {
        let display = XOpenDisplay(null());
        let screen = XDefaultScreen(display);

        let dimension = (
            XDisplayWidth(display, screen),
            XDisplayHeight(display, screen),
        );

        XCloseDisplay(display);

        dimension
    };

    let (screen_width, screen_height) = (256, 256);

    spawn(move || unsafe {
        let display = XOpenDisplay(null());
        let root_window = XDefaultRootWindow(display);

        loop {
            let mut root_return = 0;
            let mut child_return = 0;
            let mut root_x_return = 0;
            let mut root_y_return = 0;
            let mut child_x_return = 0;
            let mut child_y_return = 0;
            let mut mask_return = 0;

            XQueryPointer(
                display,
                root_window,
                &mut root_return,
                &mut child_return,
                &mut root_x_return,
                &mut root_y_return,
                &mut child_x_return,
                &mut child_y_return,
                &mut mask_return,
            );

            cast_tx.send((root_x_return, root_y_return));

            sleep(Duration::from_millis(1));
        }
    });

    let window_cell: Rc<RefCell<Option<gtk::ApplicationWindow>>> = Rc::new(RefCell::new(None));

    cast_rx.attach(None, {
        let window_cell = window_cell.clone();
        move |(x, y)| {
            match &*window_cell.borrow() {
                None => {}
                Some(window) => window.move_(x - screen_width, y - screen_height),
            }

            glib::Continue(true)
        }
    });

    app.connect_activate({
        let window_cell = window_cell.clone();

        move |app| {
            let win = gtk::ApplicationWindow::new(app);
            win.set_title("Mouse Cross");
            win.set_wmclass("mousecross", "mousecross");
            win.set_default_size(screen_width * 2, screen_height * 2);

            let mut region = Region::create_rectangle(&RectangleInt {
                x: 0,
                y: 0,
                width: screen_width * 2,
                height: screen_height * 2,
            });

            region.subtract_rectangle(&RectangleInt {
                x: screen_width - radius,
                y: screen_height - radius,
                width: radius * 2,
                height: radius * 2,
            });

            win.shape_combine_region(Some(&region));

            win.show_all();

            window_cell.borrow_mut().replace(win.clone());
        }
    });

    app.run(&args().collect::<Vec<_>>());
}
