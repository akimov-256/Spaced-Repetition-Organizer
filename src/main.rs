// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod database_manager;

use std::error::Error;
use rfd::{MessageDialog, MessageDialogResult};
use slint::{ModelRc, ToSharedString, VecModel};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    database_manager::initialize_database()?;

    let ui_clone = ui.clone_strong();
    load_topics(&ui_clone)?;

    let ui_clone = ui.clone_strong();
    let ui_clone_2 = ui.clone_strong();
    ui_clone.on_create_topic_requested(move |name| {
        match database_manager::create_topic(&name) {
            Ok(_) => {
                match load_topics(&ui_clone_2) {
                    Ok(_) => {

                    }
                    Err(error) => {
                        MessageDialog::new()
                        .set_title("Error")
                        .set_description(format!("Error loading topics: {error}"))
                        .show();
                    }
                }
            }
            Err(error) => {
                MessageDialog::new()
                .set_title("Error")
                .set_description(format!("Error creating topic {name}: {error}"))
                .show();
            }
        }
    });

    let ui_clone = ui.clone_strong();
    let ui_clone_2 = ui.clone_strong();
    ui_clone.on_delete_topic(move |name| {
        match MessageDialog::new()
            .set_title("Delete Lesson")
            .set_description(format!("You will delete topic {name}, are you sure?"))
            .set_buttons(rfd::MessageButtons::OkCancel)
            .show() {
            MessageDialogResult::Ok => {
                match database_manager::delete_topic(name.to_string()) {
                    Ok(_) => {
                        match load_topics(&ui_clone_2) {
                            Ok(_) => {
                            
                            }
                            Err(error) => {
                                MessageDialog::new()
                                .set_title("Error")
                                .set_description(format!("Error loading topics: {error}"))
                                .show();
                            }
                        }
                    }
                    Err(error) => {
                        MessageDialog::new()
                        .set_title("Error")
                        .set_description(format!("Error deleting topic {name}: {error}"))
                        .show();
                    }
                }
            }
            MessageDialogResult::Cancel => {

            }
            _ => {}
        }
    });

    let ui_clone = ui.clone_strong();
    let ui_clone_2 = ui.clone_strong();
    ui_clone.on_rename_topic_requested(move |topic, new| {
        match database_manager::rename_topic(topic.to_string(), new.to_string()) {
            Ok(_) => {
                match load_topics(&ui_clone_2) {
                    Ok(_) => {

                    }
                    Err(error) => {
                        MessageDialog::new()
                        .set_title("Error")
                        .set_description(format!("Error loading topics: {error}"))
                        .show();
                    }
                }
            }
            Err(error) => {
                MessageDialog::new()
                .set_title("Error")
                .set_description(format!("Error renaming topic {topic} to {new}: {error}"))
                .show();
            }           
        }
    });

    let ui_clone = ui.clone_strong();
    let ui_clone_2 = ui.clone_strong();
    ui_clone.on_add_lesson(move |topic, lesson| {
        match database_manager::add_lesson(topic.to_string(), lesson.to_string()) {
            Ok(_) => {
                match load_lessons(&ui_clone_2, topic.to_string()) {
                    Ok(_) => {

                    }
                    Err(error) => {
                        MessageDialog::new()
                        .set_title("Error")
                        .set_description(format!("Error loading lessons for topic {topic}: {error}"))
                        .show();
                    }
                }
            }
            Err(error) => {
                MessageDialog::new()
                .set_title("Error")
                .set_description(format!("Error adding lesson {lesson}: {error}"))
                .show();
            }
        }
    });

    let ui_clone = ui.clone_strong();
    let ui_clone_2 = ui.clone_strong();
    ui_clone.on_load_lessons_requested(move |topic| {
        match load_lessons(&ui_clone_2, topic.to_string()) {
            Ok(_) => {

            }
            Err(error) => {
                MessageDialog::new()
                .set_title("Error")
                .set_description(format!("Error loading lessons for topic {topic}: {error}"))
                .show();
            }
        }
    });
    
    let ui_clone = ui.clone_strong();
    let ui_clone_2 = ui.clone_strong();
    ui_clone.on_delete_lesson_requested(move |topic, lesson| {
        match MessageDialog::new()
            .set_title("Delete Lesson")
            .set_description(format!("You will delete lesson {lesson}, are you sure?"))
            .set_buttons(rfd::MessageButtons::OkCancel)
            .show() {
            MessageDialogResult::Ok => {
                match database_manager::delete_lesson(topic.to_string(), lesson.to_string()) {
                    Ok(_) => {
                        match load_lessons(&ui_clone_2, topic.to_string()) {
                            Ok(_) => {
                                
                            }
                            Err(error) => {
                                MessageDialog::new()
                                .set_title("Error")
                                .set_description(format!("Error loading lessons from topic {topic}: {error}"))
                                .show();
                            }
                        }
                    }
                    Err(error) => {
                        MessageDialog::new()
                        .set_title("Error")
                        .set_description(format!("Error deleting lesson {lesson} from {topic}: {error}"))
                        .show();
                    }
                }
            }
            MessageDialogResult::Cancel => {

            }
            _ => {}
        }
    });

    let ui_clone = ui.clone_strong();
    let ui_clone_2 = ui.clone_strong();
    ui_clone.on_review_lesson_requested(move |topic, lesson, stage: i32| {
        match database_manager::review_lesson(topic.to_string(), lesson.to_string(), stage) {
            Ok(_) => {
                match load_lessons(&ui_clone_2, topic.to_string()) {
                    Ok(_) => {

                    }
                    Err(error) => {
                        MessageDialog::new()
                        .set_title("Error")
                        .set_description(format!("Error loading lessons from topic {topic}: {error}"))
                        .show();
                    }
                }
            }
            Err(error) => {
                MessageDialog::new()
                .set_title("Error")
                .set_description(format!("Error reviewing lesson {lesson} from {topic}: {error}"))
                .show();
            }           
        }
    });

    let ui_clone = ui.clone_strong();
    let ui_clone_2 = ui.clone_strong();
    ui_clone.on_rename_lesson_requested(move |topic, lesson, new| {
        match database_manager::rename_lesson(topic.to_string(), lesson.to_string(), new.to_string()) {
            Ok(_) => {
                match load_lessons(&ui_clone_2, topic.to_string()) {
                    Ok(_) => {

                    }
                    Err(error) => {
                        MessageDialog::new()
                        .set_title("Error")
                        .set_description(format!("Error loading lessons from topic {topic}: {error}"))
                        .show();
                    }
                }
            }
            Err(error) => {
                MessageDialog::new()
                .set_title("Error")
                .set_description(format!("Error renaming lesson {lesson} to {new} from {topic}: {error}"))
                .show();
            }           
        }
    });

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

fn load_lessons(app: &AppWindow, topic: String) -> Result<(), Box<dyn Error>> {
    let lessons = database_manager::load_lessons(topic)?;

    let lesson_data: Vec<LessonData> = lessons
    .into_iter()
    .map(|row| LessonData {
        name: row.name.to_shared_string(),

        stage: row.stage,
        previous_review: row.previous_review.to_shared_string(),
        next_review: row.next_review.to_shared_string(),
        due: row.due as i32
    })
    .collect();

    let model = ModelRc::new(VecModel::from(lesson_data));

    load_topics(app)?;

    app.set_lessons(model);

    Ok(())
}