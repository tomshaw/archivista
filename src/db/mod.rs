use crate::config::database::DatabaseConfig;
pub mod mysql;
pub mod postgres;
pub mod sqlserver;

pub async fn get_databases(config: &DatabaseConfig) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    match &config.db_connection[..] {
        "mysql" => mysql::get_databases(config).await,
        "postgres" => postgres::get_databases(config).await,
        "sqlserver" => sqlserver::get_databases(config).await,
        _ => Err("Invalid connection type".into()),
    }
}
