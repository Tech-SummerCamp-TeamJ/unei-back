use actix_web::{
    get,
    web::{self, ServiceConfig},
    Responder,
};
use migration::MigratorTrait;
use sea_orm::SqlxPostgresConnector;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, world!"
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone());

    migration::Migrator::up(&db, None)
        .await
        .expect("Migrations failed");

    let state = web::Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(index).app_data(state);
    };

    Ok(config.into())
}
