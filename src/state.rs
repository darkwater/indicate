use color::Color;
use error::Error;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str::{self, FromStr};

pub struct State {
    pub text:                String,
    pub font:                String,
    pub color:               Color,
    pub right_aligned:       bool,
    pub progress:            Progress,
    pub indeterminate_speed: u32,
    pub progress_current:    u64,
    pub progress_max:        u64,
}

#[derive(Debug)]
pub enum Progress {
    Indeterminate,
    Determinate,
    None,
}

impl FromStr for Progress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "indeterminate" => Ok(Progress::Indeterminate),
            "determinate"   => Ok(Progress::Determinate),
            "none"          => Ok(Progress::None),
            _               => Err(Error::from_string(format!("invalid progress type {:?}", s))),
        }
    }
}

#[derive(Debug)]
pub enum UpdateMsg {
    Text(String),
    Font(String),
    Color(Color),
    Progress(Progress),
    IndeterminateSpeed(u32),
    ProgressCurrent(u64),
    ProgressMax(u64),
}

impl FromStr for UpdateMsg {
    type Err = Error;

    /// Parse a string of format \key=value
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('\\') {
            let pair = s.split_at(1).1;
            let mut pair = pair.splitn(2, '=');
            let key = pair.next().unwrap().trim();
            let value = pair.next().map(str::trim).ok_or(Error::from_string(format!("no value given for {}", key)));

            use self::UpdateMsg::*;
            return match key {
                "font"                => Ok(Font(value?.into())),
                "color"               => Ok(Color(value?.parse()?)),
                "progress"            => Ok(Progress(value?.parse()?)),
                "indeterminate_speed" => Ok(IndeterminateSpeed(value?.parse()?)),
                "progress_current"    => Ok(ProgressCurrent(value?.parse()?)),
                "progress_max"        => Ok(ProgressMax(value?.parse()?)),
                _                     => Err(Error::from_string(format!("unknown attribute {:?}", key))),
            }
        }

        Ok(UpdateMsg::Text(s.to_string()))
    }
}

impl State {
    pub fn new() -> Result<Self, Error> {
        // default values
        let mut state = State {
            text:                format!(""),
            font:                format!("Sans 12"),
            color:               Color(1.0, 1.0, 1.0, 1.0),
            right_aligned:       true,
            progress:            Progress::Indeterminate,
            indeterminate_speed: 1,
            progress_current:    0,
            progress_max:        100,
        };

        if let Ok(file_contents) = read_config_file() {
            for line in file_contents.lines() {
                let update: UpdateMsg = line.parse()?;
                state.update(update);
            }
        }

        Ok(state)
    }

    pub fn update(&mut self, msg: UpdateMsg) {
        use self::UpdateMsg::*;
        match msg {
            Text(s)               => self.text = s,
            Font(s)               => self.font = s,
            Color(c)              => self.color = c,
            Progress(p)           => self.progress = p,
            IndeterminateSpeed(u) => self.indeterminate_speed = u,
            ProgressCurrent(u)    => self.progress_current = u,
            ProgressMax(u)        => self.progress_max = u,
        }
    }
}

fn read_config_file() -> Result<String, Error> {
    let config_path = env::var("INDICATE_CONFIG").or_else(|_|
                      env::var("HOME").map(|h| format!("{}/.config/indicate/config.rc", h)))?;

    let mut contents = String::new();
    let mut file = File::open(config_path)?;
    let _ = file.read_to_string(&mut contents)?;

    Ok(contents)
}
