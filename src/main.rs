#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;
use std::collections::HashSet;
mod styles;
use discord_rich_presence::{activity, new_client, DiscordIpc};
use iced::{
    button, text_input, Align, Application, Button, Clipboard, Color, Column, Command, Element,
    Row, Settings, Space, Text, TextInput,
};
use notify_rust::{Notification, Urgency};
use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

use styles::{DiscordDefaultButton, DiscordGreenButton, DiscordRedButton, DiscordTextInput};

#[derive(Default)]
struct ClientIdState {
    st: text_input::State,
    val: String,
    valid: bool,
}

#[derive(Default, Clone)]
struct RpcPayloadStates {
    state_state: text_input::State,
    details_state: text_input::State,
}

#[derive(Default, Clone)]
struct RpcPayloadData {
    state_value: Option<String>,
    details_value: Option<String>,
}

#[derive(Default, Clone)]
struct ClientIdValidState {
    st: button::State,
    status: Option<bool>,
}

#[derive(Default, Clone)]
struct StartStopButtonState {
    start_state: button::State,
    status: Option<bool>,
    stop_state: button::State,
}

struct ThreadHandling {
    tb: std::thread::Builder,
    joiner: Option<std::thread::JoinHandle<()>>,
}

struct Counter {
    client_id: ClientIdState,
    client_id_valid: ClientIdValidState,
    dipc: Arc<Mutex<dyn DiscordIpc + Send + Sync>>,
    tb: ThreadHandling,
    stspbs: StartStopButtonState,
    data: Arc<Mutex<RpcPayloadData>>,
    dat_state: RpcPayloadStates,
}

#[derive(Debug, Clone)]
enum Message {
    StartApp(String),
    ClientIdInputChanged(String),
    ValidateClientId,
    StartRpcServer,
    StopRpcServer,
    RpcDataStateUpdate(String),
    RpcDataDetailsUpdate(String),
}

// fn get_cvv_styl(stat: Option<bool>) -> button::StyleSheet {
//     match stat {
//         Some(v) => {
//             if v {
//                 return DiscordRedButton;
//             }
//             return DiscordGreenButton;
//         }
//         None => return DiscordDefaultButton,
//     }
// }

pub async fn startup_bs() -> String {
    "Started Software!".to_string()
}

impl Application for Counter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flag: ()) -> (Self, iced::Command<Message>) {
        let builder = std::thread::Builder::new();
        let rpcc = match new_client("731140044970262590") {
            Ok(a) => a,
            Err(_e) => panic!("Unable to create rpc client"),
        };

        (
            Self {
                client_id: ClientIdState::default(),
                client_id_valid: ClientIdValidState::default(),
                dipc: Arc::new(Mutex::new(rpcc)),
                stspbs: StartStopButtonState::default(),
                data: Arc::new(Mutex::new(RpcPayloadData::default())),
                dat_state: RpcPayloadStates::default(),
                tb: ThreadHandling {
                    tb: builder,
                    joiner: None,
                },
            },
            Command::perform(startup_bs(), Message::StartApp),
        )
    }

    fn title(&self) -> String {
        String::from("Discord-RPC-GUI")
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::from_rgb(0.172, 0.184, 0.2)
    }

    fn update(&mut self, message: Message, _: &mut Clipboard) -> Command<Message> {
        match message {
            Message::StartApp(_) => {
                println!("App Start Recieved!");
            }

            Message::ValidateClientId => {
                let (tp,notif_m, urg) = match self.client_id.val.parse::<u64>() {
                    Ok(_v) => (true, "Valid Client ID provided! You may start RPC", Urgency::Normal),
                    Err(_e) => (false, "InValid Client ID provided. RPC will not work untill changed and re-validated",Urgency::Critical),
                };
                self.client_id.valid = tp;
                self.client_id_valid.status = Some(tp);
                Notification::new()
                    .summary("Discord-RPC-GUI")
                    .body(notif_m)
                    .show()
                    .unwrap();
                println!("Help");
            }
            Message::ClientIdInputChanged(v) => {
                self.client_id.val = v;
            }
            Message::StartRpcServer => {
                let vca = self.client_id.valid;
                let can_start = match self.stspbs.status {
                    Some(v) => {
                        if v {
                            false
                        } else {
                            true
                        }
                    }
                    None => true,
                };
                if can_start && vca {
                    let parec = Arc::clone(&self.dipc);
                    let rpc_data_d = Arc::clone(&self.data);
                    self.stspbs.status = Some(true);
                    // let join_handle = &self
                    //     .tb
                    //     .tb
                    std::thread::spawn(move || {
                        let mut c = parec.lock().unwrap();
                        let rpc_data = rpc_data_d.lock().unwrap();
                        let o = match c.connect() {
                            Ok(_o) => {
                                println!("Client Connec");
                                true
                            }
                            Err(_e) => {
                                println!("Errpr");
                                false
                            }
                        };
                        if o {
                            let mut act = activity::Activity::new();
                            if let Some(state) = &rpc_data.state_value {
                                act = act.state(&state);
                            }
                            if let Some(details) = &rpc_data.details_value {
                                act = act.details(&details);
                            }

                            match c.set_activity(act) {
                                Ok(_a) => println!("Set Activity"),
                                Err(_eaa) => println!("Error setting activity"),
                            };
                        };
                    });
                    Notification::new()
                        .summary("Discord-RPC-GUI")
                        .body("Started RPC server. Check discord for activity")
                        .show()
                        .unwrap();
                } else if can_start {
                    Notification::new()
                        .summary("Discord-RPC-GUI")
                        .body("Unable to Start Discord RPC because no valid appplication id has been set. Enter an pplication id and hit the validate button.")
                        .show()
                        .unwrap();
                } else {
                    Notification::new()
                        .summary("Discord-RPC-GUI")
                        .body("Unable to Start Discord RPC because the server is aldready running. Please stop server before starting again.")
                        .show()
                        .unwrap();
                }
                //     .unwrap();
                // self.tb.joiner = Some(join_handle);
            }
            Message::StopRpcServer => {
                println!("TRY STOP");
                let can_stop = match self.stspbs.status {
                    Some(v) => {
                        if v {
                            true
                        } else {
                            false
                        }
                    }
                    None => false,
                };
                if can_stop {
                    self.stspbs.status = Some(false);
                    match Arc::clone(&self.dipc).lock() {
                        Ok(mut a) => {
                            match a.close() {
                                Ok(_b) => println!("Closed!"),
                                Err(_e) => println!("Unable to close"),
                            };
                        }
                        Err(_e) => println!("Unable to get lock"),
                    };
                    // match self.tb.joiner {
                    // Some(jh) => {
                    //     jh.join().expect("Thread hath been shut");
                    // }
                    // None => {
                    //     println!("no thread to join");
                    // }
                }
            }

            Message::RpcDataStateUpdate(v) => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                vals.state_value = Some(v);
            }
            Message::RpcDataDetailsUpdate(v) => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                vals.details_value = Some(v);
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let cbs = &mut self.client_id_valid;
        let spspt = &mut self.stspbs;
        let acd_data = &mut self.dat_state;
        let acd_data_arc = Arc::clone(&self.data);
        let acd_data_data = acd_data_arc.lock().unwrap();

        let client_validty_message = match &cbs.status {
            Some(v) => {
                if *v {
                    Text::new("Valid Application ID!").color(Color::from_rgb(0.0, 1.0, 0.0))
                } else {
                    Text::new("Invalid Application ID. Enter and validate again")
                        .color(Color::from_rgb(1.0, 0.0, 0.0))
                }
            }
            None => {
                Text::new("Enter Application ID from discord developer portal and click validate.")
                    .color(Color::WHITE)
            }
        };

        Column::new()
            .padding(20)
            .align_items(Align::Start)
            .push(
                Text::new("Discord-RPC-GUI")
                    .color(iced::Color::WHITE)
                    .size(50),
            )
            .push(Space::with_height(iced::Length::Units(10)))
            .push(
                TextInput::new(
                    &mut self.client_id.st,
                    "Enter Client ID",
                    &self.client_id.val,
                    Message::ClientIdInputChanged,
                )
                .style(DiscordTextInput)
                .padding(20),
            )
            .push(client_validty_message)
            .push(Space::with_height(iced::Length::Units(20)))
            .push(
                Button::new(&mut cbs.st, Text::new("Validate Client Id"))
                    .style(DiscordDefaultButton)
                    .padding(10)
                    .on_press(Message::ValidateClientId),
            )
            .push(Space::with_height(iced::Length::Units(60)))
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .push(
                        Column::new()
                            .width(iced::Length::FillPortion(4))
                            .push(
                                Row::new()
                                    .push(Text::new("State").color(Color::WHITE).size(25))
                                    .push(
                                        Text::new("*")
                                            .color(Color::from_rgb(1.0, 0.0, 0.0))
                                            .size(25),
                                    )
                                    .push(Space::with_width(iced::Length::Units(20)))
                                    .push(
                                        TextInput::new(
                                            &mut acd_data.state_state,
                                            "Activity State",
                                            &acd_data_data
                                                .state_value
                                                .as_ref()
                                                .unwrap_or(&"".to_string()),
                                            Message::RpcDataStateUpdate,
                                        )
                                        .style(DiscordTextInput)
                                        .padding(15),
                                    ),
                            )
                            .push(Text::new("b").color(Color::WHITE)),
                    )
                    .push(Space::with_width(iced::Length::Units(20)))
                    .push(
                        Column::new()
                            .width(iced::Length::FillPortion(6))
                            .push(
                                Row::new()
                                    .push(Text::new("Details").color(Color::WHITE).size(25))
                                    .push(
                                        Text::new("*")
                                            .color(Color::from_rgb(1.0, 0.0, 0.0))
                                            .size(25),
                                    )
                                    .push(
                                        TextInput::new(
                                            &mut acd_data.details_state,
                                            "Activity Details",
                                            &acd_data_data
                                                .details_value
                                                .as_ref()
                                                .unwrap_or(&"".to_string()),
                                            Message::RpcDataDetailsUpdate,
                                        )
                                        .style(DiscordTextInput)
                                        .padding(15),
                                    ),
                            )
                            .push(Text::new("a").color(Color::WHITE)),
                    ),
            )
            .push(Space::with_height(iced::Length::Units(40)))
            .push(
                Row::new()
                    .push(
                        Button::new(&mut spspt.start_state, Text::new("Start RPC"))
                            .style(DiscordGreenButton)
                            .padding(10)
                            .on_press(Message::StartRpcServer),
                    )
                    .push(Space::with_width(iced::Length::Units(20)))
                    .push(
                        Button::new(&mut spspt.stop_state, Text::new("Stop RPC"))
                            .style(DiscordRedButton)
                            .padding(10)
                            .on_press(Message::StopRpcServer),
                    ),
            )
            .into()
    }
}
