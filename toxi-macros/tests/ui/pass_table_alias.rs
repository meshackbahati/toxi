extern crate toxi_db as toxi;

use toxi_db::Model;
use sqlx::FromRow;

#[derive(toxi_macros::Model, FromRow)]
#[model(table = "accounts")]
struct Account {
    id: i64,
    email: String,
}

fn main() {
    assert_eq!(Account::table_name(), "accounts");
}
