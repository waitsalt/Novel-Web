use crate::settings::SETTINGS;
use sqlx::postgres::PgPool;
use tokio::sync::OnceCell;

static CONNECTION: OnceCell<PgPool> = OnceCell::const_new();

pub async fn connection() -> &'static PgPool {
    CONNECTION
        .get_or_init(|| async {
            let uri = SETTINGS.database.uri.as_str();
            PgPool::connect(uri)
                .await
                .expect("Failed to connect to PostgreSQL\n")
        })
        .await
}
