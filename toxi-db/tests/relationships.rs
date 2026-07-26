extern crate toxi_db as toxi;

use toxi_db::{Model, sqlx, Database, DatabaseType, DbTransaction, Result, HasMany, HasOne, BelongsTo, DbInspector, TableSchema};
use async_trait::async_trait;
use sqlx::any::AnyRow;

mod user_mod {
    use toxi_db::Model;
    #[derive(Model, sqlx::FromRow, Clone)]
    pub struct User {
        pub id: i64,
        pub username: String,
    }
}

mod post_mod {
    use toxi_db::Model;
    #[derive(Model, sqlx::FromRow, Clone)]
    pub struct Post {
        pub id: i64,
        pub user_id: i64,
        pub title: String,
    }
}

use user_mod::User;
use post_mod::Post;

#[derive(Debug)]
struct MockDb;

#[async_trait]
impl Database for MockDb {
    fn db_type(&self) -> DatabaseType { DatabaseType::Sqlite }
    async fn execute(&self, _query: &str) -> Result<u64> { Ok(1) }
    async fn query(&self, _query: &str) -> Result<Vec<AnyRow>> { Ok(vec![]) }
    async fn query_one(&self, _query: &str) -> Result<Option<AnyRow>> { Ok(None) }
    async fn ping(&self) -> Result<()> { Ok(()) }
    async fn begin_transaction(&self) -> Result<DbTransaction> { unimplemented!() }
    async fn execute_query<'q>(&self, _query: sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>>) -> Result<u64> { Ok(1) }
    async fn fetch_all<'q>(&self, _query: sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>>) -> Result<Vec<AnyRow>> { Ok(vec![]) }
    async fn fetch_one<'q>(&self, _query: sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>>) -> Result<Option<AnyRow>> { Ok(None) }
    fn inspector(&self) -> Box<dyn DbInspector> {
        struct MockInspector;
        #[async_trait]
        impl DbInspector for MockInspector {
            async fn get_table_schema(&self, _table_name: &str) -> Result<Option<TableSchema>> { Ok(None) }
            async fn list_tables(&self) -> Result<Vec<String>> { Ok(vec![]) }
        }
        Box::new(MockInspector)
    }
}

impl User {
    fn posts(&self) -> HasMany<User, Post> {
        HasMany::new(self.id, "user_id")
    }

    fn profile(&self) -> HasOne<User, Post> {
        HasOne::new(self.id, "user_id")
    }
}

impl Post {
    fn user(&self) -> BelongsTo<Post, User> {
        BelongsTo::new(self.user_id)
    }
}

#[tokio::test]
async fn test_relationships_compilation() {
    let db = MockDb;
    let user = User { id: 1, username: "test".to_string() };
    let post = Post { id: 1, user_id: 1, title: "test".to_string() };

    let _posts = user.posts().get(&db).await;
    let _profile = user.profile().get(&db).await;
    let _user = post.user().get(&db).await;
}

#[tokio::test]
async fn eager_load_rejects_invalid_foreign_key() {
    let db = MockDb;

    let has_many_result = HasMany::<User, Post>::eager_load(&db, &[1, 2], "user_id;DROP").await;
    assert!(has_many_result.is_err());

    let has_one_result = HasOne::<User, Post>::eager_load(&db, &[1, 2], "user_id;DROP").await;
    assert!(has_one_result.is_err());
}
