pub mod cli;
pub mod core;
pub mod utils;

pub mod prelude {
    pub use crate::cli::commands::Commands;
    pub use crate::cli::Cli;
    pub use crate::utils::{default_spinner, Config, Progress, ProgressStyle};
}
