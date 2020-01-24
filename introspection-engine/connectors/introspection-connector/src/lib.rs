mod error;

use datamodel::Datamodel;
pub use error::{ConnectorError, ErrorKind};
use serde::*;
use sql_schema_describer::SqlSchema;

pub type ConnectorResult<T> = Result<T, ConnectorError>;

#[async_trait::async_trait]
pub trait IntrospectionConnector: Send + Sync + 'static {
    async fn list_databases(&self) -> ConnectorResult<Vec<String>>;

    async fn get_metadata(&self) -> ConnectorResult<DatabaseMetadata>;

    async fn get_sql_schema(&self) -> ConnectorResult<SqlSchema>;

    async fn introspect(&self) -> ConnectorResult<Datamodel>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseMetadata {
    pub table_count: usize,
    pub size_in_bytes: usize,
}
