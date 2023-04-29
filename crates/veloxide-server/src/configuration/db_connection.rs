use super::*;

#[instrument]
async fn get_database_environment_variable() -> String {
    tracing::event!(Level::INFO, "getting database environment variable");
    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::event!(Level::INFO, "database environment variable set.");
    db_url
}

cfg_if::cfg_if! {
    if #[cfg(feature = "postgres")] {
        use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
        #[instrument]
        pub async fn get_db_connection_postgres_sqlx() -> crate::prelude::Result<Pool<Postgres>> {
            let db_connection_url = get_database_environment_variable().await;

            tracing::event!(
                Level::INFO,
                "connecting to postgres db with connection string: {db_connection_url}"
            );
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(db_connection_url.as_str())
                .await?;

            Ok(pool)
        }
        pub async fn get_db_connection() -> crate::prelude::Result<Pool<Postgres>> {
            get_db_connection_postgres_sqlx().await
        }
    } else if #[cfg(feature = "mysql")] {
        use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

        #[instrument]
        pub async fn get_db_connection_mysql_sqlx() -> crate::prelude::Result<Pool<MySql>> {
            let db_connection_url = get_database_environment_variable().await;

            tracing::event!(
                Level::INFO,
                "connecting to mysql db with connection string: {db_connection_url}"
            );
            let pool = MySqlPoolOptions::new()
                .max_connections(5)
                .connect(db_connection_url.as_str())
                .await?;

            Ok(pool)
        }
        pub async fn get_db_connection() -> crate::prelude::Result<Pool<MySql>> {
            get_db_connection_mysql_sqlx().await
        }
    } else {
        compile_error!("Must specify either mysql or postgres feature");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[tokio::test]
    async fn test_get_database_environment_variable() {
        env::set_var("DATABASE_URL", "test");
        let db_connection_url = get_database_environment_variable().await;
        assert_eq!(db_connection_url, "test");
    }
}
