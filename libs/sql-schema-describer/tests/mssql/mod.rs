use tracing::debug;

use quaint::prelude::*;
use sql_schema_describer::*;
use std::sync::Arc;
use test_setup::mssql_2019_url;

pub async fn get_mssql_describer_for_schema(sql: &str, schema: &str) -> mssql::SqlSchemaDescriber {
    let url = mssql_2019_url(schema);
    let conn = test_setup::create_mssql_database(&url).await.unwrap();

    debug!("Executing SQL Server migrations: {}", sql);

    let statements = sql.split(";").filter(|s| !s.is_empty());

    for statement in statements {
        debug!("Executing migration statement: '{}'", statement);

        conn.query_raw(&statement, &[])
            .await
            .expect("executing migration statement");
    }

    mssql::SqlSchemaDescriber::new(Arc::new(conn))
}
