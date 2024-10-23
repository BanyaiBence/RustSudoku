#![allow(dead_code)]
use gtk::prelude::*;
#[derive(Debug, Clone)]
pub struct Label {
    pub content: gtk::Label,
    pub x: usize,
    pub y: usize,
}
impl Label {
    pub fn new(text: &str, x: usize, y: usize) -> Label {
        let content = gtk::Label::new(Some(text));
        content.set_hexpand(true);
        content.set_vexpand(true);
        let markup: String = format!("<span font_size=\"20000\">{}</span>", text);
        content.set_markup(&markup);
        Label {
            content: content.clone(),
            x,
            y,
        }
    }
    pub fn update(&self, val: u8) {
        let markup: String = format!("<span font_size=\"20000\">{}</span>", val.to_string());
        self.content.set_markup(&markup);
    }
}
