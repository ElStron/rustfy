use gtk::prelude::WidgetExt;
use gtk::ApplicationWindow;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

pub fn layer_shell_configure(window: &ApplicationWindow) {
    LayerShell::init_layer_shell(window);
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::OnDemand);
    window.set_focusable(true);
    window.auto_exclusive_zone_enable();
    window.set_anchor(Edge::Right, false);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Bottom, false);
    window.set_visible(false);
}
