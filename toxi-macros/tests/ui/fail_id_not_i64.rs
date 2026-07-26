use sqlx::FromRow;

#[derive(toxi_macros::Model, FromRow)]
struct User {
    id: i32,
    email: String,
}

fn main() {}
