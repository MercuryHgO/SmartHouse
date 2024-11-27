pub mod types;
pub mod helpers;

type Result<T> = std::result::Result<T,Error>;
type Error = Box<dyn std::error::Error>;
