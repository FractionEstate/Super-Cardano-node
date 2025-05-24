use crate::protocol::{Era, HardForkCombinator, ByronEra, ShelleyEra};

#[test]
fn test_hard_fork_transition() {
    let mut combinator = HardForkCombinator::new(Era::Byron, Box::new(ByronEra));
    combinator.schedule_transition(10, Era::Shelley, Box::new(ShelleyEra));

    assert_eq!(combinator.current_era(), &Era::Byron);
    combinator.check_transition(9);
    assert_eq!(combinator.current_era(), &Era::Byron);

    combinator.check_transition(10);
    assert_eq!(combinator.current_era(), &Era::Shelley);
}
