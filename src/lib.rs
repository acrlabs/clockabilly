use async_trait::async_trait;
pub use chrono::*;
use tokio::time;

// This trait exists for testing, so that we can provide consistent timestamp values to objects
// instead of just relying on whatever the current time actually is.

#[async_trait]
pub trait Clockable {
    fn now(&self) -> DateTime<Utc>;
    fn now_ts(&self) -> i64;
    async fn sleep(&self, seconds: i64);
}

#[derive(Clone)]
pub struct UtcClock;

impl UtcClock {
    pub fn new() -> UtcClock {
        UtcClock
    }

    pub fn boxed() -> Box<UtcClock> {
        Box::new(UtcClock)
    }
}

#[async_trait]
impl Clockable for UtcClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    fn now_ts(&self) -> i64 {
        Utc::now().timestamp()
    }

    async fn sleep(&self, seconds: i64) {
        time::sleep(time::Duration::from_secs(seconds as u64)).await;
    }
}

pub mod prelude {
    pub use super::Clockable;
    pub use super::UtcClock;
}

#[cfg(feature = "mock")]
pub mod mock {
    use std::collections::BTreeMap;
    use std::sync::atomic::{AtomicI64, Ordering};
    use std::sync::Arc;
    use std::sync::Mutex;

    use super::*;

    #[derive(Clone)]
    pub struct MockUtcClock {
        now: Arc<AtomicI64>,
        // This is _gross AF_ :vomit_face:
        callbacks: Arc<Mutex<BTreeMap<i64, Vec<Arc<Mutex<dyn FnMut() -> () + Send + Sync>>>>>>,
    }

    impl MockUtcClock {
        pub fn new(start_ts: i64) -> MockUtcClock {
            MockUtcClock {
                now: Arc::new(AtomicI64::new(start_ts)),
                callbacks: Arc::new(Mutex::new(BTreeMap::new())),
            }
        }

        pub fn boxed(start_ts: i64) -> Box<MockUtcClock> {
            Box::new(MockUtcClock::new(start_ts))
        }

        pub fn add_callback(&mut self, ts: i64, f: impl FnMut() -> () + Send + Sync + 'static) {
            self.callbacks
                .lock()
                .unwrap()
                .entry(ts)
                .or_insert(vec![])
                .push(Arc::new(Mutex::new(f)));
        }

        pub fn advance(&mut self, duration: i64) -> i64 {
            let old = self.now.fetch_add(duration, Ordering::Relaxed);
            let now = old + duration;
            self.run_callbacks();
            now
        }

        pub fn set(&mut self, ts: i64) -> i64 {
            self.now.store(ts, Ordering::Relaxed);
            ts
        }

        fn run_callbacks(&self) {
            let mut callbacks = self.callbacks.lock().unwrap();
            let now = self.now.load(Ordering::Relaxed);
            let new_callbacks = callbacks.split_off(&now);
            for fns in callbacks.values() {
                for f in fns {
                    f.lock().unwrap()();
                }
            }
            *callbacks = new_callbacks;
        }
    }

    #[async_trait]
    impl Clockable for MockUtcClock {
        fn now(&self) -> DateTime<Utc> {
            return DateTime::from_timestamp(self.now_ts(), 0).unwrap();
        }

        fn now_ts(&self) -> i64 {
            return self.now.load(Ordering::Relaxed);
        }

        async fn sleep(&self, seconds: i64) {
            self.now.fetch_add(seconds, Ordering::Relaxed);
            self.run_callbacks();
        }
    }
}
