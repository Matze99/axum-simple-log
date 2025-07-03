use log;

#[axum_log_util::log_request]
pub async fn example() {
    println!("example");
}

fn main() {
    println!("Hello, world!");
}
