use sqlx::mysql::MySqlPoolOptions;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://daocker:daocker123@wpan@sh-cynosdbmysql-grp-09nvk3xa.sql.tencentcdb.com:22763/daocker_db?characterEncoding=utf-8")
        .await?;

    let row: Vec<(u64,)> = sqlx::query_as("SELECT * from test")
        // .bind(150_i64)
        .fetch_all(&pool)
        .await?;

    println!("row is {:?}", row);

    // let wpan: Student = Student

    Ok(())
}
