use crate::utils::auto_complete;

mod utils;

fn main() {
    color_eyre::install().ok();
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    auto_complete::initialize();
}
