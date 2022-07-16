use yew::prelude::*;
use yew::html::Scope;
use crate::utils::scope_channel::Sender;
use std::rc::Rc;
use log::{trace, debug, info, warn, error};
pub enum LoggingTrayMsg {
    Show,
    Hide,
    LogMsg(String)
}

pub struct LoggingTray {
    visible: bool,
    msgs: String
}

#[derive(PartialEq,Properties)]
pub struct LoggingTrayProps {
    pub scope_snd: Sender<LoggingTray>,
    pub visible: bool
}

impl Component for LoggingTray {
    type Message = LoggingTrayMsg;
    type Properties = LoggingTrayProps;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.props().scope_snd.send(ctx.link().clone());
        LoggingTray { visible: ctx.props().visible, msgs: String::new() }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.visible {
            debug!("Rendering Logging Tray as Visible");
            let parent: Scope<super::App> = ctx.link().get_parent().unwrap().clone().downcast();
            html!{ < > { "LoggingTray" } <br/> <button onclick={ parent.callback(|_| super::AppMsg::ShowCalculator ) }> { "Switch to Calculator" } </button> <br/> <pre> {&self.msgs} </pre> </> }
        } else { html!{} }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoggingTrayMsg::Show => self.visible = true,
            LoggingTrayMsg::Hide => self.visible = false,
            LoggingTrayMsg::LogMsg(s) => {
                self.msgs += &s;
                self.msgs = self.msgs.trim_end_matches('\n').to_string();
            }
        }
        true
    }
}