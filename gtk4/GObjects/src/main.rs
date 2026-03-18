mod custom_button;

use gtk4 as gtk;
use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};

const APP_ID: &str = "org.gtk_rs.GObjectSubclassing1";

fn main() {
    // create a new app
    let app = Application::builder().application_id(APP_ID).build();
    
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // create button
    let button = CustomButton::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);
    
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();
    
    window.present();
}