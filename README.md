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
