use yew::prelude::*;
use yew::html::ChildrenRenderer;
use stylist::yew::Global;
use stylist::css;
use crate::utils::scope_channel::{Receiver,scope_channel};
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

mod calculator;
mod logging_tray;

pub enum AppMsg {
    ShowCalculator,
    ShowLoggingTray,
    LogMsg(String)
}

pub struct App {
    children: Html,
    calculator_recv: Receiver<calculator::Calculator>,
    logging_tray_recv: Receiver<logging_tray::LoggingTray>
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        use calculator::Calculator;
        use logging_tray::LoggingTray;
        let (calculator_snd, calculator_recv) = scope_channel();
        let (logging_tray_snd, logging_tray_recv) = scope_channel();
        let children = html!{
            < >
                <Global css={{ css!{
                    body, html {
                        width: 100%;
                        height: 100%;
                        overflow: hidden;
                    }
                    div, span, button, body, html {
                        font-size: 5vh;
                        border: 0;
                        padding: 0;
                        display: inline-block;
                    }
                    p {
                        padding: 0;
                        border: 0;
                    }
                }}}></Global>
                <Calculator visible=true scope_snd={ calculator_snd }></Calculator>
                <LoggingTray visible=false scope_snd={ logging_tray_snd }></LoggingTray>
            </>
        };
        App { children, calculator_recv, logging_tray_recv }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::ShowCalculator | AppMsg::ShowLoggingTray => {
                let mut calculator_msg = calculator::CalculatorMsg::Hide;
                let mut logging_tray_msg = logging_tray::LoggingTrayMsg::Hide;
                match msg {
                    AppMsg::ShowCalculator => calculator_msg = calculator::CalculatorMsg::Show,
                    AppMsg::ShowLoggingTray => logging_tray_msg = logging_tray::LoggingTrayMsg::Show,
                    _ => unreachable!()
                }
                self.calculator_recv.recv().send_message(calculator_msg);
                self.logging_tray_recv.recv().send_message(logging_tray_msg);    
            },
            AppMsg::LogMsg(s) => self.logging_tray_recv.recv().send_message(logging_tray::LoggingTrayMsg::LogMsg(s))
        }   
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let rendered_children = ChildrenRenderer::new(vec![self.children.clone()]);
        html!(
            < >
                {rendered_children}
            </>
        )
    }
}