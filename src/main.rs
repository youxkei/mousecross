extern crate gtk;

use cairo::{RectangleInt, Region};
use gio::prelude::*;
use gtk::prelude::*;
use x11::xlib::{XDefaultRootWindow, XOpenDisplay, XQueryPointer};

use std::cell::RefCell;
use std::env::{args, var};
use std::option::Option;
use std::ptr::null;
use std::rc::Rc;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let app =
        gtk::Application::new(Some("io.github.youxkei.mousecross"), Default::default()).unwrap();

    glib::MainContext::default().acquire();
    let (cast_tx, cast_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

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
            match &*window_cell.borrow_mut() {
                None => {}
                Some(window) => window.move_(x, y),
            }

            glib::Continue(true)
        }
    });

    app.connect_activate({
        let window_cell = window_cell.clone();

        move |app| {
            let win = gtk::ApplicationWindow::new(app);
            win.set_title("Mouse Cross");

            let rectangles = &[
                RectangleInt {
                    x: 10,
                    y: 10,
                    width: 128,
                    height: 128,
                },
                RectangleInt {
                    x: 64,
                    y: 64,
                    width: 128,
                    height: 128,
                },
            ];

            win.shape_combine_region(Some(&Region::create_rectangles(rectangles)));

            win.show_all();

            window_cell.borrow_mut().replace(win.clone());
        }
    });

    app.run(&args().collect::<Vec<_>>());
}
