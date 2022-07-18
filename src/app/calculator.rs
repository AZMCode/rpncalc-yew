use yew::prelude::*;
use stylist::css;
use yew::html::Scope;
use crate::utils::scope_channel::Sender;
use crate::utils::Ignore;
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};
use std::str::FromStr;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(CommOrOp,Op)]
trait ExtractCommand {
    fn command_or_op(self) -> rpncalc::CommandOrOp;
}

pub struct CommOrOpWrapper(CommOrOp);

impl ExtractCommand for CommOrOpWrapper {
    fn command_or_op(self) -> rpncalc::CommandOrOp {
        (self.0).command_or_op()
    }
}

#[enum_dispatch]
enum CommOrOp {
    Command(Command),
    Op(Op)
}

enum Command {
    Drop,
    Dup,
    Swap,
    Rev
}

impl ExtractCommand for Command {
    fn command_or_op(self) -> rpncalc::CommandOrOp {
        rpncalc::CommandOrOp::Command(match self {
            Command::Drop => rpncalc::CommandEnum::Drop(rpncalc::Drop::Some(1)),
            Command::Dup  => rpncalc::CommandEnum::Dup(rpncalc::Dup(1)),
            Command::Swap => rpncalc::CommandEnum::Swap(rpncalc::Swap::LastTwo),
            Command::Rev  => rpncalc::CommandEnum::Reverse(rpncalc::Reverse)
        })
    }
}

#[enum_dispatch]
enum Op {
    Arith(Arith),
    Constant(Constant),
    Exp2(Exp2),
    Exp(Exp),
    Trig(Trig)
}

enum Constant {
    Pi,
    E,
    Inf
}

impl ExtractCommand for Constant {
    fn command_or_op(self) -> rpncalc::CommandOrOp {
        match self {
            Constant::E   => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::Constants(rpncalc::ops::Constants::E  )),
            Constant::Pi  => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::Constants(rpncalc::ops::Constants::PI )),
            Constant::Inf => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::Constants(rpncalc::ops::Constants::Inf)),
        }
    }
}

enum Arith {
    Add,
    Sub,
    Mul,
    Div
}

impl ExtractCommand for Arith {
    fn command_or_op(self) -> rpncalc::CommandOrOp {
        rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::Arith(match self {
            Arith::Add => rpncalc::ops::Arith::Add,
            Arith::Sub => rpncalc::ops::Arith::Sub,
            Arith::Mul => rpncalc::ops::Arith::Mul,
            Arith::Div => rpncalc::ops::Arith::Div
        }))
    }
}

enum Exp2 {
    Pow,
    LogN,
    RootN
}

impl ExtractCommand for Exp2 {
    fn command_or_op(self) -> rpncalc::CommandOrOp {
        match self {
            Exp2::Pow => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::ExponentialsBinary(rpncalc::ops::ExponentialsBinary::Pow)),
            Exp2::LogN => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::ExponentialsBinary(rpncalc::ops::ExponentialsBinary::LogN)),
            Exp2::RootN => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::ExponentialsBinary(rpncalc::ops::ExponentialsBinary::RootN))
        }
    }
}

enum Exp {
    Log10,
    Log2,
    LogE,
    Root2
}

impl ExtractCommand for Exp {
    fn command_or_op(self) -> rpncalc::CommandOrOp {
        match self {
            Exp::Log10 => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::ExponentialsUnary(rpncalc::ops::ExponentialsUnary::Log10)),
            Exp::Log2  => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::ExponentialsUnary(rpncalc::ops::ExponentialsUnary::Log2 )),
            Exp::LogE  => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::ExponentialsUnary(rpncalc::ops::ExponentialsUnary::LogE )),
            Exp::Root2 => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::ExponentialsUnary(rpncalc::ops::ExponentialsUnary::Root2))
        }
    }
}

enum Trig {
    Sin,
    Cos,
    Tan,
    ASin,
    ACos,
    ATan
}

impl ExtractCommand for Trig {
    fn command_or_op(self) -> rpncalc::CommandOrOp {
        match self {
            Trig::Sin  => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::Trigonometric(rpncalc::ops::Trigonometric::Sin )),
            Trig::Cos  => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::Trigonometric(rpncalc::ops::Trigonometric::Cos )),
            Trig::Tan  => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::Trigonometric(rpncalc::ops::Trigonometric::Tan )),
            Trig::ASin => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::Trigonometric(rpncalc::ops::Trigonometric::ASin)),
            Trig::ACos => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::Trigonometric(rpncalc::ops::Trigonometric::ACos)),
            Trig::ATan => rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::Trigonometric(rpncalc::ops::Trigonometric::ATan))
        }
    }
}

pub enum CalculatorMsg {
    Show,
    Hide,
    DigitInput(u8),
    InsNum,
    Backspace,
    Dot,
    CommOrOp(CommOrOpWrapper)
}

#[derive(PartialEq,Properties)]
pub struct CalculatorProp {
    pub scope_snd: Sender<Calculator>,
    pub config: super::SharedConfigHandle,
    pub visible: bool
}

pub struct Calculator {
    visible: bool,
    display: String,
    stack_affected: bool,
    calc_unit: crate::calc_unit::CalcUnit
}

impl Component for Calculator {
    type Message = CalculatorMsg;
    type Properties = CalculatorProp;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.props().scope_snd.send(ctx.link().clone());
        Calculator {
            visible: ctx.props().visible,
            display: "0".to_string(),
            calc_unit: Default::default(),
            stack_affected: false
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.visible {
            let parent: Scope<super::App> = ctx.link().get_parent().unwrap().clone().downcast();
            let keypad = (0..=2u8)
                .map(|d| (d*3+1,d*3+2,d*3+3))
                .zip(vec![
                    ('a','b','c'),
                    ('d','e','f'),
                    ('g','h','i')
                ])
                .flat_map(|((a,b,c),(a_ar,b_ar,c_ar))|
                    vec![
                        html!{<button class={{css!{grid-area: ${a_ar};}}} onclick={ ctx.link().callback(move |_| CalculatorMsg::DigitInput(a)) }> { a } </button>},
                        html!{<button class={{css!{grid-area: ${b_ar};}}} onclick={ ctx.link().callback(move |_| CalculatorMsg::DigitInput(b)) }> { b } </button>},
                        html!{<button class={{css!{grid-area: ${c_ar};}}} onclick={ ctx.link().callback(move |_| CalculatorMsg::DigitInput(c)) }> { c } </button>}
                    ]
                ).chain(vec![
                        html!{<button class={{css!{grid-area: j;}}} onclick={ ctx.link().callback(move |_| CalculatorMsg::DigitInput(0)) }> { 0   } </button>},
                        html!{<button class={{css!{grid-area: k;}}} onclick={ ctx.link().callback(move |_| CalculatorMsg::Dot)           }> { "." } </button>},
                        html!{<button class={{css!{grid-area: l;}}} onclick={ ctx.link().callback(move |_| CalculatorMsg::Backspace)     }> {"<-" } </button>},
                        html!{<button class={{css!{grid-area: m;}}} onclick={ ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                                        CommOrOpWrapper(CommOrOp::Op(
                                            Op::Arith(
                                                Arith::Add
                                            )
                                        ))
                                    )
                                )}
                            > {"+"} </button>},
                        html!{<button class={{css!{grid-area: n;}}} onclick={ ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                                        CommOrOpWrapper(CommOrOp::Op(
                                            Op::Arith(
                                                Arith::Sub
                                            )
                                        ))
                                    )
                                )}> {"-"} </button>},
                        html!{<button class={{css!{grid-area: o;}}} onclick={ ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                                        CommOrOpWrapper(CommOrOp::Op(
                                            Op::Arith(
                                                Arith::Mul
                                            )
                                        ))
                                    )
                                )}> {"*"} </button>},
                        html!{<button class={{css!{grid-area: p;}}} onclick={ ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                                        CommOrOpWrapper(CommOrOp::Op(
                                            Op::Arith(
                                                Arith::Div
                                            )
                                        ))
                                    )
                                )}> {"/"} </button>},
                        html!{<button class={{css!{grid-area: q;}}}onclick={ ctx.link().callback(move |_| CalculatorMsg::InsNum ) }> {"Ins"} </button>},
                ]).collect::<Vec<_>>();
            let stack_btns = html!{
                < >
                    <button onclick={ ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Command(
                            Command::Drop
                        ))
                    )) }> { "Drop" } </button>
                    <button onclick={ ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Command(
                            Command::Dup
                        ))
                    )) }> { "Dup" } </button>
                    <button onclick={ ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Command(
                            Command::Swap
                        ))
                    )) }> { "Swap" } </button>
                    <button onclick={ ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Command(
                            Command::Rev
                        ))
                    )) }> { "Rev" } </button>
                </>
            };
            let const_btns = html!{
                < >
                    <button onclick={ ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Constant(
                                Constant::Pi
                            )
                        ))
                    )) }> {"Pi"} </button>
                    <button onclick={ ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Constant(
                                Constant::E
                            )
                        ))
                    )) }> {"E"} </button>
                    <button onclick={ ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Constant(
                                Constant::Inf
                            )
                        ))
                    )) }> {"Inf"} </button>
                    <div></div>
                </>
            };
            let pow2_btns = html!{
                < >
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Exp2(
                                Exp2::Pow
                            )
                        ))
                    ))}> { "Pow" } </button>
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Exp2(
                                Exp2::LogN
                            )
                        ))
                    ))}> { "LogN" } </button>
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Exp2(
                                Exp2::RootN
                            )
                        ))
                    ))}> { "RootN" } </button>
                    <div></div>
                </>
            };
            let pow_btns = html!{
                < >
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Exp(
                                Exp::Log10
                            )
                        ))
                    ))}> { "Log10" } </button>
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Exp(
                                Exp::Log2
                            )
                        ))
                    ))}> { "Log2" } </button>
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Exp(
                                Exp::LogE
                            )
                        ))
                    ))}> { "LogE" } </button>
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Exp(
                                Exp::Root2
                            )
                        ))
                    ))}> { "Root2" } </button>
                </>
            };
            let trig_btns = html!{
                < >
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Trig(
                                Trig::Sin
                            )
                        ))
                    ))}> { "Sin" } </button>
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Trig(
                                Trig::Cos
                            )
                        ))
                    ))}> { "Cos" } </button>
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Trig(
                                Trig::Tan
                            )
                        ))
                    ))}> { "Tan" } </button>
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Trig(
                                Trig::ASin
                            )
                        ))
                    ))}> { "ASin" } </button>
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Trig(
                                Trig::ACos
                            )
                        ))
                    ))}> { "ACos" } </button>
                    <button onclick={ctx.link().callback(move |_| CalculatorMsg::CommOrOp(
                        CommOrOpWrapper(CommOrOp::Op(
                            Op::Trig(
                                Trig::ATan
                            )
                        ))
                    ))}> { "ATan" } </button>
                </>
            };
            let stack_slice = self.calc_unit.get_stack();
            let stack = if stack_slice.len() == 0 {
                html!{
                    <p> { "<Empty Stack>" } </p>
                }
            } else {
                stack_slice.iter()
                    .rev()
                    .enumerate()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|(i,num)|
                        format!("{i:3}.- {}",rpncalc::format_num(*num))
                    ).map(|s|
                        html!{
                            <p class={{css!{text-indent: 1vh; margin: 0;}}}> { s } </p>
                        }
                    ).collect::<Html>()
            };
            let stack_align = if stack_slice.len() == 0 {
                "right"
            } else {
                "left"
            };

            html!{
                <div class={{css!{
                    width: 100%;
                    height: 100%;
                    display: grid;
                    grid-gap: 0;
                    grid:   "a b b b b" 1fr
                            ". b b b b" 2fr
                            "c c c c c" 1fr
                            "d d d d d" 2fr
                            "d d d d d" 2fr;
                }}}>
                    <button class={{css!{grid-area: a;}}} onclick={ move |_| parent.send_message(super::AppMsg::ShowLoggingTray)}> { "L" } </button>
                    <div id="stackscroller" class={{css!{grid-area: b; scroll-snap-type: y proximity;
                        overflow: hidden; overflow-y: scroll;}}}
                    >
                        <div class={{css!{
                            width: 100%;
                            vertical-align: text-top; text-wrap: nowrap; text-overflow: ellipsis;
                            text-align: ${stack_align}; scroll-snap-align: end; overflow-x: hidden;
                        }}}>
                            { stack }
                        </div>
                        <div class={{css!{
                            width: 100%;
                            height: 0;
                            scroll-snap-align: end;
                        }}}></div>
                    </div>
                    <div    class={{css!{grid-area: c; text-align: right; overflow-x: hidden; text-overflow: ellipsis;}}}> { self.display.as_str() } </div>
                    <div    class={{css!{display: grid; grid: ". . ." 1fr; scroll-snap-type: x mandatory; grid-area: d; overflow-x: scroll; overflow-y: hidden;}}}>
                        <div class={{css!{
                            scroll-snap-align: start;
                            display: grid;
                            width: 100vw;
                            height: 100%;
                            grid:   "a b c m" 1fr
                                    "d e f n" 1fr
                                    "g h i o" 1fr
                                    "k j l p" 1fr
                                    ". . . q" 1fr;
                        }}}>
                            { for keypad }
                        </div>
                        <div class={{css!{
                            scroll-snap-align: start;
                            display: grid;
                            width: 100vw;
                            height: 100%;
                            grid:   ". . . ." 1fr
                                    ". . . ." 1fr
                                    ". . . ." 1fr
                                    ". . . ." 1fr;
                        }}}>
                            { stack_btns }
                            { const_btns }
                            {  pow_btns  }
                            {  pow2_btns }
                        </div>
                        <div class={{css!{
                            scroll-snap-align: start;
                            display: grid;
                            width: 100vw;
                            height: 100%;
                            grid:   ". . ." 1fr
                                    ". . ." 1fr;
                        }}}>
                            { trig_btns }
                        </div>
                    </div>
                </div>
            }
        } else { debug!("Rendering Calculator as Invisible"); html!{} }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.stack_affected = false;
        match msg {
            CalculatorMsg::Show => self.visible = true,
            CalculatorMsg::Hide => self.visible = false,
            CalculatorMsg::DigitInput(d) => if &self.display == "0" {
                self.display = d.to_string();
            } else {
                self.display += &d.to_string();
            },
            CalculatorMsg::Backspace => { self.display.pop(); if self.display.len() == 0 { self.display.push('0'); } },
            CalculatorMsg::Dot => if !self.display.contains('.') { self.display.push('.') },
            CalculatorMsg::InsNum => {
                self.stack_affected = true;
                let ins_num = rpncalc::CommandOrOp::Op(rpncalc::ops::OpEnum::InsNum(rpncalc::ops::InsNum::from_str(&self.display).unwrap()));
                self.display = "0".to_string();
                self.calc_unit.run_command(ins_num).ignore();
            }
            CalculatorMsg::CommOrOp(c) => {
                self.stack_affected = true;
                let (stdout,res) = self.calc_unit.run_command(c.command_or_op());
                let mut out = String::new();
                out += &std::string::String::from_utf8_lossy(&stdout);
                out.push('\n');
                match res {
                    Ok(Some(s)) => out += &s,
                    Err(e) => out += &format!("{e:?}"),
                    _ => ()
                }
                out.push('\n');
                let parent_scope: Scope<super::App> = ctx.link().get_parent().unwrap().clone().downcast();
                parent_scope.send_message(super::AppMsg::LogMsg(out));
            }
        }
        true
    }
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if self.stack_affected {
            let elm = web_sys::window().unwrap().document().unwrap().get_element_by_id("stackscroller").unwrap();
            elm.scroll_to_with_scroll_to_options(&{
                let mut opt = web_sys::ScrollToOptions::new();
                opt.left(0.0);
                opt.top(elm.scroll_height() as f64);
                opt.behavior(web_sys::ScrollBehavior::Smooth);
                opt
            });
        }
    }
}