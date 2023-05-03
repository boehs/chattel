use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use backend::*;
use deadpool_diesel::sqlite::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations/");

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = establish_deadpool_connection();

    // run the migrations on server startup
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    // build our application with some routes
    let app = Router::new()
        .route("/item/create", post(create_item))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn sql<'a, F, T, E>(pool: &'a Pool, f: F) -> Result<T, (StatusCode, String)>
where
    F: FnOnce(&mut diesel::SqliteConnection) -> Result<T, E> + Send + 'static,
    E: std::error::Error + Send + 'static,
    T: Send + 'static,
{
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(f)
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(res)
}

async fn create_item(
    State(pool): State<Pool>,
    Json(new_item): Json<item::NewItem>,
) -> Result<Json<Option<item::ItemReturn>>, (StatusCode, String)> {
    sql(&pool, |f| item::create(new_item, f))
        .await
        .map(|x| Json(x))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    tracing::warn!("500: {}", err.to_string());
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
