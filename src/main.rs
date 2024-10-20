mod helpers;
mod table;
mod ui;
use gtk::prelude::*;
use gtk::{Button, Label};
use table::Table;
use ui::Cell;
use ui::UI;

fn init() -> UI {
    let mut t = Table::new();
    t.solve();
    t.remove_cells(25);

    let mut ui = UI::new();

    for y in 0..9 {
        for x in 0..9 {
            let val = t.get(x, y);
            let text = if val == 0 { "" } else { &val.to_string() };
            let elem = if val == 0 {
                let button = Button::new();
                button.set_label(text);
                Cell::Button(button)
            } else {
                let label = Label::new(Some(text));
                let markup: String = format!("<span font_size=\"20000\">{}</span>", text);
                label.set_markup(&markup);
                Cell::Label(label)
            };

            ui.add_cell(&elem, x as i32, y as i32);
        }
    }
    ui
}

fn main() {
    gtk::init().expect("Failed to initialize GTK.");
    let ui = init();
    ui.show();
    gtk::main();
}
