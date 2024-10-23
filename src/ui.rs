use crate::button::Button;
use crate::label::Label;
use crate::table::Table;
use crate::window::Window;
use gtk::prelude::*;
use gtk::Grid;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub enum Cell {
    Label(Label),
    Button(Button),
}

pub struct UI {
    pub window: Window,
}

impl UI {
    pub fn new() -> UI {
        let grid = Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);
        let window = Window::new(grid.clone());

        let mut table = Table::new();
        table.solve();
        let sol_table = table;
        table.remove_cells(25);

        let table_ref = Rc::new(RefCell::new(table));
        let sol_table_ref = Rc::new(sol_table);

        for y in 0..9 {
            for x in 0..9 {
                let val = table.get(x, y);
                let text = if val == 0 { "" } else { &val.to_string() };
                let cell = if val == 0 {
                    let button = Button::new(x, y, table_ref.clone(), sol_table_ref.clone());
                    Cell::Button(button)
                } else {
                    let label = Label::new(text, x, y);
                    Cell::Label(label)
                };
                match cell {
                    Cell::Button(button) => {
                        grid.attach(&button.content, x as i32, y as i32, 1, 1);
                    }
                    Cell::Label(label) => {
                        grid.attach(&label.content, x as i32, y as i32, 1, 1);
                    }
                }
            }
        }
        UI { window: window }
    }

    pub fn show(&self) {
        self.window.content.show_all();
    }
}
