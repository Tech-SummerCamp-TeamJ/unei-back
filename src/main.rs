mod entity;
mod event_scheme;
mod group_scheme;

use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    get,
    middleware::{self, Logger},
    post,
    web::{self, ServiceConfig},
    HttpMessage, Responder,
};
use actix_web::{
    error,
    http::{header, StatusCode},
    web::Query,
    HttpRequest, HttpResponse,
};
use chrono::NaiveDate;
use entity::{
    event,
    group::{self, Entity},
    member,
    prelude::*,
    session, user,
};
use migration::MigratorTrait;
use oauth2::{basic::BasicClient, reqwest::async_http_client};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::Client;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    SqlxPostgresConnector,
};
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct AuthRequest {
    code: String,
    state: String,
}

#[get("/")]
async fn index(identity: Option<Identity>) -> actix_web::Result<impl Responder> {
    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    Ok(format!("Hello {id}"))
}

#[get("/login")]
async fn login(app_state: web::Data<AppState>) -> impl Responder {
    // Discord OAuth用のクライアント設定
    let client = BasicClient::new(
        ClientId::new(app_state.client_id.clone()),
        Some(ClientSecret::new(app_state.client_secret.clone())),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(app_state.redirect_url.clone()).unwrap());

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    // CSRFトークンをセッションに保存することが推奨されます
    HttpResponse::Found()
        .append_header((header::LOCATION, auth_url.to_string()))
        .finish()
}

#[get("/auth/callback")]
async fn callback(
    app_state: web::Data<AppState>,
    query: Query<AuthRequest>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let client = BasicClient::new(
        ClientId::new(app_state.client_id.clone()),
        Some(ClientSecret::new(app_state.client_secret.clone())),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(app_state.redirect_url.clone()).unwrap());

    // 認証コードを取得
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .map_err(|err| {
            log::info!("Failed to get token: {:?}", err);
            error::ErrorInternalServerError(err)
        })?;

    // トークンからDiscordユーザー情報を取得
    let user_info = get_discord_user_info(token_result.access_token().secret()).await?;

    let user_id = user::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(user_info.username.clone()),
        email: Set(user_info.email.clone().unwrap_or("".to_string())),
        icon_path: Set(format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png",
            user_info.id, user_info.avatar
        )),
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| {
        log::info!("Failed to insert user: {:?}", err);
        error::ErrorInternalServerError(err)
    })?
    .id;

    // セッションにユーザーIDを保存
    let identity = Identity::login(&req.extensions(), user_info.id)?;

    session::ActiveModel {
        id: Set(Uuid::now_v7()),
        session_id: Set(identity.id()?.to_string()),
        user_id: Set(user_id),
    };

    Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
}

#[get("/logout")]
async fn logout(id: Identity) -> impl Responder {
    id.logout();
    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}

async fn get_discord_user_info(token: &str) -> actix_web::Result<DiscordUser> {
    let client = Client::new();
    let res = client
        .get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    let user_info = res
        .json::<DiscordUser>()
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;
    Ok(user_info)
}

#[post("/groups")]
async fn groups(
    identity: Option<Identity>,
    app_state: web::Data<AppState>,
    group: web::Json<group_scheme::Group>,
) -> actix_web::Result<impl Responder> {
    log::info!("Identity: {:?}", identity.is_some());
    let session_id = match identity {
        Some(id) => id.id()?,
        None => return Err(error::ErrorUnauthorized("Unauthorized")),
    };
    let groups = Group::find().all(&app_state.db).await.map_err(|err| {
        error::ErrorInternalServerError(format!("Failed to get groups: {:?}", err))
    })?;
    log::info!("FirstGroups: {}", groups.len());

    let user_id = Session::find()
        .filter(session::Column::SessionId.eq(session_id))
        .one(&app_state.db)
        .await
        .map_err(|err| {
            error::ErrorInternalServerError(format!("Failed to get session: {:?}", err))
        })?
        .map(|session| session.user_id)
        .ok_or_else(|| error::ErrorUnauthorized("Unauthorized"))?;

    let group = group.into_inner();
    let group_id = group::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(group.name),
        icon_path: Set(group.icon_path),
        theme: Set(group.theme),
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| error::ErrorInternalServerError(format!("Failed to insert group: {:?}", err)))?
    .id;

    member::ActiveModel {
        id: Set(Uuid::now_v7()),
        user_id: Set(user_id),
        is_admin: Set(true),
        group_id: Set(group_id),
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| {
        error::ErrorInternalServerError(format!("Failed to insert member: {:?}", err))
    })?;

    let groups = Group::find().all(&app_state.db).await.map_err(|err| {
        error::ErrorInternalServerError(format!("Failed to get groups: {:?}", err))
    })?;
    log::info!("Groups: {}", groups.len());

    Ok(HttpResponse::Created().json(group_id))
}

#[post("/groups/{group_id}/events")]
async fn events(
    identity: Option<Identity>,
    app_state: web::Data<AppState>,
    event: web::Json<event_scheme::Event>,
) -> actix_web::Result<impl Responder> {
    let event = event.into_inner();
    let session_id = match identity {
        Some(id) => id.id()?,
        None => return Err(error::ErrorUnauthorized("Unauthorized")),
    };
    let user_id = Session::find()
        .filter(session::Column::SessionId.eq(session_id))
        .one(&app_state.db)
        .await
        .map_err(|err| {
            error::ErrorInternalServerError(format!("Failed to get session: {:?}", err))
        })?
        .map(|session| session.user_id)
        .ok_or_else(|| error::ErrorUnauthorized("Unauthorized"))?;

    event::ActiveModel {
        id: Set(Uuid::now_v7()),
        description: Set(event.description),
        event_date: Set(NaiveDate::parse_from_str(&event.event_date, "%Y-%m-%d").unwrap()),
        author_id: Set(Some(user_id)),
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| error::ErrorInternalServerError(format!("Failed to insert event: {:?}", err)))?;

    Ok(HttpResponse::Created().finish())
}
#[get("/groups/{group_id}/events")]
async fn get_events(
    identity: Option<Identity>,
    app_state: web::Data<AppState>,
    group_id: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let session_id = match identity {
        Some(id) => id.id()?,
        None => return Err(error::ErrorUnauthorized("Unauthorized")),
    };
    let user_id = Session::find()
        .filter(session::Column::SessionId.eq(session_id))
        .one(&app_state.db)
        .await
        .map_err(|err| {
            error::ErrorInternalServerError(format!("Failed to get session: {:?}", err))
        })?
        .map(|session| session.user_id)
        .ok_or_else(|| error::ErrorUnauthorized("Unauthorized"))?;

    let event_list = entity::prelude::Event::find()
        .all(&app_state.db)
        .await
        .map_err(|err| {
            error::ErrorInternalServerError(format!("Failed to get events: {:?}", err))
        })?;
    let event_list = event_list
        .into_iter()
        .map(|event| event_scheme::EventListResponse {
            id: event.id.to_string(),
            name: "Test".to_string(),
            description: event.description,
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(event_list))
}
#[derive(Deserialize, Debug)]
struct DiscordUser {
    id: String,
    username: String,
    avatar: String,
    email: Option<String>,
}

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
    client_id: String,
    client_secret: String,
    redirect_url: String,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone());

    migration::Migrator::up(&db, None)
        .await
        .expect("Migrations failed");

    let client_id = secret_store.get("CLIENT_ID").expect("CLIENT_ID is not set");
    let client_secret = secret_store
        .get("CLIENT_SECRET")
        .expect("CLIENT_SECRET is not set");
    let redirect_url = secret_store
        .get("REDIRECT_URL")
        .expect("REDIRECT_URL is not set");

    let state = web::Data::new(AppState {
        db,
        client_id,
        client_secret,
        redirect_url,
    });

    let secret_key = Key::generate();
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .app_data(state.clone())
                .wrap(IdentityMiddleware::default()) // IdentityMiddlewareを登録
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                        .cookie_name("unei".to_owned())
                        .cookie_secure(false)
                        .build(),
                )
                .wrap(middleware::NormalizePath::trim())
                .wrap(middleware::Logger::default())
                .service(index)
                .service(login)
                .service(callback)
                .service(logout)
                .service(groups)
                .service(events)
                .service(get_events)
                .wrap(Logger::default()),
        );
    };

    Ok(config.into())
}
