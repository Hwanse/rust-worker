extern crate taskworker;

use actix_rt::{Arbiter, System};
use taskworker::dispatch::TaskDispatch;
use taskworker::worker::WorkerPool;

const WORKER_COUNT: usize = 3;

fn main() {
    let system = System::new();
    let pool = WorkerPool::new(WORKER_COUNT);
    let dispatch = TaskDispatch { pool };

    // start dispatcher
    let arbiter = Arbiter::new();
    arbiter.spawn_fn(move || dispatch.run());

    // run actix_rt runtime
    system.run().expect("event loop panic");
}
