extern crate gtk;

use cairo::{RectangleInt, Region};
use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn main() {
    let app =
        gtk::Application::new(Some("io.github.youxkei.mousecross"), Default::default()).unwrap();

    app.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);
        win.set_title("Mouse Cross");

        let rectangles = &[
            RectangleInt {
                x: 0,
                y: 0,
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

        win.add_events(gdk::EventMask::POINTER_MOTION_MASK);
        win.connect_motion_notify_event(
            |_window: &gtk::ApplicationWindow, event: &gdk::EventMotion| {
                println!("{:?}", event);

                gtk::Inhibit(false)
            },
        );
    });

    app.run(&args().collect::<Vec<_>>());
}
