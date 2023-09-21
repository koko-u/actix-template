#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, derive_more::Display)]
#[display("Application Error")]
pub struct AppErr;

impl error_stack::Context for AppErr {}
