use crate::encode;
use actix::prelude::*;

pub struct WorkerPool {
    addr: Addr<TaskWorker>,
}

#[derive(Debug)]
pub struct TaskMessage {
    pub input_name: String,
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
        println!("task request message : {:?}", msg);
        encode::acc_encode(msg.input_name.as_str());
    }
}

impl WorkerPool {
    pub fn new(pool_size: usize) -> WorkerPool {
        WorkerPool {
            addr: SyncArbiter::start(pool_size, || TaskWorker),
        }
    }

    pub fn send(&self, msg: TaskMessage) {
        self.addr.do_send(msg);
    }
}
