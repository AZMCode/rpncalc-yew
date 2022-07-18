use yew::prelude::*;
use yew::html::Scope;
use stylist::css;
use crate::utils::scope_channel::Sender;
#[allow(unused_imports)]
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
    pub config: super::SharedConfigHandle,
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
            let parent: Scope<super::App> = ctx.link().get_parent().unwrap().clone().downcast();
            let switch_to = match ctx.props().config.get().color_theme {
                super::ColorTheme::Dark => super::ColorTheme::Light,
                super::ColorTheme::Light => super::ColorTheme::Dark
            };
            let msg = format!("Switch to {}",match switch_to {
                super::ColorTheme::Dark => "Dark Mode",
                super::ColorTheme::Light => "Light Mode"
            });
            html!{ 
                <div class={{ css!{
                    display: grid;
                    grid:   "a b b b b" 1fr
                            ". b b b b" 15fr;
                }}}>
                    <button class={{css!{grid-area: a;}}} onclick={ parent.callback(|_| super::AppMsg::ShowCalculator ) }> { "Return to Calculator" } </button> <br/>
                    <div class={{css!{grid-area: b;}}}>
                        <p> { "Logging Tray and Configuration" } </p>
                        <details>
                            <summary> { "Logging Tray" } </summary>
                            <pre> {&self.msgs} </pre>
                        </details>
                        <button onclick={ parent.callback(move |_| super::AppMsg::ChangeColorTheme(switch_to)) }> { msg } </button>
                    </div>
                </div>
            }
        } else { html!{} }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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