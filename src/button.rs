use crate::table::Table;
use gtk::prelude::{ButtonExt, WidgetExt};
use std::cell::RefCell;
use std::rc::Rc;

type TableRef = Rc<RefCell<Table>>;

#[derive(Clone)]
pub struct Button {
    pub content: gtk::Button,
}
impl Button {
    pub fn new(x: usize, y: usize, table_ref: TableRef, sol_table_ref: Rc<Table>) -> Button {
        let content = gtk::Button::new();
        content.set_hexpand(true);
        content.set_vexpand(true);

        let content_clone = content.clone();

        content.connect_clicked(move |_| {
            let mut table = table_ref.borrow_mut();
            let value = sol_table_ref.get(x, y);
            table.set(x, y, value);
            content_clone.set_label(&value.to_string());
            content_clone.set_sensitive(false);
        });
        Button {
            content: content.clone(),
        }
    }
}
