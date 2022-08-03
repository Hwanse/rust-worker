extern crate taskworker;

use std::thread;
use actix::prelude::*;
use taskworker::dispatcher::{TaskDispatcher};
use taskworker::worker::WorkerPool;

fn main() {
    let system = System::new();
    let pool = WorkerPool::new(3);
    let dispatcher = TaskDispatcher{ pool };

    thread::spawn(move || dispatcher.run());

    system.run();
}