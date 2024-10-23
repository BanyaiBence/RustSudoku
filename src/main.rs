mod button;
mod helpers;
mod label;
mod table;
mod ui;
mod window;
use std::process::Command;
use ui::UI;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");
    let ui = UI::new();
    ui.show();

    gtk::main();
    let output = Command::new("echo")
        .arg("Hello")
        .output()
        .expect("Command failed");
    if !(output.status.success()) {
        println!("Sorry, cleanin up the terminal failed! :(")
    }
}
