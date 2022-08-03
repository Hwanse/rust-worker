use actix::prelude::*;
use std::time::Duration;
use std::thread;
use futures::executor::block_on;

#[derive(Clone)]
pub struct WorkerPool {
    addr: Addr<TaskWorker>
}

#[derive(Debug)]
pub struct TaskMessage {
    pub duration: u64
}

impl Message for TaskMessage {
    type Result = ();
}

struct TaskWorker;

impl Actor for TaskWorker {
    type Context = SyncContext<Self>;
}

impl Handler<TaskMessage> for TaskWorker {
    type Result = ();

    fn handle(&mut self, msg: TaskMessage, _ctx: &mut Self::Context) -> Self::Result {
        let id = thread::current().id();

        println!("{:?} : task work start.. (duration {})", id, msg.duration);
        thread::sleep(Duration::from_secs(msg.duration));
        println!("{:?} : task work end.. (duration {})", id, msg.duration);
    }
}

impl WorkerPool {
    pub fn new(count: usize) -> WorkerPool {
        let addr = SyncArbiter::start(count, || TaskWorker);
        WorkerPool {
            addr
        }
    }

    pub fn send(&self, msg: TaskMessage) {
        self.addr.do_send(msg);
        // block_on(async {
        //     self.addr.do_send(msg);
        // });
    }
}

