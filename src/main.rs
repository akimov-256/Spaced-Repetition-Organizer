// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database_manager;

use std::error::Error;
use slint::{ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    database_manager::initialize_database();

    ui.on_create_topic_requested(|name| {database_manager::create_topic(&name);});

    load_topics(&ui);

    ui.run()?;

    Ok(())
}

fn load_topics(app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let topics = 
                    VecModel::from(vec![TopicData {
                        title: "arabic".into(),
                        lessons: 7,
                        due: 3,
                    },
                    TopicData {
                        title: "math".into(),
                        lessons: 12,
                        due: 5,
                    }]);

    app.set_topics(ModelRc::new(topics));

    Ok(())
}
