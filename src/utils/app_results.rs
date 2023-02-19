use super::app_errors::AppError;

pub enum AppResult {
    Unit(Result<(), AppError>),
}
