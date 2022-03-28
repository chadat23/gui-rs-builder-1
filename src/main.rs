use gui_rs::guiprocessing;
use gui_rs::guiproperties::guiposition::GUISize;
use gui_rs::guiproperties::guitraits::*;
use gui_rs::guiproperties::GUIColor;
use gui_rs::guiresources::GUIResources;
use gui_rs::guiwidgets::{GUIButton, GUIWindow};

fn main() {
    // let window = GUIWindow::new().set_width(400).set_height(600).set_title(String::from("Hello"));
    let mut window = GUIWindow::default();
    window.set_size(GUISize::from_pixels(800, 300));
    window.set_title("Hello");
    window.set_background_color(GUIColor::from_rgba_u8u8u8u8(50, 50, 50, 255));

    let button = GUIButton::default();
    window.add_area_child(Box::from(button));

    let resources = GUIResources::default();

    guiprocessing::run(window, resources);
}
