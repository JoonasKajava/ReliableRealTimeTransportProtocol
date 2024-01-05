use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::SystemTime;
use iced::{Alignment, Application, Color, Command, Element, Executor, executor, Sandbox, Theme};
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use log::error;
use lib_rrttp::window::Window;
use crate::ui::message_subscriber::{Event, subscribe_to_messages};

use crate::ui::status::{ConnectionStatus, LocalSocketStatus};

pub struct RRTPApplication {
    local_address: String,
    remote_address: String,

    message_to_send: String,

    sending_window: Arc<RwLock<Window>>,

    local_socket_status: LocalSocketStatus,
    connection_status: ConnectionStatus,

    socket_listen_handle: Option<thread::JoinHandle<()>>,
    socket_message_reader_handle: Option<thread::JoinHandle<()>>,

    log: Arc<Mutex<Vec<String>>>,
}

impl RRTPApplication {
    pub fn append_log(&self, message: String) {
        let mut log = self.log.lock().unwrap();
        log.push(format!("[{}] {}", humantime::format_rfc3339_seconds(SystemTime::now()), message));
    }

    pub fn append_to_log_vector(log: &Arc<Mutex<Vec<String>>>, message: String) {
        let mut log = log.lock().unwrap();
        log.push(format!("[{}] {}", humantime::format_rfc3339_seconds(SystemTime::now()), message));
    }

    pub fn read_incoming_messages(&mut self) {
        self.socket_listen_handle = Some(self.sending_window.read().unwrap().listen());

        let sending_window = Arc::clone(&self.sending_window);
        let incoming = sending_window.read().unwrap().incoming_messages().clone();
        let log = Arc::clone(&self.log);
        self.socket_message_reader_handle =  Some(thread::spawn(move || {
            loop {
                let message_body = incoming.lock().unwrap().recv().unwrap();
                RRTPApplication::append_to_log_vector(&log,format!("Received message: {}", String::from_utf8_lossy(message_body.as_slice())));
            }
        }));
    }
}


#[derive(Debug, Clone)]
pub enum Message {
    LocalAddressChanged(String),
    RemoteAddressChanged(String),
    OnMessageChanged(String),
    IncomingMessage(Event),
    SendMessage,
    StartLocalSocket,
    StopLocalSocket,
    Connect,
    Disconnect,
}

impl Application for RRTPApplication {
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self {
            local_address: "".to_string(),
            remote_address: "".to_string(),
            message_to_send: "".to_string(),
            sending_window: Default::default(),
            connection_status: Default::default(),
            local_socket_status: Default::default(),
            log: Default::default(),
            socket_listen_handle: None,
            socket_message_reader_handle: None,
        }, Command::none())
    }

    fn title(&self) -> String {
        "RRTTP".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Connect => {
                match self.sending_window.read().unwrap().connect(self.remote_address.as_str()) {
                    Ok(_) => {
                        self.append_log(format!("Connected to {}", self.remote_address));
                        self.connection_status = ConnectionStatus::Connected;
                    }
                    Err(e) => {
                        self.append_log(format!("Failed to connect to remote address: {}", e));
                        error!("Failed to connect to remote address: {}", e)
                    }
                };
            }
            Message::Disconnect => {}
            Message::LocalAddressChanged(value) => {
                self.local_address = value;
            }
            Message::RemoteAddressChanged(value) => {
                self.remote_address = value;
            }
            Message::StopLocalSocket => {}
            Message::StartLocalSocket => {
                let bind_result =
                {
                    let mut sending_window = self.sending_window.write().unwrap();
                     sending_window.bind_local_socket(self.local_address.as_str())
                };
                match bind_result {
                    Ok(_) => {
                        self.append_log(format!("Bound to {}", self.local_address));
                        self.local_socket_status = LocalSocketStatus::Started;
                        self.read_incoming_messages();

                    }
                    Err(e) =>
                        {
                            self.append_log(format!("Failed to bind local socket: {}", e));
                            error!("Failed to bind local socket: {}", e)
                        }
                };
            }
            Message::OnMessageChanged(message) => {
                self.message_to_send = message;
            }
            Message::SendMessage => {
                match self.sending_window.write().unwrap().send(self.message_to_send.as_bytes()) {
                    Ok(_) => {
                        self.append_log(format!("Sent message: {}", self.message_to_send));
                    }
                    Err(e) => {
                        self.append_log(format!("Failed to send message: {}", e));
                        error!("Failed to send message: {}", e)
                    }
                }
            }
            Message::IncomingMessage(message) => {
                match message {
                    Event::MessageReceived(content) => self.append_log(content),
                }
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let local_socket_address_input = row![
            text("Local address"),
            text_input("127.0.0.1:12345", self.local_address.as_str()).on_input(Message::LocalAddressChanged).width(iced::Length::Fixed(200f32)),
            button("Start local socket").on_press(Message::StartLocalSocket),
        ]
            .align_items(Alignment::Center)
            .spacing(10);
        let remote_address_input = row![
            text("Remote address"),
            text_input("127.0.0.1:12345", self.remote_address.as_str()).on_input(Message::RemoteAddressChanged).width(iced::Length::Fixed(200f32)),
            button("Connect to remote address").on_press(Message::Connect),
        ]
            .align_items(Alignment::Center)
            .spacing(10);


        let mut local_socket_status = row![
            text("Local socket: "),
            text({match self.local_socket_status {
                LocalSocketStatus::Started => "Started",
                LocalSocketStatus::Stopped => "Stopped"
                }}),

        ].align_items(Alignment::Center)
            .spacing(10);

        if self.local_socket_status == LocalSocketStatus::Started {
            local_socket_status = local_socket_status.push(button("Stop").on_press(Message::StopLocalSocket));
        }

        let connection_status = row![
            text("Connection status: "),
            text({match self.connection_status {
                ConnectionStatus::Connected => "Connected",
                ConnectionStatus::Disconnected => "Disconnected"
                }}),
            button("Disconnect").on_press(Message::Disconnect),
        ].align_items(Alignment::Center)
            .spacing(10);

        let status_row = column![local_socket_status, connection_status]
            .width(iced::Length::Fill);

        let mut log_items = column![];

        for item in self.log.lock().unwrap().iter() {
            log_items = log_items.push(text(item));
        }

        let log = scrollable(
            log_items
        ).width(iced::Length::Fill).height(iced::Length::Fill);



        let send_message = row![
            text("Send message"),
            text_input("Message", self.message_to_send.as_str()).on_input(Message::OnMessageChanged).width(iced::Length::Fill),
            button("Send").on_press(Message::SendMessage),
        ].align_items(Alignment::Center)
            .spacing(10);

        let messaging = column![
            send_message
        ].width(iced::Length::Fill).height(iced::Length::Fill);

        let log_and_messaging = row![
            log,
            messaging
        ].width(iced::Length::Fill).height(iced::Length::Fill);

        let content = column![
            status_row,
            local_socket_address_input,
            remote_address_input,
            log_and_messaging
        ].spacing(10);

        let container_content: Element<_> = container(
            content
        ).padding(10).width(iced::Length::Fill)
            .into();
        container_content
            .explain(Color::BLACK)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let arc = self.sending_window.read().unwrap().incoming_messages();
        let message = arc.lock().unwrap();
        for item in message.iter() {

            self.append_log(format!("Received message: {}", String::from_utf8_lossy(item.as_slice())));
        }
        subscribe_to_messages().map(Message::IncomingMessage)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
}