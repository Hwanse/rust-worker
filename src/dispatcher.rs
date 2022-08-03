use actix_rt::Arbiter;
use serde::Deserialize;
use amiquip::{Channel, Connection, Consumer, ConsumerMessage, ConsumerOptions, FieldTable, Queue, QueueDeclareOptions, Result};
use worker::{WorkerPool, TaskMessage};
use crate::worker;

const TASK_QUEUE_NAME: &str = "task";

pub struct TaskDispatcher{
    pub pool: WorkerPool
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaskRequest {
    #[serde(default)]
    duration: u64
}

impl TaskDispatcher {

    pub fn run(&self) {
        // Open connection.
        let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672").unwrap();

        // Open a channel - None says let the library choose the channel ID.
        let channel = connection.open_channel(None).unwrap();

        // Declare the durable queue we will consume from.
        let queue = channel.queue_declare(
            TASK_QUEUE_NAME,
            QueueDeclareOptions {
                durable: false,
                ..QueueDeclareOptions::default()
            },
        ).unwrap();

        // Set QOS to only send us 1 message at a time.
        channel.qos(0, 1, false).unwrap();

        // Start a consumer.
        let consumer = queue.consume(ConsumerOptions::default()).unwrap();
        println!("Waiting for messages. Press Ctrl-C to exit.");

        for (_i, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let task: TaskRequest = serde_json::from_slice(&delivery.body).unwrap();
                    self.pool.send(TaskMessage { duration: task.duration });
                    consumer.ack(delivery).unwrap();
                }
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }

        connection.close();
    }

}