#![feature(box_syntax, duration_extras, specialization, termination_trait)]
extern crate cairo;
extern crate gdk;
extern crate gtk;
extern crate pango;
extern crate pangocairo;

mod color;
mod error;
mod state;

use color::Color;
use error::Error;
use gdk::prelude::*;
use gtk::prelude::*;
use pango::LayoutExt;
use state::{Progress, State};
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() -> Result<(), Error> {
    gtk::init()?;

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

    let mut state = State::new().unwrap();

    let mut input = String::with_capacity(64);
    while state.text == "" {
        io::stdin().read_line(&mut input).unwrap();
        state.update(input.parse()?);
        input.clear();
    }

    let state_ = Arc::new(Mutex::new(state));

    let state = state_.clone();
    thread::spawn(move || {
        let mut input = String::with_capacity(64);
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let mut state = state.lock().unwrap();
                    state.update(input.parse().unwrap());
                },
                Err(e) => panic!("{:?}", e),
            }
            input.clear();
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
        let layout   = pangocairo::functions::create_layout(context).unwrap();
        let mut font = pango::FontDescription::from_string(&state.font);
        let size     = font.get_size() as f64;
        font.set_size((size * resolution) as i32);
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
            Progress::Indeterminate => {
                let elapsed = start.elapsed() * state.indeterminate_speed;
                let t = elapsed.as_secs() as f64 + elapsed.subsec_millis() as f64 / 1000.0;
                let a = (t.sin() + 0.15 * (2.0*t).sin()) / 2.2 + 0.5;
                let Color(r, g, b, _) = state.color;
                context.set_source_rgba(r, g, b, a);

                context.rectangle(0.0, height - bar_height, width, bar_height);
                context.fill();
            },
            Progress::Determinate => {
                let bar_width = state.progress_current as f64 / state.progress_max as f64 * width;
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

    Ok(())
}
