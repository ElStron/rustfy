use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Box as GtkBox, Entry, Orientation};
use gtk4_layer_shell::LayerShell;
mod config;
mod utils;
use config::layer_shell_configure;
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
        layer_shell_configure(&window);

        let main_box = GtkBox::new(Orientation::Vertical, 0);

        let vbox = GtkBox::new(Orientation::Vertical, 0);
        vbox.set_halign(Align::Fill);
        vbox.set_valign(Align::Start);

        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some("Buscar..."));
        search_entry.set_hexpand(true);

        // Estado compartido para el índice seleccionado
        use std::cell::RefCell;
        use std::rc::Rc;
        let selected_index = Rc::new(RefCell::new(0));
        let filtered_apps = Rc::new(RefCell::new(list_applications.clone()));

        search_entry.connect_changed({
            let vbox = vbox.clone();
            let applications = list_applications.clone();
            let win = window.clone();
            let selected_index = selected_index.clone();
            let filtered_apps = filtered_apps.clone();
            move |entry| {
                let search_text = entry.text().to_string();
                let filtered = filter_applications(&search_text, &vbox, &applications, &win);
                *filtered_apps.borrow_mut() = filtered.clone();
                *selected_index.borrow_mut() = 0;
            }
        });

        // Manejar teclas arriba/abajo
        search_entry.add_controller({
            let selected_index = selected_index.clone();
            let filtered_apps = filtered_apps.clone();
            let key_controller = gtk::EventControllerKey::new();
            let win = window.clone();
            key_controller.set_propagation_phase(gtk::PropagationPhase::Capture);
            key_controller.connect_key_pressed(move |_, keyval, _, _| {
                use gtk::gdk::Key;
                let mut idx: usize = *selected_index.borrow();
                let apps = filtered_apps.borrow();
                let len = apps.len();
                if len == 0 {
                    return false.into();
                }

                match keyval {
                    Key::Up => {
                        idx = idx.saturating_sub(1);
                        *selected_index.borrow_mut() = idx;

                        true.into()
                    }
                    Key::Down => {
                        if idx + 1 < len {
                            idx += 1;
                        }
                        *selected_index.borrow_mut() = idx;
                        true.into()
                    }
                    Key::Return => {
                        if let Some(app) = apps.get(idx) {
                            let exec_cmd = app.exec.clone();
                            let win = win.clone();

                            std::process::Command::new("sh")
                                .arg("-c")
                                .arg(&exec_cmd)
                                .current_dir(std::env::var("HOME").unwrap())
                                .spawn()
                                .ok();
                            win.close();
                        }
                        true.into()
                    }
                    Key::Escape => {
                        win.close();
                        true.into()
                    }

                    _ => false.into(),
                }
            });
            key_controller
        });

        main_box.append(&search_entry);
        let scrolled_window = gtk::ScrolledWindow::builder()
            .min_content_width(220)
            .min_content_height(600)
            .child(&vbox)
            .build();

        filter_applications("", &vbox, &list_applications, &window);
        main_box.append(&scrolled_window);
        window.set_child(Some(&main_box));
        window.show();
    });

    app.run();
}
