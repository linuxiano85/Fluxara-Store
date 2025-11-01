use crate::app::FluxaraApp;
use adw::subclass::prelude::*;
use gtk4::prelude::*;
use gtk4::{self as gtk, Orientation};
use libadwaita as adw;

pub struct FluxaraWindow {
    window: adw::ApplicationWindow,
}

impl FluxaraWindow {
    pub fn new(app: &adw::Application) -> adw::ApplicationWindow {
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Fluxara Store")
            .default_width(1200)
            .default_height(800)
            .build();

        let header_bar = adw::HeaderBar::new();

        let tab_view = adw::TabView::new();

        // Create tabs
        let home_page = Self::create_home_page();
        tab_view.append(&home_page);
        tab_view.get_page(&home_page).set_title("Home");

        let updates_page = Self::create_updates_page();
        tab_view.append(&updates_page);
        tab_view.get_page(&updates_page).set_title("Updates");

        let drivers_page = Self::create_drivers_page();
        tab_view.append(&drivers_page);
        tab_view.get_page(&drivers_page).set_title("Drivers");

        let maintenance_page = Self::create_maintenance_page();
        tab_view.append(&maintenance_page);
        tab_view
            .get_page(&maintenance_page)
            .set_title("Maintenance");

        let settings_page = Self::create_settings_page();
        tab_view.append(&settings_page);
        tab_view.get_page(&settings_page).set_title("Settings");

        let tab_bar = adw::TabBar::new();
        tab_bar.set_view(Some(&tab_view));

        let content = gtk::Box::new(Orientation::Vertical, 0);
        content.append(&header_bar);
        content.append(&tab_bar);
        content.append(&tab_view);

        window.set_content(Some(&content));

        window
    }

    fn create_home_page() -> gtk::Box {
        let page = gtk::Box::new(Orientation::Vertical, 12);
        page.set_margin_top(24);
        page.set_margin_bottom(24);
        page.set_margin_start(24);
        page.set_margin_end(24);

        let title = gtk::Label::new(Some("Welcome to Fluxara Store"));
        title.add_css_class("title-1");
        page.append(&title);

        let subtitle = gtk::Label::new(Some("Universal package manager for Linux"));
        subtitle.add_css_class("dim-label");
        page.append(&subtitle);

        let search_entry = gtk::SearchEntry::new();
        search_entry.set_placeholder_text(Some("Search for applications..."));
        page.append(&search_entry);

        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_vexpand(true);

        let list_box = gtk::ListBox::new();
        list_box.add_css_class("boxed-list");
        scrolled.set_child(Some(&list_box));

        page.append(&scrolled);

        page
    }

    fn create_updates_page() -> gtk::Box {
        let page = gtk::Box::new(Orientation::Vertical, 12);
        page.set_margin_top(24);
        page.set_margin_bottom(24);
        page.set_margin_start(24);
        page.set_margin_end(24);

        let title = gtk::Label::new(Some("Available Updates"));
        title.add_css_class("title-2");
        page.append(&title);

        let update_all_button = gtk::Button::with_label("Update All");
        update_all_button.add_css_class("suggested-action");
        page.append(&update_all_button);

        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_vexpand(true);

        let list_box = gtk::ListBox::new();
        list_box.add_css_class("boxed-list");
        scrolled.set_child(Some(&list_box));

        page.append(&scrolled);

        page
    }

    fn create_drivers_page() -> gtk::Box {
        let page = gtk::Box::new(Orientation::Vertical, 12);
        page.set_margin_top(24);
        page.set_margin_bottom(24);
        page.set_margin_start(24);
        page.set_margin_end(24);

        let title = gtk::Label::new(Some("Hardware Drivers"));
        title.add_css_class("title-2");
        page.append(&title);

        let detect_button = gtk::Button::with_label("Detect Hardware");
        page.append(&detect_button);

        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_vexpand(true);

        let list_box = gtk::ListBox::new();
        list_box.add_css_class("boxed-list");
        scrolled.set_child(Some(&list_box));

        page.append(&scrolled);

        page
    }

    fn create_maintenance_page() -> gtk::Box {
        let page = gtk::Box::new(Orientation::Vertical, 12);
        page.set_margin_top(24);
        page.set_margin_bottom(24);
        page.set_margin_start(24);
        page.set_margin_end(24);

        let title = gtk::Label::new(Some("System Maintenance"));
        title.add_css_class("title-2");
        page.append(&title);

        let group = gtk::Box::new(Orientation::Vertical, 6);

        let clean_cache_button = gtk::Button::with_label("Clean Package Cache");
        group.append(&clean_cache_button);

        let remove_orphans_button = gtk::Button::with_label("Remove Orphaned Packages");
        group.append(&remove_orphans_button);

        let test_mirrors_button = gtk::Button::with_label("Test Mirror Speeds");
        group.append(&test_mirrors_button);

        page.append(&group);

        page
    }

    fn create_settings_page() -> gtk::Box {
        let page = gtk::Box::new(Orientation::Vertical, 12);
        page.set_margin_top(24);
        page.set_margin_bottom(24);
        page.set_margin_start(24);
        page.set_margin_end(24);

        let title = gtk::Label::new(Some("Settings"));
        title.add_css_class("title-2");
        page.append(&title);

        let preferences_group = adw::PreferencesGroup::new();
        preferences_group.set_title("User Interface");

        let tray_row = adw::ActionRow::new();
        tray_row.set_title("Enable Tray Icon");
        tray_row.set_subtitle("Show system tray icon for background updates");
        let tray_switch = gtk::Switch::new();
        tray_switch.set_active(true);
        tray_switch.set_valign(gtk::Align::Center);
        tray_row.add_suffix(&tray_switch);
        preferences_group.add(&tray_row);

        page.append(&preferences_group);

        let repos_group = adw::PreferencesGroup::new();
        repos_group.set_title("Repositories");

        let flathub_beta_row = adw::ActionRow::new();
        flathub_beta_row.set_title("Flathub Beta");
        flathub_beta_row.set_subtitle("Enable Flathub beta repository");
        let flathub_beta_switch = gtk::Switch::new();
        flathub_beta_switch.set_active(true);
        flathub_beta_switch.set_valign(gtk::Align::Center);
        flathub_beta_row.add_suffix(&flathub_beta_switch);
        repos_group.add(&flathub_beta_row);

        let aur_row = adw::ActionRow::new();
        aur_row.set_title("AUR (Arch User Repository)");
        aur_row.set_subtitle("Enable AUR support (Arch/Manjaro only)");
        let aur_switch = gtk::Switch::new();
        aur_switch.set_valign(gtk::Align::Center);
        aur_row.add_suffix(&aur_switch);
        repos_group.add(&aur_row);

        page.append(&repos_group);

        let privacy_group = adw::PreferencesGroup::new();
        privacy_group.set_title("Privacy");

        let telemetry_row = adw::ActionRow::new();
        telemetry_row.set_title("Anonymous Telemetry");
        telemetry_row.set_subtitle("Help improve Fluxara Store by sending anonymous usage data");
        let telemetry_switch = gtk::Switch::new();
        telemetry_switch.set_active(false);
        telemetry_switch.set_valign(gtk::Align::Center);
        telemetry_row.add_suffix(&telemetry_switch);
        privacy_group.add(&telemetry_row);

        page.append(&privacy_group);

        page
    }

    pub fn present(&self) {
        self.window.present();
    }
}
