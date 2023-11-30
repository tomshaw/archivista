use crate::config::database::DatabaseConfig;
use mysql::prelude::*;
use mysql::*;

pub async fn get_databases(config: &DatabaseConfig) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let opts = config.mysql_opts();
    let pool = Pool::new(opts)?;
    let mut conn = pool.get_conn()?;

    let databases: Vec<String> = conn.query_map("SHOW DATABASES", |database: String| database)?;

    Ok(databases)
}
