use crate::config::database::DatabaseConfig;
use futures::TryStreamExt;
use tiberius::{Config, Client};
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub async fn get_databases(config: &DatabaseConfig) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let conn_string = format!(
        "server=tcp:{},{};user id={};password={};",
        config.db_host, config.db_port, config.db_username, config.db_password
    );
    let config = Config::from_ado_string(&conn_string)?;
    let tcp = tokio::net::TcpStream::connect(config.get_addr()).await?;
    let tcp = tcp.compat_write();
    let mut client = Client::connect(config, tcp).await?;

    let mut stream = client.query("SELECT name FROM sys.databases", &[]).await?;
    let mut databases = Vec::new();

    while let Some(row) = stream.try_next().await? {
        let name: &str = row.get("name").unwrap();
        databases.push(name.to_string());
    }

    Ok(databases)
}
