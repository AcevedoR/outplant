pub mod engine;
pub mod dtos;
pub mod effect;
pub mod embed;
pub mod event;
pub mod event_chain;
pub mod event_store;
pub mod state;
pub mod trigger;
pub mod ui_orchestrator;
mod macros;


use yew::html::Scope;
use yew::prelude::*;

use crate::dtos::ChoiceWrapper;
use crate::dtos::OngoingEventChain;
use crate::ui_orchestrator::UIOrchestrator;



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
    current_events: Vec<OngoingEventChain>,
}

impl Component for App {
    type Message = AppEvent;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game: UIOrchestrator::new(),
            game_state: GameState::Pause,
            current_events: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppEvent::MakeChoice(choice) => {
                self.game_state = GameState::Pause;
                self.game.make_a_choice(&choice);
                self.current_events.clear();
            }
            AppEvent::WaitOneCycle => {
                self.current_events.clear();
                let events = self.game.play_next_cycle();
                if !!!events.is_empty() {
                    self.game_state = GameState::Waiting;
                    self.game_state = GameState::ResolvingEvent;
                    self.current_events = events;
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
                    <p>{"natural_balance: "}{ self.game.get_state().ecology}</p>
                    <p>{"ext reserve: "}{ self.game.get_state().money}</p>
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
        return if !!!self.current_events.is_empty() {
            let event = self.current_events.first().unwrap();
            html! {
                <div class="event">
                    <h3>{"Something happened: "}{&event.event.text}</h3>
                    <ul class="todo-list">
                        {for event.event.choices.clone().unwrap().iter().map(|choice| self.view_one_choice(
                            ChoiceWrapper{choice: choice.clone(), event_chain_id: event.event_chain_id.clone()},
                            link
                        ))}
                    </ul>
                </div>
            }
        } else {
            self.view_continue_button(link)
        };
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
    fn view_continue_button(&self, link: &Scope<Self>) -> Html {
        return if self.game_state == GameState::Pause {
            html! {
                <button
                    type="button"
                    id="wait-one-cycle"
                    onclick={link.callback(|_| AppEvent::WaitOneCycle)}
                > {"wait one cycle"}</button>
            }
        } else {
            html! {}
        };
    }
}
pub fn run() {
    yew::Renderer::<App>::new().render();
}