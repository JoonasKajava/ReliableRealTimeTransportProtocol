use std::sync::Arc;
use std::sync::mpsc::Sender;

use crate::application_layer::message::{Message, MessageTypeTrait};
use crate::transport_layer::ExtractUDPData;
use crate::transport_layer::window::Window;

#[derive(Debug)]
pub enum ConnectorEvents<TMessage: MessageTypeTrait> {
    MessageReceived(Message<TMessage>),
    MessageSent(Message<TMessage>),
}

pub struct Connector<TMessage: MessageTypeTrait> {
    window: Arc<Window>,
    connector_events_sender: Sender<ConnectorEvents<TMessage>>,
}


impl<TMessage: MessageTypeTrait + 'static> Connector<TMessage> {
    pub fn new(local_addr: &str) -> std::io::Result<(Self, std::sync::mpsc::Receiver<ConnectorEvents<TMessage>>)> {
        let window = Window::new(local_addr)?;
        let new_window = Arc::new(window.0);
        let (sender, receiver) = std::sync::mpsc::channel();
        let connector = Self {
            window: new_window.clone(),
            connector_events_sender: sender,
        };
        

        Window::listen(new_window.clone());
        
        let sender_clone = connector.connector_events_sender.clone();

        std::thread::spawn(move || {
            loop {
                let message = window.1.recv().unwrap();
                let message = Message::from(message.as_slice());
                sender_clone.send(ConnectorEvents::MessageReceived(message)).unwrap();
            }
        });

        Ok((connector, receiver))
    }

    pub fn send(&self, request: Message<TMessage>) -> std::io::Result<usize> {
        self.connector_events_sender.send(ConnectorEvents::MessageSent(request.clone())).unwrap();
        let payload = request.consume_udp_data();
        let result = self.window.send(payload.as_slice());
        result
    }

    pub fn connect(&self, remote_addr: &str) -> std::io::Result<()> {
        self.window.connect(remote_addr)
    }

}