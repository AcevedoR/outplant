use yew::html::Scope;
use yew::prelude::*;

use crate::ui_orchestrator::{UIOrchestrator, ViewModel};

pub mod engine;
pub mod effect;
pub mod embed;
pub mod event;
pub mod event_chain;
pub mod event_store;
pub mod random;
pub mod state;
pub mod trigger;
pub mod ui_orchestrator;
mod macros;


pub enum AppEvent {
    MakeChoice(usize),
    WaitOneCycle,
}

pub struct App {
    game: UIOrchestrator,
    view_model: ViewModel,
}

impl Component for App {
    type Message = AppEvent;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game: UIOrchestrator::new(),
            view_model: ViewModel { lines: vec!["hello first turn".to_string()], choices: vec![] },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppEvent::MakeChoice(choice) => {
                self.view_model = self.game.make_choice(choice);
            }
            AppEvent::WaitOneCycle => {
                self.view_model = self.game.next_cycle();
            }
        }
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <div id="unknown-game">
                <div id="state-dashboard">
                    <h2>{ "Species dashboard" }</h2>
                    <div id="kpi">
                        <p>{"population: "} { self.game.get_state().population}</p>
                        <p>{"ecology: "}{ self.game.get_state().ecology}</p>
                        <p>{"money: "}{ self.game.get_state().money}</p>
                    </div>
                </div>
                <div class="game-board">
                    <h2>{ "Entry logs" }</h2>
                    {self.view_event()}
                </div>
                <div class="game-board">
                    <h2>{ "Control panel" }</h2>
                    {self.view_choices(ctx.link())}
                </div>
            </div>
        };
    }
}

impl App {
    fn view_event(&self) -> Html {
        log!(format!("debug view_model : {:?}", self.view_model));

        return if !!!self.view_model.lines.is_empty() {
            html! {
                <div class="event">
                    <ul class="log-entries">
                        {for self.view_model.lines.clone().iter().map(|log_entry| self.view_one_log_entry(log_entry))}
                    </ul>
                </div>
            }
        } else {
            html! {}
        };
    }

    fn view_one_log_entry(&self, log_entry: &String) -> Html {
        return html! {
             <li class="log-entry">
                { log_entry }
             </li>
        };
    }

    fn view_choices(&self, link: &Scope<Self>) -> Html {
        return
            if !!!self.view_model.choices.is_empty() {
                html! {
                <div class="choices">
                    <h3>{"What is your response ?"}</h3>
                    <ul class="choices">
                        {for self.view_model.choices.clone().iter().enumerate().map(|(index, choice)| self.view_one_choice(choice, index, link))}
                    </ul>
                </div>
            }
            } else {
                self.view_continue_next_cycle(link)
            };
    }

    fn view_one_choice(&self, choice: &String, index: usize, link: &Scope<Self>) -> Html {
        return html! {
             <li class="choice">
                <button
                    type="button"
                    id="wait-one-cycle"
                    onclick={link.callback(move |_| AppEvent::MakeChoice(index))}
                > { choice }</button>
             </li>
        };
    }

    fn view_continue_next_cycle(&self, link: &Scope<Self>) -> Html {
        return html! {
            <div>
                <h3>{"There is nothing you can do"}</h3>
                <button
                    type="button"
                    id="wait-one-cycle"
                    onclick={link.callback(|_| AppEvent::WaitOneCycle)}
                > {"Wait until next cycle"}</button>
            </div>
            };
    }
}

pub fn run() {
    yew::Renderer::<App>::new().render();
}
