use serde::Serialize;
use tracing::info;
use tracing_subscriber::fmt;
use valuable::Valuable;

#[derive(Debug, Serialize, Valuable)]
struct User {
    name: String,
    cars: Vec<Car>,
}

#[derive(Debug, Serialize, Valuable)]
struct Car {
    make: String,
    model: String,
    transmission: Transmission,
}

#[derive(Debug, Serialize, Valuable)]
enum Transmission {
    Automatic,
    Manual,
}

fn main() {
    fmt().json().with_target(false).init();

    let user = User {
        name: "Jose".to_string(),
        cars: vec![
            Car {
                make: "Toyota".to_string(),
                model: "Rav4".to_string(),
                transmission: Transmission::Manual,
            },
            Car {
                make: "Tesla".to_string(),
                model: "Cybertruck".to_string(),
                transmission: Transmission::Automatic,
            },
        ],
    };
    // Using the Valuable crate
    info!(user = user.as_value());
}
