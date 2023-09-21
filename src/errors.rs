pub mod app_err;
pub mod res_err;

pub type AppResult<T> = error_stack::Result<T, app_err::AppErr>;
