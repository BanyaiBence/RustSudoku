use gtk::glib::Propagation::Proceed;
use gtk::prelude::*;
use gtk::Grid;

#[derive(Debug, Clone)]
pub struct Window {
    pub content: gtk::Window,
}
impl Window {
    pub fn new(grid: Grid) -> Window {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_title("Sudoku");
        window.set_default_size(800, 800);

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Proceed
        });
        window.add(&grid);
        Window { content: window }
    }
}
