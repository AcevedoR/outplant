use unnamed_game::{self, random::PseudoRandomGenerator};

#[test]
fn population_should_grow_if_no_event() {
    let mut engine = unnamed_game::engine::Engine::new(PseudoRandomGenerator{});

    engine.next_cycle();

    assert_eq!(engine.get_state().population, 2);
}
