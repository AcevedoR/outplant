#[macro_use]
extern crate assert_matches;

use unnamed_game::engine::engine::{Engine, ViewModel};
use unnamed_game::engine::language::Language;
use unnamed_game::engine::random::PseudoRandomGenerator;

#[test]
fn population_should_grow_if_no_event() {
    let mut engine = Engine::new(
        vec!["./tests/chains/never_triggers.json".to_string()],
        None,
        PseudoRandomGenerator {},
    );

    for _i in 0..6 {
        engine.next_cycle();
    }

    assert_eq!(engine.get_state().population(), &7);
}

#[test]
fn a_simple_autoselect_chain_should_always_resolve() {
    let mut engine = Engine::new(
        vec!["./tests/chains/a_simple_empty_chain.json".to_string()],
        Some("./tests/chains/"),
        PseudoRandomGenerator {},
    );

    let first_turn = engine.next_cycle();

    assert_matches!(first_turn, ViewModel::InGame(view) => {
        assert_eq!(view.events_by_chain[0].1[0], "Hello world!")
    });
}

#[test]
fn french_locale_should_translate_the_game_to_french() {
    let mut engine = Engine::new(
        vec!["./tests/chains/a_simple_empty_chain.json".to_string()],
        Some("./tests/chains/"),
        PseudoRandomGenerator {},
    );
    engine.change_language(Language::LocaleFrFr);

    let first_turn = engine.next_cycle();

    assert_matches!(first_turn, ViewModel::InGame(view) => {
        assert_eq!(view.events_by_chain[0].1[0], "Bonjour monde !")
    });
}
