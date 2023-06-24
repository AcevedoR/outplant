use unnamed_game;

#[test]
fn population_should_grow_if_no_event() {
    let mut engine = unnamed_game::engine::Engine::new();

    engine.play_next_cycle();

    assert_eq!(engine.get_state().population, 2);
}
