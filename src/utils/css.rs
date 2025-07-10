use gtk::gdk::Display;
use gtk::CssProvider;

pub fn load_css() {
    let css = include_str!("../ui/style.css");

    let provider = CssProvider::new();
    provider.load_from_data(css);

    if let Some(display) = Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );
    }
}
