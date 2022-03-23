use gui_rs::guiresources::GUIResources;
use gui_rs::guiposition::guilengths::{GUILength, SetLength};
use gui_rs::guiprocessing::gui_processing;
use gui_rs::guiwindow::GUIWindow;

fn main() {
    // let window = GUIWindow::new().set_width(400).set_height(600).set_title(String::from("Hello"));
    let mut window = GUIWindow::default();
    window
        .set_width(GUILength::from_pixels(850.))
        .set_height(GUILength::from_pixels(250.))
        .set_title("Hello");

    let resources = GUIResources::default();
    
    gui_processing(window, resources);
}