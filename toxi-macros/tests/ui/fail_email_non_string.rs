#[derive(toxi_macros::Model)]
struct Bad {
    id: i64,
    #[validate(email)]
    email: i64,
}

fn main() {}
