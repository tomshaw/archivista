use crate::config::database::DatabaseConfig;
use tokio_postgres::NoTls;

pub async fn get_databases(config: &DatabaseConfig) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let conn_string = format!(
        "host={} port={} user={} password={}",
        config.db_host, config.db_port, config.db_username, config.db_password
    );

    let (client, connection) = tokio_postgres::connect(&conn_string, NoTls).await.map_err::<Box<dyn std::error::Error>, _>(|e| Box::new(e))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client.query("SELECT datname FROM pg_database", &[]).await.map_err(|e| Box::new(e))?;
    let databases: Vec<String> = rows.iter().map(|row| row.get(0)).collect();

    Ok(databases)
}
