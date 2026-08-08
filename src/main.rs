// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod database_manager;

use std::error::Error;
use slint::{ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    database_manager::initialize_database();

    let ui_clone = ui.clone_strong();
    ui.on_create_topic_requested(move |name| {
                                    match database_manager::create_topic(&name) {
                                        Ok(_) => {
                                            load_topics(&ui_clone);
                                        }
                                        Err(error) => {
                                            println!("error: {error}");
                                        }
                                    }
                                });

    load_topics(&ui)?;

    ui.run()?;

    Ok(())
}

fn load_topics(app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let topics= database_manager::load_topics()?;

    let topic_data: Vec<TopicData> = topics
    .into_iter()
    .map(|topic| TopicData {
        title: topic.title.into(),
        lessons: topic.lessons,
        due: topic.due,
    })
    .collect();

    let model = ModelRc::new(VecModel::from(topic_data));

    app.set_topics(model);

    Ok(())
}
