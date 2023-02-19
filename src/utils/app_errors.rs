use std::error::Error;

pub enum AppError {
    Error(Box<dyn Error>),
}
