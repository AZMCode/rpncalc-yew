mod app;
mod utils;
mod calc_unit;

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    yew::start_app::<app::App>();
}