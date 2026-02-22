use crate::config::database::DatabaseConfig;
use futures::TryStreamExt;
use tiberius::{Client, Config};
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub async fn get_databases(
    config: &DatabaseConfig,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let conn_string = format!(
        "server=tcp:{},{};user id={};password={};",
        config.db_host, config.db_port, config.db_username, config.db_password
    );
    let config = Config::from_ado_string(&conn_string)?;
    let tcp = tokio::net::TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let tcp = tcp.compat_write();
    let mut client = Client::connect(config, tcp).await?;

    let mut stream = client.query("SELECT name FROM sys.databases", &[]).await?;
    let mut databases = Vec::new();

    while let Some(item) = stream.try_next().await? {
        if let Some(row) = item.into_row() {
            let name: &str = row
                .get("name")
                .ok_or("Failed to read database name from row")?;
            databases.push(name.to_string());
        }
    }

    Ok(databases)
}
