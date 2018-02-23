#![feature(duration_extras)]
extern crate cairo;
extern crate gdk;
extern crate gtk;
extern crate pango;
extern crate pangocairo;

mod color;
mod config;

use color::Color;
use gdk::prelude::*;
use gtk::prelude::*;
use pango::LayoutExt;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

struct State {
    text: String,
    font: String,
    color: Color,
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
    let (width, height) = (280, 50);
    let (x, y) = (monitor.x + monitor.width - width - padding, monitor.y + monitor.height - height - padding);
    window.move_(x, y);
    window.resize(width, height);

    let mut input = String::with_capacity(64);
    io::stdin().read_line(&mut input).unwrap();

    let state_ = Arc::new(Mutex::new(State {
        text: input,
        font: "Droid Sans Mono 16".into(),
        color: Color(1.0, 1.0, 1.0, 1.0),
    }));

    let state = state_.clone();
    thread::spawn(move || {
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
    window.connect_draw(move |window, context| {
        let width  = window.get_allocated_width()  as f64;
        let height = window.get_allocated_height() as f64;
        let margin = 15.0;

        let mut position = 0.0;
        position += margin;

        // background
        let (r, g, b, a) = (0.10, 0.10, 0.10, 0.90);
        context.set_source_rgba(r, g, b, a);
        context.rectangle(0.0, 0.0, width, height);
        context.fill();

        let state = state.lock().unwrap();

        // text
        let layout = pangocairo::functions::create_layout(context).unwrap();
        let font = pango::FontDescription::from_string(&state.font);
        layout.set_text(&state.text);
        layout.set_font_description(Some(&font));

        let extents = layout.get_extents().1;

        let (x, y) = (position, 12.0);
        let text_width = extents.width as f64 / pango::SCALE as f64;

        let Color(r, g, b, a) = state.color;
        context.set_source_rgba(r, g, b, a);

        context.move_to(x, y);
        pangocairo::functions::show_layout(&context, &layout);

        position += text_width;
        position += margin;

        println!("{}", position);

        if position > width {
            let (x, y) = window.get_position();
            let new_width = position.ceil() as i32;
            window.set_size_request(new_width, height as i32);
            window.move_(x - (new_width - width as i32), y);
        }

        Inhibit(false)
    });

    let window_c = window.clone();
    gtk::timeout_add(200, move || {
        window_c.queue_draw();
        Continue(true)
    });

    window.show_all();
    window.set_keep_above(true);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
