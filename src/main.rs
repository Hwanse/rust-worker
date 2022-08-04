extern crate taskworker;

use std::thread;
use actix::prelude::*;
use taskworker::dispatcher::{TaskDispatcher};
use taskworker::worker::WorkerPool;

fn main() {
    let system = System::new();
    let pool = WorkerPool::new(3);
    let dispatcher = TaskDispatcher{ pool };

    // start dispatcher on other thread
    thread::spawn(move || dispatcher.run());

    // run actix_rt event loop
    system.run().expect("event loop panic");
}