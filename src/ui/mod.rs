use yew::html::Scope;
use yew::prelude::*;

use crate::engine::engine::{EndOfGameView, Engine, InGameView, ViewModel};
use crate::engine::random::PseudoRandomGenerator;
use crate::log;

pub enum AppEvent {
    MakeChoice(usize),
    WaitOneCycle,
}

pub struct App {
    game: Engine<PseudoRandomGenerator>,
    view_model: ViewModel,
}

impl Component for App {
    type Message = AppEvent;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game: Engine::new(vec![], PseudoRandomGenerator {}),
            view_model: ViewModel::InGame {
                0: InGameView {
                    lines: vec!["Welcome to unnamed game".to_string()],
                    choices: vec![],
                }
            },
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
                        <p>{ "population: " } { self.game.get_state().population}</p>
                        <p>{ "ecology: " }{ self.game.get_state().ecology}</p>
                        <p>{ "money: " }{ self.game.get_state().money}</p>
                        <p>{ "turn " }{ self.game.get_state().turn_counter}</p>
                    </div>
                </div>
                    {
                        match &self.view_model{
                            ViewModel::InGame(in_game_view) => {
                                html!{
                                    <>
                                        <div class="game-board">
                                            <h2>{ "Entry logs" }</h2>
                                            { self.view_event(in_game_view) }
                                        </div>
                                        <div class="game-board">
                                            <h2>{ "Control panel" }</h2>
                                            {self.view_choices(ctx.link(), in_game_view)}
                                        </div>
                                    </>}
                            },
                            ViewModel::EndOfGame(end_of_game_view) => {
                                self.view_end_of_game(end_of_game_view)
                            },
                        }
                    }
            </div>
        };
    }
}

impl App {
    fn view_event(&self, in_game_view: &InGameView) -> Html {
        log!(format!("debug view_model : {:?}", in_game_view));

        return if !!!in_game_view.lines.is_empty() {
            html! {
                <div class="event">
                    <ul class="log-entries">
                        {for in_game_view.lines.clone().iter().map(|log_entry| self.view_one_log_entry(log_entry))}
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

    fn view_choices(&self, link: &Scope<Self>, in_game_view: &InGameView) -> Html {
        return if !!!in_game_view.choices.is_empty() {
            html! {
                <div class="choices">
                    <h3>{"What is your response?"}</h3>
                    <ul class="choices">
                        {for in_game_view.choices.clone().iter().enumerate().map(|(index, choice)| self.view_one_choice(choice, index, link))}
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
    fn view_end_of_game(&self, end_of_game_view: &EndOfGameView) -> Html {
        return html! {
             <div class="game-board">
                {
                    if end_of_game_view.is_victory {
                       {"You have won, good job !"}
                    } else {
                       {"You have lost !"}
                    }
                }
                <button id="toto" type="button" onclick={
                        move |_| web_sys::window()
                        .unwrap()
                        .location()
                        .reload()
                        .expect("Failed to reload")
                    }>{"Play again"}</button>
             </div>
        };
    }
}

pub fn run() {
    yew::Renderer::<App>::new().render();
}
