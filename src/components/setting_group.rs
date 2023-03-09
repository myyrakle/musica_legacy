use fltk::{
    button::Button,
    dialog::{FileDialog, FileDialogType},
    group::{Flex, Group},
    prelude::*,
};

use crate::types::state::SharedState;

pub fn create_setting_group(state: SharedState, window_width: i32, window_height: i32) -> Group {
    let group_top_margin = 30;

    let setting_group = Group::new(0, group_top_margin, window_width, window_height, "Setting");

    let mut global_flex = Flex::new(0, group_top_margin, window_width, window_height, None);

    global_flex.set_margin(15);

    {
        let mut flex = Flex::default().row();

        let browse_button_width = 140;
        let browse_button_height = 40;
        let mut browse_button = Button::default();
        browse_button.set_label("@fileopen Choose Folder");

        global_flex.set_size(&mut flex, browse_button_height);
        flex.set_size(&mut browse_button, browse_button_width);

        flex.end();

        browse_button.set_callback(move |_| {
            let mut file_dialog = FileDialog::new(FileDialogType::BrowseDir);
            file_dialog.show();
            let path = file_dialog.filename();

            if let Ok(mut state) = state.lock() {
                println!("안되나?");
                state.set_directory_path(path);
                state.write_to_config_file();
                state.read_music_list();
            } else {
                println!("????");
            }
        });
    }

    // empty flex
    {
        let flex = Flex::default().row();

        flex.end();
    }

    global_flex.end();

    setting_group.end();
    setting_group
}
