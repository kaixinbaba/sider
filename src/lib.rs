mod cli;
mod widget;
mod errors;
mod events;


pub type SDResult<T> = std::result::Result<T, errors::SDError>;
