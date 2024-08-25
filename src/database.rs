use crate::settings::SETTINGS;
use sqlx::postgres::PgPool;
use tokio::sync::OnceCell;

static CONNECTION: OnceCell<PgPool> = OnceCell::const_new();

pub async fn connection() -> &'static PgPool {
    CONNECTION
        .get_or_init(|| async {
            let url = SETTINGS.database.uri.as_str();
            PgPool::connect(url)
                .await
                .expect("Failed to connect to PostgreSQL\n")
        })
        .await
}
