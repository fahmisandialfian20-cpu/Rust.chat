use axum::{
    extract::State,
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;
use time::OffsetDateTime;

use crate::auth::middleware::AuthUser;
use crate::domain::user::ThemePreferences;
use crate::state::AppState;
use crate::error::AppError;

#[derive(Debug, Deserialize)]
pub struct UpdateThemeRequest {
    pub mode: Option<String>,
    pub accent: Option<String>,
    pub density: Option<String>,
    pub message_display: Option<String>,
    pub settings: Option<serde_json::Value>,
}

pub async fn get_theme(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ThemePreferences>, AppError> {
    let user_id = auth_user.user_id_uuid()?;

    let row = sqlx::query_as::<_, SqlxTheme>(
        r#"
        SELECT id, user_id, theme, accent_color, settings, created_at, updated_at
        FROM user_theme_preferences
        WHERE user_id = $1
        "#
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    match row {
        Some(r) => {
            let mode = r.theme;
            let accent = r.accent_color.unwrap_or_else(|| "brand".to_string());
            let density = r.settings.get("density").and_then(|v| v.as_str()).unwrap_or("comfortable").to_string();
            let message_display = r.settings.get("message_display").and_then(|v| v.as_str()).unwrap_or("cozy").to_string();

            Ok(Json(ThemePreferences {
                id: r.id,
                user_id,
                mode,
                accent,
                density,
                message_display,
                settings: r.settings,
            }))
        }
        None => Ok(Json(ThemePreferences {
            id: Uuid::nil(),
            user_id,
            mode: "dark".to_string(),
            accent: "brand".to_string(),
            density: "comfortable".to_string(),
            message_display: "cozy".to_string(),
            settings: serde_json::json!({}),
        })),
    }
}

pub async fn update_theme(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateThemeRequest>,
) -> Result<Json<ThemePreferences>, AppError> {
    let user_id = auth_user.user_id_uuid()?;

    let mode = payload.mode.unwrap_or_else(|| "dark".to_string());
    let accent = payload.accent;
    let density = payload.density.unwrap_or_else(|| "comfortable".to_string());
    let message_display = payload.message_display.unwrap_or_else(|| "cozy".to_string());

    let existing_settings = sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT settings FROM user_theme_preferences WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::InternalServerError(e.to_string()))?
    .unwrap_or_else(|| serde_json::json!({}));

    let mut map = match existing_settings {
        serde_json::Value::Object(obj) => obj,
        _ => serde_json::Map::new(),
    };

    map.insert("density".to_string(), serde_json::Value::String(density));
    map.insert("message_display".to_string(), serde_json::Value::String(message_display));

    if let Some(settings) = payload.settings {
        if let serde_json::Value::Object(obj) = settings {
            for (k, v) in obj {
                map.insert(k, v);
            }
        }
    }

    let merged_settings = serde_json::Value::Object(map);

    let row = sqlx::query_as::<_, SqlxTheme>(
        r#"
        INSERT INTO user_theme_preferences (id, user_id, theme, accent_color, settings, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $6)
        ON CONFLICT (user_id) DO UPDATE SET
            theme = EXCLUDED.theme,
            accent_color = EXCLUDED.accent_color,
            settings = EXCLUDED.settings,
            updated_at = EXCLUDED.updated_at
        RETURNING id, user_id, theme, accent_color, settings, created_at, updated_at
        "#
    )
    .bind(Uuid::now_v7())
    .bind(user_id)
    .bind(&mode)
    .bind(&accent)
    .bind(&merged_settings)
    .bind(OffsetDateTime::now_utc())
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let accent = row.accent_color.unwrap_or_else(|| "brand".to_string());
    let density = row.settings.get("density").and_then(|v| v.as_str()).unwrap_or("comfortable").to_string();
    let message_display = row.settings.get("message_display").and_then(|v| v.as_str()).unwrap_or("cozy").to_string();

    Ok(Json(ThemePreferences {
        id: row.id,
        user_id,
        mode: row.theme,
        accent,
        density,
        message_display,
        settings: row.settings,
    }))
}

#[derive(sqlx::FromRow)]
struct SqlxTheme {
    id: Uuid,
    user_id: Uuid,
    theme: String,
    accent_color: Option<String>,
    settings: serde_json::Value,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::{get, put};
    axum::Router::new()
        .route("/profile/theme", get(get_theme))
        .route("/profile/theme", put(update_theme))
}
