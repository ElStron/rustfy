use crate::AppInfo;
use std::fs;

use gtk::prelude::*;
use gtk::ApplicationWindow;
use gtk::{Box as GtkBox, Button, Entry, Image, Orientation};

use std::path::Path;
use std::process::Command;

pub fn list_applications() -> Vec<AppInfo> {
    let mut apps = vec![];
    let paths = [
        "/usr/share/applications".to_string(),
        format!(
            "{}/.local/share/applications",
            std::env::var("HOME").unwrap()
        ),
    ];

    for path in paths {
        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "desktop") {
                continue;
            }

            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            if let Some(app) = parse_desktop_file(&content) {
                apps.push(app);
            }
        }
    }

    apps
}

fn parse_desktop_file(content: &str) -> Option<AppInfo> {
    let mut name = None;
    let mut exec = None;
    let mut icon = None;

    for line in content.lines() {
        if name.is_none() {
            if let Some(stripped) = line.strip_prefix("Name=") {
                name = Some(stripped.to_string());
            }
        }
        if exec.is_none() {
            if let Some(stripped) = line.strip_prefix("Exec=") {
                let exec_clean = stripped
                    .split_whitespace()
                    .filter(|s| !s.starts_with('%'))
                    .collect::<Vec<_>>()
                    .join(" ");
                exec = Some(exec_clean);
            }
        }
        if icon.is_none() {
            if let Some(stripped) = line.strip_prefix("Icon=") {
                icon = Some(stripped.to_string());
            }
        }

        if name.is_some() && exec.is_some() && icon.is_some() {
            break;
        }
    }

    Some(AppInfo {
        name: name?,
        exec: exec?,
        icon,
    })
}

pub fn filter_applications(
    search_text: &str,
    vbox: &GtkBox,
    applications: &[AppInfo],
    window: &ApplicationWindow,
) -> Vec<AppInfo> {
    let mut children = vec![];
    let mut child = vbox.first_child();
    while let Some(widget) = child {
        child = widget.next_sibling();

        if !widget.is::<Entry>() {
            children.push(widget);
        }
    }
    for widget in children {
        vbox.remove(&widget);
    }

    let filtered: Vec<AppInfo> = applications
        .iter()
        .filter(|AppInfo { name, .. }| name.to_lowercase().contains(&search_text.to_lowercase()))
        .cloned()
        .collect();

    for AppInfo { name, icon, exec } in &filtered {
        let hbox = GtkBox::new(Orientation::Horizontal, 3);
        hbox.add_css_class("app_container");
        hbox.set_hexpand(true);

        if let Some(ref icon_name) = icon {
            let image = load_icon(icon_name, 16);
            hbox.append(&image);
        }

        let button = Button::with_label(name);
        let exec_cmd = exec.clone();

        let window_clone = window.clone();
        button.connect_clicked(move |_| {
            let _ = Command::new("sh")
                .arg("-c")
                .arg(&exec_cmd)
                .current_dir(std::env::var("HOME").unwrap())
                .spawn();
            window_clone.close();
        });

        hbox.append(&button);
        vbox.append(&hbox);
    }
    filtered
}

pub fn load_icon(icon_name: &str, size: i32) -> Image {
    let image = if icon_name.contains('/')
        || icon_name.ends_with(".png")
        || icon_name.ends_with(".svg")
        || icon_name.ends_with(".xpm")
    {
        if Path::new(icon_name).exists() {
            Image::from_file(icon_name)
        } else {
            Image::new()
        }
    } else {
        Image::from_icon_name(icon_name)
    };

    image.set_pixel_size(size);
    image
}
