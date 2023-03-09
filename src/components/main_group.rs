use std::sync::Arc;

use fltk::{
    button::Button,
    enums::Align,
    frame::Frame,
    group::{Flex, Group, Scroll},
    prelude::*,
};

use crate::types::{client_event::ClientEvent, music_status::MusicPlayStatus, state::SharedState};

pub fn create_main_group(state_: SharedState, window_width: i32, window_height: i32) -> Group {
    let _event_sender = state_.lock().unwrap().event_sender.clone();

    let group_top_margin = 30;

    let main_group = Group::new(0, group_top_margin, window_width, window_height, "Main");

    let mut global_flex = Flex::new(0, group_top_margin, window_width, window_height, None);

    global_flex.set_margin(15);
    global_flex.set_pad(15);

    // current music title
    {
        let mut flex = Flex::default().column();

        let mut frame = Frame::default()
            .with_label("none")
            .with_align(Align::Center);

        global_flex.set_size(&mut flex, 20);

        let state = Arc::clone(&state_);
        tokio::spawn(async move {
            loop {
                println!("title backgroud loop");
                {
                    match state.try_lock() {
                        Ok(state) => {
                            if let Some(file) = state.get_current_file() {
                                frame.set_label(file.filename.as_str());
                            }
                        }
                        Err(error) => {
                            println!("{:?}", error);
                        }
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
        });

        flex.end();
    }

    // 제어 버튼 열
    {
        let mut flex = Flex::default().row();
        flex.set_align(Align::Center);

        let buttons_width = 40;
        let buttons_height = 40;

        // left space
        Flex::default().end();

        let mut left_button = Button::default().with_label("@<-");

        let mut stop_button = Button::default().with_label("@||");

        let mut right_button = Button::default().with_label("@->");

        // right space
        Flex::default().end();

        global_flex.set_size(&mut flex, buttons_height);
        flex.set_size(&mut left_button, buttons_width);
        flex.set_size(&mut stop_button, buttons_width);
        flex.set_size(&mut right_button, buttons_width);

        let _state = Arc::clone(&state_);
        left_button.set_callback(move |_| {});

        let state = Arc::clone(&state_);
        let event_sender = _event_sender.clone();

        stop_button.set_callback(move |_| match state.try_lock().unwrap().status {
            MusicPlayStatus::Stopped => {
                if let Err(error) = event_sender.send(ClientEvent::Start) {
                    println!("{:?}", error);
                }
            }
            MusicPlayStatus::Playing => {
                if let Err(error) = event_sender.send(ClientEvent::Stop) {
                    println!("{:?}", error);
                }
            }
            MusicPlayStatus::Paused => {
                if let Err(error) = event_sender.send(ClientEvent::Resume) {
                    println!("{:?}", error);
                }
            }
        });

        let event_sender = _event_sender.clone();
        left_button.set_callback(move |_| {
            if let Err(error) = event_sender.send(ClientEvent::Left) {
                println!("{:?}", error);
            }
        });

        let event_sender = _event_sender.clone();
        right_button.set_callback(move |_| {
            if let Err(error) = event_sender.send(ClientEvent::Right) {
                println!("{:?}", error);
            }
        });

        flex.end();
    }

    {
        let flex = Flex::default().column();
        let scoll = Scroll::default();

        let button = Button::default().with_label("test");

        //

        //

        scoll.end();
        flex.end();
    }

    // empty flex
    {
        let flex = Flex::default().row();

        flex.end();
    }

    global_flex.end();

    main_group.end();
    main_group
}
