mod api_doc;
pub mod dtos;
mod errors;
pub mod handlers;
pub mod routes;

pub use api_doc::ApiDoc;
pub use errors::app_err::AppErr;
pub use errors::AppResult;
