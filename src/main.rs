use serde::Serialize;
use tracing::{Level, event};
use valuable::Valuable;

#[derive(Serialize, Valuable)]
struct Tag {
    name: &'static str,
    value: &'static str,
}

#[derive(Serialize, Valuable)]
struct Job {
    company: &'static str,
    salary: u32,
}

#[derive(Serialize, Valuable)]
struct User {
    name: &'static str,
    age: u8,
    job: Job,
    tags: Vec<Tag>,
}

fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_line_number(true)
        .with_file(true)
        .with_target(true)
        .init();

    let user = User {
        name: "Jose Valerio",
        age: 28,
        job: Job {
            company: "TechCorp",
            salary: 85000,
        },
        tags: vec![
            Tag {
                name: "fav_sport",
                value: "baseball",
            },
            Tag {
                name: "fav_color",
                value: "blue",
            },
            Tag {
                name: "hobby",
                value: "photography",
            },
        ],
    };

    event!(Level::INFO, user = user.as_value(), "User profile logged");
}
