use gui_rs::guiprocessing;
use gui_rs::guiproperties::guiposition::{GUISize, GUIPosition, GUILength};
use gui_rs::guiproperties::guitraits::*;
use gui_rs::guiproperties::GUIColor;
use gui_rs::guiresources::GUIResources;
use gui_rs::guiwidgets::{GUIButton, GUIWindow};

fn main() {
    let mut window = GUIWindow::default();
    window.set_size(GUISize::from_pixels(800., 800.));
    window.set_title("Hello");
    window.set_background_color(GUIColor::from_rgba_u8u8u8u8(50, 50, 50, 255));

    let mut button = GUIButton::default();

    let mut child_button = GUIButton::default();
    child_button.set_size(GUISize::from_pixels(50., 25.));
    child_button.set_position_from_position(GUIPosition::from_pixels(10., 10.));
    child_button.set_radius_from_pixels(10.);
    child_button.set_background_color(GUIColor { r: 1., g: 0., b: 0., a: 1.});
    button.add_child(Box::from(child_button));
    window.add_child(Box::from(button));

    let button2 = GUIButton {
        text: "h",
        size: GUISize::from_pixels(200., 300.),
        position: GUIPosition::from_pixels(300., 255.),
        radius: GUILength::from_pixels(20.),
        background_color: GUIColor::from_rgba_u8u8u8u8(0, 185, 0, 255),
        children: Vec::new()
    };
    window.add_child(Box::from(button2));

    let resources = GUIResources::default();

    guiprocessing::run(window, resources);
}
