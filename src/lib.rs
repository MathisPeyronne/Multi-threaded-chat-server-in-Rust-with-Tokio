use sea_orm::Database;

pub async fn run_database(database_uri: &str) {
    let database = Database::connect(database_uri).await;
}
