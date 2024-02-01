use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::transport_layer::connection_manager::{ConnectionEventType, SequenceNumber};
use crate::transport_layer::socket::Socket;

pub struct QueueSender {
    sender_thread_handle: Option<thread::JoinHandle<()>>,
}

// TODO: Rewrite this completely to allow only one thread to send messages
// IE SINGLE SENDER WINDOW AND SINGLE RECEIVER WINDOW
// Doing multiple channels is a bad idea
// Sending large file will block sending other messages

impl QueueSender {
    pub fn start(
        &self,
        socket: Arc<Socket>,
        event_sender: Sender<ConnectionEventType>,
        ack_receiver: std::sync::mpsc::Receiver<SequenceNumber>,
        queue: Receiver<Vec<u8>>) -> Self {


        let ack_manager_handle = thread::spawn(move || {
            loop {
                for i in ack_receiver.try_iter() {
                    // TODO: Handle acks
                }
            }
        });

        let message_sender_thread_handle = thread::spawn(move || {
            loop {
                let next_message = queue.recv().unwrap();
            }
        });

        Self {
            sender_thread_handle: Some(message_sender_thread_handle),
        }
    }
}


impl Drop for QueueSender {
    fn drop(&mut self) {
        if let Some(handle) = self.sender_thread_handle.take() {
            handle.join().expect("Failed to join thread");
        }
    }
}