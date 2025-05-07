# Clockabilly

![an AI-generated bird with gears and a clock in its belly](https://raw.githubusercontent.com/acrlabs/clockabilly/master/img/clockabilly.jpg)

A really simple trait wrapper for [chrono](https://docs.rs/chrono/latest/chrono/) that lets you mock out time in tests.

## Usage

Any place you want to mock time, you need to pass in a `Clockable` interface.  In your "real" code you can use a
`UtcClock`, and in your testing code you can use a `MockUtcClock` (you need to include the `mock` feature for this).
See the `basic` example for an example:

```
> cargo run --example basic --features=mock
   Compiling clockabilly v0.0.0 (/build/src/clockabilly)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/basic`
2024-07-17T17:41:02.369894602Z
1970-01-01T00:00:10Z
```

For "simplicity", this re-exports all of `chrono`.

### Sleeping

You can also use this in place of calls to `tokio::time::sleep`; the real clock is just a wrapper around `sleep`, and
the fake clock just advances the internal time counter by however many seconds you slept for.

### Callbacks

You can provide callback functions that change external state using the `add_callbacks` function, e.g.,

```
let mut x = 10;
let mut clock = MockUtcClock::new(0);
clock.add_callback(1, || x += 5); // Probably doesn't actually compile, because you probably need Arcs and stuff
clock.sleep(20);
assert_eq!(15, x);
```

You can add multiple callbacks for each timestamp; callbacks for a particular timestamp are executed in the order they
were added.  Note that `sleep` and `advance` both execute the callbacks, but `set` does not.
