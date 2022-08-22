use sea_query::{ColumnDef, Expr, Iden, Query, SqliteQueryBuilder, Table};
use sqlx::query_as;

use super::sea_query_driver_sqlite::bind_query_as;

#[derive(Debug, sqlx::FromRow)]
pub struct Model {
    value: String,
}

pub struct Entity;

#[derive(Debug, Iden)]
pub enum Configs {
    Table,
    Key,
    Value,
}

impl Entity {
    pub async fn create_table(pool: &sqlx::SqlitePool) -> sqlx::Result<()> {
        let sql = Table::create()
            .table(Configs::Table)
            .if_not_exists()
            .col(ColumnDef::new(Configs::Key).text().primary_key())
            .col(ColumnDef::new(Configs::Value).text().not_null())
            .build(SqliteQueryBuilder);

        let query_result = sqlx::query(&sql).execute(pool).await?;

        log::info!("Create table `configs` :{:?}", query_result);

        Ok(())
    }

    pub async fn load_config<K, F, E, T>(
        pool: &sqlx::SqlitePool,
        key: &K,
        mapper: F,
    ) -> sqlx::Result<Option<Result<T, E>>>
    where
        F: FnOnce(String) -> Result<T, E>,
        K: AsRef<str> + ?Sized,
    {
        let (sql, value) = Query::select()
            .column(Configs::Value)
            .from(Configs::Table)
            .cond_where(Expr::col(Configs::Key).eq(key.as_ref()))
            .build(SqliteQueryBuilder);
        let result = bind_query_as(query_as::<_, Model>(&sql), &value)
            .fetch_optional(pool)
            .await?;

        Ok(result.map(|v| mapper(v.value)))
    }
}
