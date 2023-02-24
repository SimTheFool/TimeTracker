use super::app_errors::AppError;

pub type AppResult<T> = Result<T, AppError>;
