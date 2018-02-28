use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;

#[derive(Debug)]
pub struct Error {
    message: String,
    cause: Option<Box<StdError>>,
}

impl Error {
    pub fn from_string(s: String) -> Self {
        Self {
            message: s,
            cause: None,
        }
    }
}

// impl std::Termination for Error {
//     fn report(self) -> i32 {
//         1
//     }
// }

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.message)
    }
}

// impl StdError for Error {
//     fn description(&self) -> &str {
//         &self.message
//     }

//     fn cause(&self) -> Option<&StdError> {
//         self.cause.as_ref().map(Box::as_ref)
//     }
// }

impl<E> From<E> for Error where E: StdError + 'static {
    fn from(e: E) -> Error {
        Error {
            message: e.description().into(),
            cause: Some(box e),
        }
    }
}

// impl From<String> for Error {
//     fn from(s: String) -> Error {
//         Error {
//             message: s,
//             cause: None,
//         }
//     }
// }
