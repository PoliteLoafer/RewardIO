use crate::auth::dtos::{SigninRequest, SignupRequest, UserResponse};
use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use rewardio_core::AuthError;

pub async fn signup(State(state): State<AppState>, Json(payload): Json<SignupRequest>) -> Response {
    match state.auth_service.sign_up(payload.into_user()).await {
        Ok(user) => (StatusCode::CREATED, Json(UserResponse::from(user))).into_response(),
        Err(e) => map_auth_error(e),
    }
}

pub async fn signin(State(state): State<AppState>, Json(payload): Json<SigninRequest>) -> Response {
    match state
        .auth_service
        .sign_in(&payload.login, &payload.password)
        .await
    {
        Ok(user) => (StatusCode::OK, Json(UserResponse::from(user))).into_response(),
        Err(e) => map_auth_error(e),
    }
}

fn map_auth_error(error: AuthError) -> Response {
    let status = match error {
        AuthError::UserAlreadyExists => StatusCode::BAD_REQUEST,
        AuthError::UserNotFound => StatusCode::NOT_FOUND,
        AuthError::InvalidPassword => StatusCode::UNAUTHORIZED,
        AuthError::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    (
        status,
        Json(serde_json::json!({ "error": error.to_string() })),
    )
        .into_response()
}
