extern crate toxi_db as toxi;

use toxi_db::Model;
use sqlx::FromRow;

#[derive(toxi_macros::Model, FromRow)]
#[model(table_name = "people")]
struct Person {
    id: i64,
    email: String,
}

fn main() {
    assert_eq!(Person::table_name(), "people");
}
