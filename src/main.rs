use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Box as GtkBox, Entry, Orientation};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

mod utils;
use utils::{
    applications::{filter_applications, list_applications},
    css,
};

#[derive(Debug, Clone)]
pub struct AppInfo {
    name: String,
    exec: String,
    icon: Option<String>,
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.LayerPanel")
        .build();

    app.connect_activate(|app| {
        let list_applications: Vec<AppInfo> = list_applications();

        css::load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(600)
            .title("Panel de apps")
            .build();

        LayerShell::init_layer_shell(&window);
        window.set_layer(Layer::Top);
        window.set_keyboard_mode(KeyboardMode::OnDemand);
        window.set_focusable(true);
        window.auto_exclusive_zone_enable();
        window.set_anchor(Edge::Right, false);
        window.set_anchor(Edge::Left, false);
        window.set_anchor(Edge::Top, false);
        window.set_anchor(Edge::Bottom, false);

        let main_box = GtkBox::new(Orientation::Vertical, 0);

        let vbox = GtkBox::new(Orientation::Vertical, 0);
        vbox.set_halign(Align::Fill);
        vbox.set_valign(Align::Start);

        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some("Buscar..."));
        search_entry.set_hexpand(true);
        search_entry.connect_changed({
            let vbox = vbox.clone();
            let applications = list_applications.clone();
            move |entry| {
                let search_text = entry.text().to_string();

                filter_applications(&search_text, &vbox, &applications);
            }
        });
        main_box.append(&search_entry);
        let scrolled_window = gtk::ScrolledWindow::builder()
            .min_content_width(220)
            .min_content_height(600)
            .child(&vbox)
            .build();

        filter_applications("", &vbox, &list_applications);
        main_box.append(&scrolled_window);
        window.set_child(Some(&main_box));
        window.show();
    });

    app.run();
}
