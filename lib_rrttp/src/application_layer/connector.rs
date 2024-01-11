use std::sync::Arc;
use std::sync::mpsc::Sender;

use crate::application_layer::message::Message;
use crate::transport_layer::ExtractUDPData;
use crate::transport_layer::window::Window;

#[derive(Debug)]
pub enum ConnectorEvents {
    MessageReceived(Message),
    MessageSent(Message),
}

pub struct Connector {
    window: Arc<Window>,
    connector_events_sender: Sender<ConnectorEvents>,
}


impl Connector {
    pub fn new(local_addr: &str) -> std::io::Result<(Self, std::sync::mpsc::Receiver<ConnectorEvents>)> {
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

    pub fn send(&self, request: Message) -> std::io::Result<usize> {
        self.connector_events_sender.send(ConnectorEvents::MessageSent(request.clone())).unwrap();
        let payload = request.consume_udp_data();
        let result = self.window.send(payload.as_slice());
        result
    }
}