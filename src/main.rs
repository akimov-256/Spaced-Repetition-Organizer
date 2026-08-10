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

    database_manager::initialize_database()?;

    let ui_clone = ui.clone_strong();
    ui.on_create_topic_requested(move |name| {
        match database_manager::create_topic(&name) {
            Ok(_) => {
                match load_topics(&ui_clone) {
                    Ok(_) => {

                    }
                    Err(error) => {
                        print!("Error loading topics: {error}");
                    }
                }
            }
            Err(error) => {
                println!("Error creating topic {name}: {error}");
            }
        }
    });

    let ui_clone = ui.clone_strong();
    ui.on_delete_topic(move |name| {
        match database_manager::delete_topic(name.to_string()) {
            Ok(_) => {
                match load_topics(&ui_clone) {
                    Ok(_) => {

                    }
                    Err(error) => {
                        print!("Error loading topics: {error}");
                    }
                }
            }
            Err(error) => {
                println!("Error deleting topic {name}: {error}");
            }
        }
    });

    let ui_clone = ui.clone_strong();
    ui_clone.on_add_lesson(move |topic, lesson| {
        match database_manager::add_lesson(topic.to_string(), lesson.to_string()) {
            Ok(_) => {
                database_manager::load_lessons(topic.to_string());
            }
            Err(error) => {
                println!("Error adding lesson {lesson}: {error}");
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
