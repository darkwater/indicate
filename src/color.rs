use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default)]
pub struct Color(pub f64, pub f64, pub f64, pub f64);

impl FromStr for Color {
    type Err = &'static str;

    /// Parse a string of format #rrggbb
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_msg = "invalid color";
        let mut iter = s.chars().into_iter().peekable();

        // Optional leading #
        if let Some(&'#') = iter.peek() {
            let _ = iter.next();
        }

        let red = iter.next().and_then(|d| d.to_digit(16)).ok_or(err_msg)? * 16
                + iter.next().and_then(|d| d.to_digit(16)).ok_or(err_msg)?;

        let green = iter.next().and_then(|d| d.to_digit(16)).ok_or(err_msg)? * 16
                  + iter.next().and_then(|d| d.to_digit(16)).ok_or(err_msg)?;

        let blue = iter.next().and_then(|d| d.to_digit(16)).ok_or(err_msg)? * 16
                 + iter.next().and_then(|d| d.to_digit(16)).ok_or(err_msg)?;

        if let Some(_) = iter.next() {
            return Err(err_msg);
        }

        let red   = red   as f64 / 255.0;
        let green = green as f64 / 255.0;
        let blue  = blue  as f64 / 255.0;

        Ok(Color(red, green, blue, 1.0))
    }
}
