extern crate gtk;

use cairo::{RectangleInt, Region};
use gio::prelude::*;
use glib::MainContext;
use gtk::prelude::*;
use gtk::{Application, CssProvider};

use x11::xlib::{
    XCloseDisplay, XDefaultRootWindow, XDefaultScreen, XDisplayHeight, XDisplayWidth, XOpenDisplay,
    XQueryPointer,
};

use std::cell::RefCell;
use std::env::args;
use std::option::Option;
use std::ptr::null;
use std::rc::Rc;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let radius = 64;
    let width = 2;
    let center_radius = 256;

    let app = Application::new(Some("io.github.youxkei.mousecross"), Default::default()).unwrap();

    MainContext::default().acquire();
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

            cast_tx.send((root_x_return, root_y_return)).unwrap();

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
            win.set_accept_focus(false);
            win.stick();

            let css_provider = CssProvider::new();
            css_provider
                .load_from_data(b"window { background-color: rgb(255, 0, 0); }")
                .unwrap();
            win.get_style_context()
                .add_provider(&css_provider, std::u32::MAX);

            let region = Region::create_rectangle(&RectangleInt {
                x: 0,
                y: 0,
                width: screen_width * 2,
                height: screen_height * 2,
            });

            let center_region = &RectangleInt {
                x: screen_width - center_radius,
                y: screen_height - center_radius,
                width: center_radius * 2,
                height: center_radius * 2,
            };
            let top_left_region = &RectangleInt {
                x: 0,
                y: 0,
                width: screen_width - radius,
                height: screen_height - radius,
            };
            let top_right_region = &RectangleInt {
                x: screen_width + radius,
                y: 0,
                width: screen_width - radius,
                height: screen_height - radius,
            };
            let bottom_left_region = &RectangleInt {
                x: 0,
                y: screen_height + radius,
                width: screen_width - radius,
                height: screen_height - radius,
            };
            let bottom_right_region = &RectangleInt {
                x: screen_width + radius,
                y: screen_height + radius,
                width: screen_width - radius,
                height: screen_height - radius,
            };
            let top_region = &RectangleInt {
                x: screen_width - radius + width,
                y: 0,
                width: (radius - width) * 2,
                height: screen_height - center_radius - width,
            };
            let bottom_region = &RectangleInt {
                x: screen_width - radius + width,
                y: screen_height + center_radius + width,
                width: (radius - width) * 2,
                height: screen_height - center_radius - width,
            };
            let left_region = &RectangleInt {
                x: 0,
                y: screen_height - radius + width,
                width: screen_width - center_radius - width,
                height: (radius - width) * 2,
            };
            let right_region = &RectangleInt {
                x: screen_width + center_radius + width,
                y: screen_height - radius + width,
                width: screen_width - center_radius - width,
                height: (radius - width) * 2,
            };

            region.subtract_rectangle(&center_region).unwrap();
            region.subtract_rectangle(&top_left_region).unwrap();
            region.subtract_rectangle(&top_right_region).unwrap();
            region.subtract_rectangle(&bottom_left_region).unwrap();
            region.subtract_rectangle(&bottom_right_region).unwrap();
            region.subtract_rectangle(&top_region).unwrap();
            region.subtract_rectangle(&bottom_region).unwrap();
            region.subtract_rectangle(&left_region).unwrap();
            region.subtract_rectangle(&right_region).unwrap();
            win.shape_combine_region(Some(&region));

            win.show_all();

            *window_cell.borrow_mut() = Some(win.clone());
        }
    });

    app.run(&args().collect::<Vec<_>>());
}
