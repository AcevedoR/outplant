use unnamed_game;
use unnamed_game::engine::engine::Engine;
use unnamed_game::engine::random::PseudoRandomGenerator;

#[test]
fn tests_should_be_run_with_integration_test_feature() {
    #[cfg(not(feature = "integration-test"))]
    panic!("'integration-test' feature is mandatory to run tests");
}


#[test]
fn population_should_grow_if_no_event() {
    let mut engine = Engine::new("./tests/event_chains/never_triggers.json", PseudoRandomGenerator{});

    for _i in 0..7 {
        engine.next_cycle();
    }

    assert_eq!(engine.get_state().population, 8);
}
