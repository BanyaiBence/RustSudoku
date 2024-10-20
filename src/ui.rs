#![allow(dead_code)]
use gtk::glib::Propagation::Proceed;
use gtk::prelude::*;
use gtk::{Button, Grid, Label, Window, WindowType};

pub enum Cell {
    Label(Label),
    Button(Button),
}

pub struct UI {
    window: Window,
    grid: Grid,
}

impl UI {
    pub fn new() -> UI {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Sudoku");
        window.set_default_size(800, 800);

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Proceed
        });

        let grid = Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);
        window.add(&grid);

        UI { window, grid }
    }

    pub fn add_cell(&mut self, cell: &Cell, x: i32, y: i32) {
        match cell {
            Cell::Button(button) => {
                button.set_hexpand(true);
                button.set_vexpand(true);
                self.grid.attach(button, x, y, 1, 1);
            }
            Cell::Label(label) => {
                label.set_hexpand(true);
                label.set_vexpand(true);
                self.grid.attach(label, x, y, 1, 1);
            }
        }
    }

    pub fn show(&self) {
        self.window.show_all();
    }
}
