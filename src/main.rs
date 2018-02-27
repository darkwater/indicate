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
use std::time::Instant;

struct State {
    text:     String,
    font:     String,
    color:    Color,
    progress: Progress,
}

enum Progress {
    Indeterminate {
        speed: u32,
    },
    Determinate {
        progress: u64,
        max:      u64,
    },
    None,
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Popup);

    let screen     = window.get_screen().unwrap();
    let monitor_id = screen.get_primary_monitor();
    let monitor    = screen.get_monitor_geometry(monitor_id);
    let resolution = screen.get_property_resolution() / 96.0;
    let res_scale  = |i: i32| ((i as f64) * resolution) as i32;

    let visual = screen.get_rgba_visual().unwrap();
    window.set_app_paintable(true);
    window.set_visual(Some(&visual));

    let margin = res_scale(25);
    let width  = res_scale(180);
    let height = res_scale(35);
    let (x, y) = (monitor.x + monitor.width - width - margin, monitor.y + monitor.height - height - margin);
    window.move_(x, y);
    window.resize(width, height);

    let mut input = String::with_capacity(64);
    io::stdin().read_line(&mut input).unwrap();

    let state_ = Arc::new(Mutex::new(State {
        text:    input,
        font:    format!("Droid Sans Mono {}", res_scale(11)),
        color:   Color(1.0, 1.0, 1.0, 1.0),
        progress: Progress::Determinate { progress: 10, max: 100 },
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

    let start = Instant::now();

    let state = state_.clone();
    window.connect_draw(move |window, context| {
        let width  = window.get_allocated_width()  as f64;
        let height = window.get_allocated_height() as f64;
        let margin = 13.0 * resolution;

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
        let font   = pango::FontDescription::from_string(&state.font);
        layout.set_text(&state.text);
        layout.set_font_description(Some(&font));

        let extents = layout.get_extents().1;

        let (x, y)     = (position, 7.0 * resolution);
        let text_width = extents.width as f64 / pango::SCALE as f64;

        let Color(r, g, b, a) = state.color;
        context.set_source_rgba(r, g, b, a);

        context.move_to(x, y);
        pangocairo::functions::show_layout(&context, &layout);

        position += text_width;

        // progress bar
        let bar_height = 2.0 * resolution;
        match state.progress {
            Progress::Indeterminate { speed } => {
                let elapsed = start.elapsed() * speed;
                let t = elapsed.as_secs() as f64 + elapsed.subsec_millis() as f64 / 1000.0;
                let a = (t.sin() + 0.15 * (2.0*t).sin()) / 2.2 + 0.5;
                let Color(r, g, b, _) = state.color;
                context.set_source_rgba(r, g, b, a);

                context.rectangle(0.0, height - bar_height, width, bar_height);
                context.fill();
            },
            Progress::Determinate { progress, max } => {
                let bar_width = progress as f64 / max as f64 * width;
                context.rectangle(0.0, height - bar_height, bar_width, bar_height);
                context.fill();
            },
            Progress::None => (),
        }

        position += margin;
        if position > width {
            let (x, y)    = window.get_position();
            let new_width = position.ceil() as i32;
            window.set_size_request(new_width, height as i32);
            window.move_(x - (new_width - width as i32), y);
        }

        Inhibit(false)
    });

    let window_c = window.clone();
    gtk::timeout_add(20, move || {
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
