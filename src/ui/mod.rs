use std::collections::HashMap;
use stylist::css;
use stylist::yew::Global;

use yew::html::Scope;
use yew::prelude::*;

use crate::engine::engine::{EndOfGameView, Engine, InGameView, ViewModel};
use crate::engine::random::PseudoRandomGenerator;
use crate::log;

mod theme;

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
                    events_by_chain: HashMap::from(
                        [(
                            "Introduction".to_string(),
                            vec!["Welcome. You are a scientist on orbit around an uninhabited planet, and your job is to implement a new species. The end goal is to study how this species adapts to its environment, and as well find new evolutionary traits that your company could patent and sell!".to_string(),
                                    "You have just implemented the first subjects that you previously created in your lab. You are eager to watch them grow, and, hopefully, survive and adapt!".to_string(),
                                    "You are not sure to what extent it is wise that you physically intervene with them, so, for now, you try to let them as much possible on their own.".to_string()]
                        )]
                    ),
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

    #[allow(non_upper_case_globals)]
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {<>
            <Global css={
                let selected_theme = theme::Theme::AtrociousPurpleTheme;

                css!(
                r#"
                    body {
                        --background-color: #141414;
                        --main-color: ${main_color};
                        --main-color-light: ${main_color_light};
                        --main-color-dark: ${main_color_dark};
                    }
                "#,
                main_color = selected_theme.value().main_color,
                main_color_light = selected_theme.value().main_color_light,
                main_color_dark = selected_theme.value().main_color_dark,
            )} />
            <header classes="magnificent-blue-theme">
                <ul class="variable-dashboard">
                    <li class="variable-dashboard__item">{ "Pop: " }{ self.game.get_state().population() }</li>
                    <li class="variable-dashboard__item">{ "Eco: " }{ self.game.get_state().ecology() }</li>
                    <li class="variable-dashboard__item">{ "€€€: " }{ self.game.get_state().money() }</li>
                    <li class="variable-dashboard__item">{
                        match self.game.get_state().turn_counter() {
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
                                    { self.choice_displayer(ctx.link(), in_game_view, self.game.get_state().turn_counter() == &0) }
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
            <audio controls=true autoplay=false loop=true>
                <source src="https://soundimage.org/wp-content/uploads/2014/02/Blazing-Stars.mp3" alt="https://soundimage.org/sci-fi/"/>
            </audio>
        </>}
    }
}

impl App {
    fn log_displayer(&self, in_game_view: &InGameView) -> Html {
        log!(format!("debug view_model : {:?}", in_game_view));

        if !!!in_game_view.events_by_chain.is_empty() {
            html! {
                <section>
                    <ul>
                        {for in_game_view.events_by_chain.clone().iter().map(|events_for_chain| self.log_displayer_entry_group(events_for_chain)) }
                    </ul>
                </section>
            }
        } else {
            html! {}
        }
    }

    fn log_displayer_entry_group(&self, events_for_chain: (&String, &Vec<String>)) -> Html {
        html! {
            <li class="log-displayer__entry-group">
                <ul>
                    {for events_for_chain.1.iter().map(|event| self.log_displayer_entry(event))}
                </ul>
            </li>
        }
    }

    fn log_displayer_entry(&self, log_entry: &String) -> Html {
        html! {
            <li class="log-displayer__entry">
                { log_entry }
            </li>
        }
    }

    fn choice_displayer(
        &self,
        link: &Scope<Self>,
        in_game_view: &InGameView,
        game_start: bool,
    ) -> Html {
        html!(
            <section class="choice-displayer">{
                if !!!in_game_view.choices.is_empty() {
                    html!(<>
                        <p class="choice-displayer__cta">{ "What's your response?" }</p>
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
        html! {
             <li>
                <button
                    type="button"
                    class="choice-displayer__button"
                    onclick={link.callback(move |_| AppEvent::MakeChoice(index))}
                > { choice } </button>
             </li>
        }
    }

    fn choice_displayer_next_cycle(&self, link: &Scope<Self>) -> Html {
        html! {
        <>
            <p class="choice-displayer__cta">{"There is nothing to do for you right now."}</p>
            <button
                type="button"
                class="choice-displayer__button"
                onclick={link.callback(|_| AppEvent::WaitOneCycle)}
            > {"Wait until next cycle"}</button>
        </>
        }
    }

    fn end_of_game(&self, end_of_game_view: &EndOfGameView) -> Html {
        html! {
             <section class="end-of-game-section">
                <p class="choice-displayer__cta">{
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
        }
    }
}

pub fn run() {
    yew::Renderer::<App>::new().render();
}
