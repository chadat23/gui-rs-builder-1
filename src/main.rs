use gui_rs::guiposition::guisizes::{GUISize, SetSize};
use gui_rs::guiwindow::GUIWindow;

fn main() {
    // let window = GUIWindow::new().set_width(400).set_height(600).set_title(String::from("Hello"));
    let mut window = GUIWindow::default();
    window
        .set_width(GUISize::from_pixels(650.))
        .set_height(GUISize::from_pixels(550.))
        .set_title(String::from("Hello"));
    window.start();
}
