use gio::ApplicationExt;
use gtk::{WidgetExt, WindowExt};

fn main() {
    match gtk::Application::new(
        "io.github.youxkei.mousecross",
        gio::APPLICATION_HANDLES_OPEN,
    ) {
        Ok(app) => {
            app.connect_activate(|app| {
                let win = gtk::ApplicationWindow::new(&app);
                win.set_title("Hello Gtk-rs");
                win.show_all();
            });

            app.run(&[""]);
        }
        Err(_) => {
            println!("Application start up error");
        }
    };
}
