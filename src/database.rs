use crate::config::DatabaseConfig;
use anyhow::Result;
use tiberius::{Client, Config as TiberiusConfig, Query};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub struct Database {
    config: DatabaseConfig,
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub row_count: usize,
}

impl Database {
    pub fn new(config: DatabaseConfig) -> Self {
        Self { config }
    }

    async fn connect(&self) -> Result<Client<tokio_util::compat::Compat<TcpStream>>> {
        let mut tiberius_config = TiberiusConfig::new();

        // Parse server name - extract hostname before backslash
        let hostname = self
            .config
            .server
            .split('\\')
            .next()
            .unwrap_or(&self.config.server);

        // Connect directly to hostname:port without SQL Browser
        let addr = format!("{}:{}", hostname, self.config.port);

        tiberius_config.host(hostname);
        tiberius_config.database(&self.config.database);
        tiberius_config.trust_cert();

        if !self.config.user.is_empty() {
            tiberius_config.authentication(tiberius::AuthMethod::sql_server(
                &self.config.user,
                &self.config.password,
            ));
        }
        // Windows authentication is default when no auth method is set

        let tcp = TcpStream::connect(&addr).await?;
        tcp.set_nodelay(true)?;
        let client = Client::connect(tiberius_config, tcp.compat_write()).await?;
        Ok(client)
    }

    pub async fn execute_query(&self, query: &str) -> Result<QueryResult> {
        let mut client = self.connect().await?;
        let mut stream = Query::new(query).query(&mut client).await?;

        let mut columns = Vec::new();
        let mut rows = Vec::new();

        if let Some(cols) = stream.columns().await? {
            columns = cols.iter().map(|c| c.name().to_string()).collect();
        }

        let row_stream = stream.into_first_result().await?;
        for row in row_stream {
            let mut row_data = Vec::new();
            for col_idx in 0..columns.len() {
                let value = match row.try_get::<&str, _>(col_idx) {
                    Ok(Some(s)) => s.to_string(),
                    Ok(None) => "NULL".to_string(),
                    Err(_) => {
                        // Try other types if string fails
                        if let Ok(Some(i)) = row.try_get::<i32, _>(col_idx) {
                            i.to_string()
                        } else if let Ok(Some(i)) = row.try_get::<i64, _>(col_idx) {
                            i.to_string()
                        } else if let Ok(Some(f)) = row.try_get::<f64, _>(col_idx) {
                            f.to_string()
                        } else if let Ok(Some(b)) = row.try_get::<bool, _>(col_idx) {
                            b.to_string()
                        } else {
                            format!("{:?}", row.get::<&[u8], _>(col_idx).unwrap_or(&[]))
                        }
                    }
                };
                row_data.push(value);
            }
            rows.push(row_data);
        }

        let row_count = rows.len();
        Ok(QueryResult {
            columns,
            rows,
            row_count,
        })
    }
}
