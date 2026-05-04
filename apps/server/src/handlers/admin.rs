use axum::{
    extract::{Query, State},
    response::Json,
};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::auth::middleware::AuthUser;
use crate::domain::audit::AuditEntry;
use crate::error::AppError;
use crate::state::AppState;

#[derive(Deserialize, ToSchema)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default = "default_offset")]
    offset: i64,
}

fn default_limit() -> i64 {
    50
}
fn default_offset() -> i64 {
    0
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/audit-logs",
    tag = "admin",
    params(
        ("limit" = Option<i64>, Query, description = "Page limit"),
        ("offset" = Option<i64>, Query, description = "Page offset"),
    ),
    responses(
        (status = 200, description = "Audit logs", body = Vec<AuditEntry>),
        (status = 403, description = "Forbidden"),
    ),
    security(
        ("bearer_auth" = [])
),
)]
pub async fn list_audit_logs(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<AuditEntry>>, AppError> {
    let logs = state
        .audit_service
        .list_audit_logs(auth_user.user_id_uuid()?, query.limit, query.offset)
        .await?;
    Ok(Json(logs))
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::get;

    axum::Router::new().route("/admin/audit-logs", get(list_audit_logs))
}
