#[macro_use]
extern crate log;
mod styles;
use discord_rich_presence::{activity, new_client, DiscordIpc};
use iced::{
    button, scrollable, text_input, Align, Application, Button, Clipboard, Color, Column, Command,
    Element, Row, Scrollable, Settings, Space, Text, TextInput,
};
use notify_rust::Notification;
use std::sync::{Arc, Mutex};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

use styles::{
    DiscordDefaultButton, DiscordGreenButton, DiscordHiddenButton, DiscordRedButton,
    DiscordTextInput,
};

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
    small_image_key: Option<String>,
    small_image_text: Option<String>,
    large_image_key: Option<String>,
    large_image_text: Option<String>,
    button_a_text: Option<String>,
    button_a_url: Option<String>,
    button_b_text: Option<String>,
    button_b_url: Option<String>,
}

#[derive(Default, Clone)]
struct SmallImageStates {
    small_image_key: text_input::State,
    small_image_text: text_input::State,
    small_image_show_button: button::State,
    small_image_hide_button: button::State,
    small_image_show: bool,
}

#[derive(Default, Clone)]
struct ButtonAStates {
    buttona_text: text_input::State,
    buttona_url: text_input::State,
    buttona_show_button: button::State,
    buttona_hide_button: button::State,
    buttona_show: bool,
    buttona_valid: bool,
}

#[derive(Default, Clone)]
struct ButtonBStates {
    buttonb_text: text_input::State,
    buttonb_url: text_input::State,
    buttonb_show_button: button::State,
    buttonb_hide_button: button::State,
    buttonb_show: bool,
    buttonb_valid: bool,
}

#[derive(Default, Clone)]
struct LargeImageStates {
    large_image_key: text_input::State,
    large_image_text: text_input::State,
    large_image_show_button: button::State,
    large_image_hide_button: button::State,
    large_image_show: bool,
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
    smallimg: SmallImageStates,
    largeimg: LargeImageStates,
    buttona: ButtonAStates,
    buttonb: ButtonBStates,
    scroll_state: scrollable::State,
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
    RpcDataSmallTextUpdate(String),
    RpcDataSmallImageUpdate(String),
    RpcDataLargeTextUpdate(String),
    RpcDataLargeImageUpdate(String),
    RpcDataLargeButtonATextUpdate(String),
    RpcDataLargeButtonAUrlUpdate(String),
    RpcDataLargeButtonBTextUpdate(String),
    RpcDataLargeButtonBUrlUpdate(String),
    SmallImgShowUpdate,
    SmallImgHideUpdate,
    LargeImgShowUpdate,
    LargeImgHideUpdate,
    ButtonAShowUpdate,
    ButtonAHideUpdate,
    ButtonBShowUpdate,
    ButtonBHideUpdate,
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
        info!("Error");
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
                scroll_state: scrollable::State::default(),
                buttona: ButtonAStates::default(),
                buttonb: ButtonBStates::default(),
                smallimg: SmallImageStates {
                    small_image_show: false,
                    ..SmallImageStates::default()
                },
                largeimg: LargeImageStates {
                    large_image_show: false,
                    ..LargeImageStates::default()
                },
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
                let val_l = self.client_id.val.len() == 18;
                let (tp,notif_m) = match self.client_id.val.parse::<u64>() {
                    Ok(_v) => {
                        if val_l {
                        (true, "Valid Client ID provided! You may start RPC")
                    } else {
                        (false, "InValid Client ID provided. RPC will not work untill changed and re-validated")
                    }
                    },
                    Err(_e) => (false, "InValid Client ID provided. RPC will not work untill changed and re-validated"),
                };
                self.client_id.valid = tp;
                self.client_id_valid.status = Some(tp);
                if tp {
                    self.dipc = Arc::new(Mutex::new(new_client(&self.client_id.val).unwrap()))
                }
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
                let add_button_a = self.buttona.buttona_valid;
                let showba = self.buttona.buttona_show;
                let add_button_b = self.buttonb.buttonb_valid;
                let showbb = self.buttonb.buttonb_show;
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

                if !add_button_a && showba {
                    Notification::new()
                        .summary("Discord-RPC-GUI")
                        .body("The RPC will start without Button A, for the button config is invalid.")
                        .show()
                        .unwrap();
                };

                if !add_button_b && showbb {
                    Notification::new()
                        .summary("Discord-RPC-GUI")
                        .body("The RPC will start without Button B, for the button config is invalid.")
                        .show()
                        .unwrap();
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
                            let mut assets = activity::Assets::new();
                            let mut add_assets = false;
                            let mut buttons: Vec<activity::Button> = Vec::new();
                            let mut add_buts = false;
                            if let Some(state) = &rpc_data.state_value {
                                act = act.state(&state);
                            }
                            if let Some(details) = &rpc_data.details_value {
                                act = act.details(&details);
                            }
                            if let Some(s_key) = &rpc_data.small_image_key {
                                add_assets = true;
                                assets = assets.small_image(&s_key);
                            }
                            if let Some(s_text) = &rpc_data.small_image_text {
                                add_assets = true;
                                assets = assets.small_text(&s_text);
                            }
                            if let Some(l_key) = &rpc_data.large_image_key {
                                add_assets = true;
                                println!("Lagre image {}", l_key);
                                assets = assets.large_image(&l_key);
                            }
                            if let Some(l_text) = &rpc_data.large_image_text {
                                add_assets = true;
                                println!("Lagre image Text {}", l_text);
                                assets = assets.large_text(&l_text);
                            }
                            if let Some(ba_te) = &rpc_data.button_a_text {
                                if let Some(ba_url) = &rpc_data.button_a_url {
                                    if add_button_a {
                                        add_buts = true;
                                        println!(
                                            "{}",
                                            format!("Add Buttons {}  {}", ba_te, ba_url)
                                        );
                                        buttons.push(activity::Button::new(&ba_te, &ba_url));
                                    }
                                }
                            }
                            if let Some(bb_te) = &rpc_data.button_b_text {
                                if let Some(bb_url) = &rpc_data.button_b_url {
                                    if add_button_b {
                                        add_buts = true;
                                        println!(
                                            "{}",
                                            format!("Add Buttons B {}  {}", bb_te, bb_url)
                                        );
                                        buttons.push(activity::Button::new(&bb_te, &bb_url));
                                    }
                                }
                            }
                            if add_assets {
                                act = act.assets(assets);
                            }
                            if add_buts {
                                act = act.buttons(buttons);
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
                                Ok(_b) => {
                                    Notification::new()
                                        .summary("Discord-RPC-GUI")
                                        .body("Successfully stopped RPC server")
                                        .show()
                                        .unwrap();
                                    println!("Closed!")
                                }
                                Err(e) => {
                                    Notification::new()
                                        .summary("Discord-RPC-GUI")
                                        .body(&format!("Unable to stop RPC server {}", e))
                                        .show()
                                        .unwrap();
                                    println!("Unable to close")
                                }
                            };
                        }
                        Err(e) => {
                            Notification::new()
                                .summary("Discord-RPC-GUI")
                                .body(&format!("Unable to stop RPC server {}", e))
                                .show()
                                .unwrap();
                            println!("Unable to get lock")
                        }
                    };
                    // match self.tb.joiner {
                    // Some(jh) => {
                    //     jh.join().expect("Thread hath been shut");
                    // }
                    // None => {
                    //     println!("no thread to join");
                    // }
                } else {
                    Notification::new()
                        .summary("Discord-RPC-GUI")
                        .body("Cannot stop RPC server as there is no running server!")
                        .show()
                        .unwrap();
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
            Message::RpcDataSmallImageUpdate(v) => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                vals.small_image_key = Some(v);
            }
            Message::RpcDataSmallTextUpdate(v) => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                vals.small_image_text = Some(v);
            }
            Message::SmallImgShowUpdate => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                self.smallimg.small_image_show = true;
                vals.small_image_key = None;
                vals.small_image_text = None;
            }
            Message::SmallImgHideUpdate => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                self.smallimg.small_image_show = false;
                vals.small_image_key = None;
                vals.small_image_text = None;
            }
            Message::RpcDataLargeImageUpdate(v) => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                vals.large_image_key = Some(v);
            }
            Message::RpcDataLargeTextUpdate(v) => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                vals.large_image_text = Some(v);
            }
            Message::RpcDataLargeButtonATextUpdate(v) => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                vals.button_a_text = Some(v);
            }
            Message::RpcDataLargeButtonAUrlUpdate(v) => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                vals.button_a_url = Some(v);
            }
            Message::RpcDataLargeButtonBTextUpdate(v) => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                vals.button_b_text = Some(v);
            }
            Message::RpcDataLargeButtonBUrlUpdate(v) => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                vals.button_b_url = Some(v);
            }
            Message::LargeImgShowUpdate => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                self.largeimg.large_image_show = true;
                vals.large_image_key = None;
                vals.large_image_text = None;
            }
            Message::LargeImgHideUpdate => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                self.largeimg.large_image_show = false;
                vals.large_image_key = None;
                vals.large_image_text = None;
            }
            Message::ButtonAShowUpdate => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                self.buttona.buttona_show = true;
                vals.button_a_text = None;
                vals.button_a_url = None;
            }
            Message::ButtonAHideUpdate => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                self.buttona.buttona_show = false;
                vals.button_b_text = None;
                vals.button_b_url = None;
            }
            Message::ButtonBShowUpdate => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                self.buttonb.buttonb_show = true;
                vals.button_b_text = None;
                vals.button_b_url = None;
            }
            Message::ButtonBHideUpdate => {
                let vals_a = Arc::clone(&self.data);
                let mut vals = vals_a.lock().unwrap();
                self.buttonb.buttonb_show = false;
                vals.button_b_text = None;
                vals.button_b_url = None;
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let cbs = &mut self.client_id_valid;
        let spspt = &mut self.stspbs;
        let acd_data = &mut self.dat_state;
        let smallimgd = &mut self.smallimg;
        let largeimgd = &mut self.largeimg;
        let buttona = &mut self.buttona;
        let buttonb = &mut self.buttonb;
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

        let small_img_portion = match smallimgd.small_image_show {
            true => Column::new()
                .push(
                    Button::new(
                        &mut smallimgd.small_image_hide_button,
                        Text::new("- Small Image").color(Color::WHITE),
                    )
                    .padding(5)
                    .style(DiscordDefaultButton)
                    .on_press(Message::SmallImgHideUpdate),
                )
                .push(Space::with_height(iced::Length::Units(40)))
                .push(
                    Row::new()
                        .push(Text::new("Image Key").color(Color::WHITE).size(25))
                        .push(Space::with_width(iced::Length::Units(20)))
                        .push(
                            TextInput::new(
                                &mut smallimgd.small_image_key,
                                "Small Image Key.",
                                &acd_data_data
                                    .small_image_key
                                    .as_ref()
                                    .unwrap_or(&"".to_string()),
                                Message::RpcDataSmallImageUpdate,
                            )
                            .style(DiscordTextInput)
                            .padding(15),
                        )
                        .push(Space::with_height(iced::Length::Units(60)))
                        .push(Text::new("Image Text").color(Color::WHITE).size(25))
                        .push(Space::with_width(iced::Length::Units(20)))
                        .push(
                            TextInput::new(
                                &mut smallimgd.small_image_text,
                                "Small Image Text.",
                                &acd_data_data
                                    .small_image_text
                                    .as_ref()
                                    .unwrap_or(&"".to_string()),
                                Message::RpcDataSmallTextUpdate,
                            )
                            .style(DiscordTextInput)
                            .padding(15),
                        ),
                ),
            false => Column::new().push(
                Button::new(
                    &mut smallimgd.small_image_show_button,
                    Text::new("+ Small Image").color(Color::WHITE),
                )
                .padding(5)
                .style(DiscordDefaultButton)
                .on_press(Message::SmallImgShowUpdate),
            ),
        };
        let large_img_portion = match largeimgd.large_image_show {
            true => Column::new()
                .push(
                    Button::new(
                        &mut largeimgd.large_image_hide_button,
                        Text::new("- Large Image"),
                    )
                    .padding(5)
                    .on_press(Message::LargeImgHideUpdate),
                )
                .push(Space::with_height(iced::Length::Units(40)))
                .push(
                    Row::new()
                        .push(Text::new("Image Key").color(Color::WHITE).size(25))
                        .push(Space::with_width(iced::Length::Units(20)))
                        .push(
                            TextInput::new(
                                &mut largeimgd.large_image_key,
                                "Large Image Key.",
                                &acd_data_data
                                    .large_image_key
                                    .as_ref()
                                    .unwrap_or(&"".to_string()),
                                Message::RpcDataLargeImageUpdate,
                            )
                            .style(DiscordTextInput)
                            .padding(15),
                        )
                        .push(Space::with_height(iced::Length::Units(60)))
                        .push(Text::new("Image Text").color(Color::WHITE).size(25))
                        .push(Space::with_width(iced::Length::Units(20)))
                        .push(
                            TextInput::new(
                                &mut largeimgd.large_image_text,
                                "Large Image Text.",
                                &acd_data_data
                                    .large_image_text
                                    .as_ref()
                                    .unwrap_or(&"".to_string()),
                                Message::RpcDataLargeTextUpdate,
                            )
                            .style(DiscordTextInput)
                            .padding(15),
                        ),
                ),
            false => Column::new().push(
                Button::new(
                    &mut largeimgd.large_image_show_button,
                    Text::new("+ Large Image"),
                )
                .padding(5)
                .on_press(Message::LargeImgShowUpdate),
            ),
        };

        let buttona_text_val = &acd_data_data.button_a_text;
        let buttona_url_val = &acd_data_data.button_a_url;
        buttona.buttona_valid = false;
        let mut batext =
            Text::new("Invalid Button A. Both Text and url must exist.").color(Color::WHITE);
        if let Some(bav) = buttona_text_val {
            if let Some(bau) = buttona_url_val {
                if bau.len() > 1 && bav.len() > 1 {
                    batext = match url::Url::parse(bau) {
                        Ok(_u) => {
                            buttona.buttona_valid = true;
                            Text::new("Valid Button Entered").color(Color::from_rgb(0.0, 1.0, 0.0))
                        }
                        Err(_e) => Text::new("Valid Button Text, bad Button URL")
                            .color(Color::from_rgb(1.0, 0.0, 0.0)),
                    };
                } else {
                    batext = Text::new("InValid Button Entered. Please enter text inside")
                        .color(Color::from_rgb(1.0, 0.0, 0.0));
                }
            }
        };

        let buttonb_text_val = &acd_data_data.button_b_text;
        let buttonb_url_val = &acd_data_data.button_b_url;
        buttonb.buttonb_valid = false;
        let mut bbtext =
            Text::new("Invalid Button B. Both Text and url must exist.").color(Color::WHITE);
        if let Some(bbv) = buttonb_text_val {
            if let Some(bbu) = buttonb_url_val {
                if bbu.len() > 1 && bbv.len() > 1 {
                    bbtext = match url::Url::parse(bbu) {
                        Ok(_u) => {
                            buttonb.buttonb_valid = true;
                            Text::new("Valid Button B Entered")
                                .color(Color::from_rgb(0.0, 1.0, 0.0))
                        }
                        Err(_e) => Text::new("Valid Button B Text, bad Button B URL")
                            .color(Color::from_rgb(1.0, 0.0, 0.0)),
                    };
                } else {
                    bbtext = Text::new("InValid Button B Entered. Please enter text inside")
                        .color(Color::from_rgb(1.0, 0.0, 0.0));
                }
            }
        };

        let button_a_portion = match buttona.buttona_show {
            true => Column::new()
                .push(
                    Button::new(&mut buttona.buttona_hide_button, Text::new("- Button A"))
                        .padding(5)
                        .on_press(Message::ButtonAHideUpdate),
                )
                .push(Space::with_height(iced::Length::Units(40)))
                .push(
                    Row::new()
                        .push(Text::new("Button Text").color(Color::WHITE).size(25))
                        .push(Space::with_width(iced::Length::Units(20)))
                        .push(
                            TextInput::new(
                                &mut buttona.buttona_text,
                                "Button Text",
                                buttona_text_val.as_ref().unwrap_or(&"".to_string()),
                                Message::RpcDataLargeButtonATextUpdate,
                            )
                            .style(DiscordTextInput)
                            .padding(15),
                        )
                        .push(Space::with_height(iced::Length::Units(60)))
                        .push(Text::new("Image Text").color(Color::WHITE).size(25))
                        .push(Space::with_width(iced::Length::Units(20)))
                        .push(
                            TextInput::new(
                                &mut buttona.buttona_url,
                                "Button Url",
                                buttona_url_val.as_ref().unwrap_or(&"".to_string()),
                                Message::RpcDataLargeButtonAUrlUpdate,
                            )
                            .style(DiscordTextInput)
                            .padding(15),
                        ),
                )
                .push(Space::with_height(iced::Length::Units(20)))
                .push(batext),
            false => Column::new().push(
                Button::new(&mut buttona.buttona_show_button, Text::new("+ Button A"))
                    .padding(5)
                    .on_press(Message::ButtonAShowUpdate),
            ),
        };

        let button_b_portion = match buttonb.buttonb_show {
            true => Column::new()
                .push(
                    Button::new(&mut buttonb.buttonb_hide_button, Text::new("- Button B"))
                        .padding(5)
                        .on_press(Message::ButtonBHideUpdate),
                )
                .push(Space::with_height(iced::Length::Units(40)))
                .push(
                    Row::new()
                        .push(Text::new("Button Text").color(Color::WHITE).size(25))
                        .push(Space::with_width(iced::Length::Units(20)))
                        .push(
                            TextInput::new(
                                &mut buttonb.buttonb_text,
                                "Button Text",
                                buttonb_text_val.as_ref().unwrap_or(&"".to_string()),
                                Message::RpcDataLargeButtonBTextUpdate,
                            )
                            .style(DiscordTextInput)
                            .padding(15),
                        )
                        .push(Space::with_height(iced::Length::Units(60)))
                        .push(Text::new("Image Text").color(Color::WHITE).size(25))
                        .push(Space::with_width(iced::Length::Units(20)))
                        .push(
                            TextInput::new(
                                &mut buttonb.buttonb_url,
                                "Button Url",
                                buttonb_url_val.as_ref().unwrap_or(&"".to_string()),
                                Message::RpcDataLargeButtonBUrlUpdate,
                            )
                            .style(DiscordTextInput)
                            .padding(15),
                        ),
                )
                .push(Space::with_height(iced::Length::Units(20)))
                .push(bbtext),
            false => Column::new().push(
                Button::new(&mut buttonb.buttonb_show_button, Text::new("+ Button B"))
                    .padding(5)
                    .on_press(Message::ButtonBShowUpdate),
            ),
        };

        Scrollable::new(&mut self.scroll_state)
            .push(
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
                    .push(small_img_portion)
                    .push(Space::with_height(iced::Length::Units(40)))
                    .push(large_img_portion)
                    .push(Space::with_height(iced::Length::Units(40)))
                    .push(button_a_portion)
                    .push(Space::with_height(iced::Length::Units(40)))
                    .push(button_b_portion)
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
                    ),
            )
            .into()
    }
}
