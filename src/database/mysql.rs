use crate::config::Mysql;
use mysql::{Pool, PooledConn};
use once_cell::sync::OnceCell;
use tracing::instrument;

static DB_POOL: OnceCell<Pool> = OnceCell::new();

#[instrument]
pub async fn init_mysql(mysql: &Mysql) -> anyhow::Result<()> {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        mysql.user, mysql.password, mysql.host, mysql.port, mysql.database
    );
    DB_POOL
        .set(mysql::Pool::new(url.clone().as_str()).expect("mysql连接失败"))
        .unwrap_or_else(|_| tracing::info!("初始化mysql连接池失败"));
    tracing::info!("初始化mysql连接池成功");
    Ok(())
}

#[instrument]
pub async fn get_conn() -> PooledConn {
    DB_POOL
        .get()
        .expect("无法从连接池中获取连接")
        .get_conn()
        .expect("链接失败")
}
