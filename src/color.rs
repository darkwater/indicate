use error::Error;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default)]
pub struct Color(pub f64, pub f64, pub f64, pub f64);

impl FromStr for Color {
    type Err = Error;

    /// Parse a string of format #rrggbb[aa]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_msg  = || Error::from_string(format!("invalid color {}", s));
        let mut iter = s.chars().into_iter().peekable();

        // Optional leading #
        if let Some(&'#') = iter.peek() {
            let _ = iter.next();
        }

        let red = iter.next().and_then(|d| d.to_digit(16)).ok_or_else(&err_msg)? * 16
                + iter.next().and_then(|d| d.to_digit(16)).ok_or_else(&err_msg)?;

        let green = iter.next().and_then(|d| d.to_digit(16)).ok_or_else(&err_msg)? * 16
                  + iter.next().and_then(|d| d.to_digit(16)).ok_or_else(&err_msg)?;

        let blue = iter.next().and_then(|d| d.to_digit(16)).ok_or_else(&err_msg)? * 16
                 + iter.next().and_then(|d| d.to_digit(16)).ok_or_else(&err_msg)?;

        let alpha = if iter.peek().is_some() {
            iter.next().and_then(|d| d.to_digit(16)).ok_or_else(&err_msg)? * 16
          + iter.next().and_then(|d| d.to_digit(16)).ok_or_else(&err_msg)?
        } else {
            255
        };

        if let Some(_) = iter.next() {
            return Err(Error::from_string("color is too long".to_string()));
        }

        let red   = red   as f64 / 255.0;
        let green = green as f64 / 255.0;
        let blue  = blue  as f64 / 255.0;
        let alpha = alpha as f64 / 255.0;

        Ok(Color(red, green, blue, alpha))
    }
}
