use yew::prelude::*;

mod calculator;
mod logging_tray;


pub struct App {
    calculator: calculator::Calculator,
    logging_tray: logging_tray::LoggingTray
}

impl Component for App {
    type 
}