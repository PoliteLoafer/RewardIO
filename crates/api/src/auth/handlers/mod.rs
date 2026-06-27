use crate::auth::dtos::{SigninRequest, SignupRequest, UserResponse};
use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header},
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
        Ok(user) => {
            let token = match state.session_manager.issue_token(&user) {
                Ok(token) => token,
                Err(error) => {
                    tracing::error!("failed to issue auth token: {error}");
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "error": "Internal server error" })),
                    )
                        .into_response();
                }
            };

            let mut response = (StatusCode::OK, Json(UserResponse::from(user))).into_response();
            response.headers_mut().append(
                header::SET_COOKIE,
                state
                    .session_manager
                    .auth_cookie(&token)
                    .parse()
                    .expect("valid auth cookie header"),
            );
            response
        }
        Err(e) => map_auth_error(e),
    }
}

pub async fn me(State(state): State<AppState>, headers: HeaderMap) -> Response {
    match state
        .session_manager
        .authorized_user_from_headers(&headers)
        .await
    {
        Some(user) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "login": user.login,
                "name": user.name,
                "email": user.email,
                "role": user.role,
            })),
        )
            .into_response(),
        None => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Unauthorized" })),
        )
            .into_response(),
    }
}

pub async fn logout(State(state): State<AppState>, headers: HeaderMap) -> Response {
    if let Some(user) = state
        .session_manager
        .authorized_user_from_headers(&headers)
        .await
    {
        state.session_manager.revoke(user.jti).await;
    }

    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().append(
        header::SET_COOKIE,
        state
            .session_manager
            .clear_auth_cookie()
            .parse()
            .expect("valid clear auth cookie header"),
    );
    response
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
