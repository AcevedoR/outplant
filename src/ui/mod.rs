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
                    lines: vec!["Welcome. You are a scientist on orbit around an uninhabited planet, and your job is to implement a new species. The end goal is to study how this species adapts to its environment, and as well as finding new evolutionary traits that your company could patent and sell!".to_string(),
                                "You have just implemented the first subjects that you previously created in your lab. You are eager to watch them grow, and, hopefully, survive and adapt!".to_string(),
                                "You are not sure to what extent it is wise that you physically intervene with them, so, for now, you try to let them as much possible on their own.".to_string()],
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
        html! {<>
            <header>
                <ul class="variable-dashboard">
                    <li class="variable-dashboard__item">{ "Pop: " }{ self.game.get_state().population}</li>
                    <li class="variable-dashboard__item">{ "Eco: " }{ self.game.get_state().ecology}</li>
                    <li class="variable-dashboard__item">{ "€€€: " }{ self.game.get_state().money}</li>
                    <li class="variable-dashboard__item">{
                        match self.game.get_state().turn_counter {
                            0 => "New game".to_owned(),
                            x => "Turn ".to_owned() + &x.to_string(),
                        }
                    }</li>
                </ul>
                </header>
                {
                    match &self.view_model{
                        ViewModel::InGame(in_game_view) => {
                            html!{
                                <>
                                    { self.log_displayer(in_game_view) }
                                    { self.choice_displayer(ctx.link(), in_game_view, self.game.get_state().turn_counter == 0) }
                                </>}
                        },
                        ViewModel::EndOfGame(end_of_game_view) => {
                            self.end_of_game(end_of_game_view)
                        },
                    }
                }
                <div class="background">
                    <div class="background__stars"/>
                    <div class="background__stars"/>
                    <div class="background__stars"/>
                    <div class="background__stars"/>
                    <div class="background__stars"/>
                </div>

        </>}
    }
}

impl App {
    fn log_displayer(&self, in_game_view: &InGameView) -> Html {
        log!(format!("debug view_model : {:?}", in_game_view));


        return if !!!in_game_view.lines.is_empty() {
            html! {
                <section class="log-displayer">
                    <ul class="log-displayer__entries">
                        {for in_game_view.lines.clone().iter().map(|log_entry| self.log_displayer_entry(log_entry))}
                    </ul>
                </section>
            }
        } else {
            html! {}
        };
    }

    fn log_displayer_entry(&self, log_entry: &String) -> Html {
        html! {
             <li class="log-displayer__entry">
                { log_entry }
             </li>
        }
    }

    fn choice_displayer(&self, link: &Scope<Self>, in_game_view: &InGameView, game_start: bool) -> Html {
        html!(
            <section class="choice-displayer">{
                if !!!in_game_view.choices.is_empty() {
                    html!(<>
                        <h3 class="choice-displayer__cta">{ "What's your response?" }</h3>
                        <ul class="choice-displayer__entries">
                            {for in_game_view.choices.clone().iter().enumerate().map(|(index, choice)| self.choice_displayer_entry(choice, index, link))}
                        </ul>
                    </>)
                } else if game_start {
                    html!(
                        <button
                            type="button"
                            class="choice-displayer__button"
                            onclick={link.callback(|_| AppEvent::WaitOneCycle)}
                        > {"Let's begin!"}</button>
                    )
                } else {
                    self.choice_displayer_next_cycle(link)
                }
            }</section>
        )
    }

    fn choice_displayer_entry(&self, choice: &String, index: usize, link: &Scope<Self>) -> Html {
        return html! {
             <li>
                <button
                    type="button"
                    class="choice-displayer__button"
                    onclick={link.callback(move |_| AppEvent::MakeChoice(index))}
                > { choice } </button>
             </li>
        };
    }

    fn choice_displayer_next_cycle(&self, link: &Scope<Self>) -> Html {
        return html! {
        <>
            <h3>{"There is nothing to do for you right now."}</h3>
            <button
                type="button"
                class="choice-displayer__button"
                onclick={link.callback(|_| AppEvent::WaitOneCycle)}
            > {"Wait until next cycle"}</button>
        </>
        };
    }

    fn end_of_game(&self, end_of_game_view: &EndOfGameView) -> Html {
        return html! {
             <section class="end-of-game-section">
                <p>{
                    if end_of_game_view.is_victory {
                       {"You have won, good job!"}
                    } else {
                       {"You have lost!"}
                    }
                }</p>
                <button type="button" class="end-of-game-section__play-again" onclick={
                        move |_| web_sys::window()
                        .unwrap()
                        .location()
                        .reload()
                        .expect("Failed to reload")
                    }>{"Play again"}</button>
             </section>
        };
    }
}

pub fn run() {
    yew::Renderer::<App>::new().render();
}
