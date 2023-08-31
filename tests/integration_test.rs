use unnamed_game::engine::engine::Engine;
use unnamed_game::engine::random::PseudoRandomGenerator;

#[test]
fn population_should_grow_if_no_event() {
    let mut engine = Engine::new(
        vec!["./tests/chains/never_triggers.json".to_string()],
        PseudoRandomGenerator {},
    );

    for _i in 0..6 {
        engine.next_cycle();
    }

    assert_eq!(engine.get_state().population(), &7);
}
