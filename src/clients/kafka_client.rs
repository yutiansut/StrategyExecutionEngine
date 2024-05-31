/*******************************************************************************
Copyright (c) 2024.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
******************************************************************************/

/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/5/24
******************************************************************************/

use crate::MessagingClient;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::message::Message;
use rdkafka::producer::{BaseProducer, BaseRecord};
use std::error::Error;
use tokio::time::{sleep, Duration};

pub struct KafkaClient {
    producer: BaseProducer,
    consumer: BaseConsumer,
    brokers: String,
    group_id: String,
}

impl KafkaClient {
    pub fn new(brokers: String, group_id: String) -> Self {
        // Create producer configuration
        let producer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .create()
            .expect("Producer creation error");

        // Create consumer configuration
        let consumer = ClientConfig::new()
            .set("group.id", &group_id)
            .set("bootstrap.servers", &brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .create()
            .expect("Consumer creation error");

        KafkaClient {
            producer,
            consumer,
            brokers,
            group_id,
        }
    }
}

impl MessagingClient for KafkaClient {
    fn produce(&self, topic: &str, message: &str) -> Result<(), String> {
        let record = BaseRecord::to(topic).payload(message);
        self.producer.send(record).map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn consume(&self, topic: &str) -> Result<String, String> {
        self.consumer
            .subscribe(&[topic])
            .map_err(|e| e.to_string())?;

        for _ in 0..5 {
            match self.consumer.poll(Duration::from_secs(1)) {
                Some(Ok(m)) => {
                    if let Some(payload) = m.payload() {
                        return Ok(String::from_utf8_lossy(payload).to_string());
                    }
                }
                Some(Err(e)) => return Err(e.to_string()),
                None => sleep(Duration::from_millis(100)).await,
            }
        }
        Err("No message received".to_string())
    }
}
