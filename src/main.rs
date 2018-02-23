#![feature(duration_extras)]
extern crate cairo;
extern crate gdk;
extern crate gtk;

mod color;
mod config;

use gdk::prelude::*;
use gtk::prelude::*;
use std::thread;
use std::sync::{Arc, Mutex};

struct State {
    text: String,
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Popup);

    let screen = window.get_screen().unwrap();
    let monitor_id = screen.get_primary_monitor();
    let monitor = screen.get_monitor_geometry(monitor_id);

    let visual = screen.get_rgba_visual().unwrap();
    window.set_app_paintable(true);
    window.set_visual(Some(&visual));

    let padding = 40;
    let (width, height) = (280, 55);
    let (x, y) = (monitor.x + monitor.width - width - padding, monitor.y + monitor.height - height - padding);
    window.move_(x, y);
    window.resize(width, height);

    let state_ = Arc::new(Mutex::new(State {
        text: "Downloading".into(),
    }));

    let state = state_.clone();
    thread::spawn(move || {
        use std::io;
        let mut input = String::with_capacity(64);
        loop {
            input.clear();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let mut state = state.lock().unwrap();
                    state.text = input.trim_right().to_string();
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    });

    let state = state_.clone();
    window.connect_draw(move |widget, context| {
        let width  = widget.get_allocated_width()  as f64;
        let height = widget.get_allocated_height() as f64;

        let (r, g, b, a) = (0.10, 0.10, 0.10, 0.90);
        context.set_source_rgba(r, g, b, a);
        context.rectangle(0.0, 0.0, width, height);
        context.fill();

        let state = state.lock().unwrap();
        context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
        context.move_to(0.0, 40.0);
        context.show_text(&state.text);

        widget.queue_draw();

        Inhibit(false)
    });

    window.show_all();
    window.set_keep_above(true);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
