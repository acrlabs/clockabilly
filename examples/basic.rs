use clockabilly::mock::MockUtcClock;
use clockabilly::*;

fn do_something(clock: &(dyn Clockable)) {
    let t = clock.now();
    println!("{t:?}");
}

fn main() {
    do_something(&UtcClock);

    let mock_clock = MockUtcClock::new(10);
    do_something(&mock_clock);
}
