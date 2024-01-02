mod dashboard;
mod password;
mod logout;
mod newsletter;

pub use dashboard::admin_dashboard;
pub use logout::log_out;
pub use password::*;
pub use newsletter::*;