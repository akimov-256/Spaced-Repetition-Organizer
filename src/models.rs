pub struct Topic {
    pub title: String,
    pub lessons: i32,
    pub due: i32,
}

pub struct Lesson {
    pub name: String,

    pub stage: i32,
    pub previous_review: String,
    pub next_review: String,
    pub due: i64
}