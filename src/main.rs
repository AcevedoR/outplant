extern crate core;

use yew::html::Scope;
use yew::prelude::*;
use crate::dtos::ChoiceWrapper;
use crate::dtos::EventWrapper;

use crate::event::Choice;
use crate::ui_orchestrator::UIOrchestrator;

mod state;
mod engine;
mod event;
mod ui_orchestrator;
mod event_store;
mod event_chain;
mod effect;
mod dtos;

// see https://github.com/yewstack/yew/blob/yew-v0.20.0/examples/todomvc/src/main.rs

#[derive(PartialEq)]
pub enum GameState {
    Pause,
    Waiting,
    ResolvingEvent,
}

pub enum AppEvent {
    MakeChoice(ChoiceWrapper),
    WaitOneCycle,
}

pub struct App {
    game: UIOrchestrator,
    game_state: GameState,
    current_event: Option<EventWrapper>,
}

impl Component for App {
    type Message = AppEvent;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game: UIOrchestrator::new(),
            game_state: GameState::Pause,
            current_event: None::<EventWrapper>,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppEvent::MakeChoice(choice) => {
                self.game_state = GameState::Pause;
                self.game.make_a_choice(&choice);
                self.current_event = None;
            }
            AppEvent::WaitOneCycle => {
                self.current_event = None;
                let opt_event = self.game.play_next_cycle();
                if let Some(event) = opt_event {
                    self.game_state = GameState::Waiting;
                    self.game_state = GameState::ResolvingEvent;
                    self.current_event = Some(event);
                }
            }
        }
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <div id="unknown-game">
                <div id="state-dashboard">
                    <h2>{ "Species dashboard:" }</h2>
                    <p>{"population: "} { self.game.get_state().population}</p>
                    <p>{"natural_balance: "}{ self.game.get_state().natural_balance}</p>
                    <p>{"ext reserve: "}{ self.game.get_state().external_intervention_reserve}</p>
                </div>

                <div id="game-board">
                    <h2>{ "Species interface:" }</h2>
                    {self.view_event(ctx.link())}
                </div>
            </div>
        };
    }
}

impl App {
    fn view_event(&self, link: &Scope<Self>) -> Html {
        if let Some(event) = self.current_event.clone() {
            return html! {
                <div class="event">
                    <h3>{"Something happened: "}{event.event.text}</h3>
                    <ul class="todo-list">
                        {for event.event.choices.unwrap().iter().map(|choice| self.view_one_choice(
                            ChoiceWrapper{choice: choice.clone(), event_chain_id: event.event_chain_id.clone()},
                            link
                        ))}
                    </ul>
                </div>
            };
        } else {
            return self.view_continue_button(link);
        }
    }
    fn view_one_choice(&self, choice: ChoiceWrapper, link: &Scope<Self>) -> Html {
        let choice2 = choice.clone();
        return html! {
             <li class="choice">
                <button class="choice" onclick={link.callback(move |_| AppEvent::MakeChoice(choice.clone()))}>
                    { choice2.choice.text }
                </button>
             </li>
        };
    }
    fn view_continue_button(&self, link: &Scope<Self>) -> Html{
        if self.game_state == GameState::Pause {
           return html! {
            <button
                type="button"
                id="wait-one-cycle"
                onclick={link.callback(|_| AppEvent::WaitOneCycle)}
            > {"wait one cycle"}</button>
        }
        } else{
            return html!{}
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    println!();
}