mod app;
mod window;

use gtk4::prelude::*;
use libadwaita as adw;

fn main() {
    let application = adw::Application::builder()
        .application_id("com.fluxara.Store")
        .build();

    application.connect_activate(|app| {
        let window = window::FluxaraWindow::new(app);
        window.present();
    });

    application.run();
}
