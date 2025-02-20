pub mod types;
pub mod helpers;
pub mod house_layout;

type Result<T> = std::result::Result<T,Error>;
type Error = Box<dyn std::error::Error>;
