use axum::http::{HeaderMap, header};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rewardio_core::{User, UserRole};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use uuid::Uuid;

const AUTH_COOKIE_NAME: &str = "rewardio_auth";

#[derive(Clone)]
pub struct SessionManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
    ttl_seconds: u64,
    cookie_secure: bool,
    revoked_jti: Arc<RwLock<HashSet<String>>>,
}

#[derive(Debug, Clone)]
pub struct SessionUser {
    pub login: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub jti: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SessionClaims {
    sub: String,
    name: String,
    email: String,
    role: String,
    jti: String,
    exp: u64,
    iat: u64,
}

impl SessionManager {
    pub fn new(secret: &str, ttl_seconds: u64, cookie_secure: bool) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            validation,
            ttl_seconds,
            cookie_secure,
            revoked_jti: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub fn issue_token(&self, user: &User) -> Result<String, jsonwebtoken::errors::Error> {
        let issued_at = now_unix_seconds();
        let claims = SessionClaims {
            sub: user.login.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
            role: role_to_claim(&user.role).to_string(),
            jti: Uuid::new_v4().to_string(),
            exp: issued_at + self.ttl_seconds,
            iat: issued_at,
        };

        encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
    }

    pub async fn authorized_user_from_headers(&self, headers: &HeaderMap) -> Option<SessionUser> {
        let token = read_auth_cookie_token(headers)?;

        let data = decode::<SessionClaims>(&token, &self.decoding_key, &self.validation).ok()?;
        let claims = data.claims;

        if self.revoked_jti.read().await.contains(&claims.jti) {
            return None;
        }

        Some(SessionUser {
            login: claims.sub,
            name: claims.name,
            email: claims.email,
            role: role_from_claim(&claims.role)?,
            jti: claims.jti,
        })
    }

    pub fn auth_cookie(&self, token: &str) -> String {
        format!(
            "{AUTH_COOKIE_NAME}={token}; Path=/; Max-Age={}; HttpOnly; SameSite=Strict{}",
            self.ttl_seconds,
            if self.cookie_secure { "; Secure" } else { "" }
        )
    }

    pub fn clear_auth_cookie(&self) -> String {
        format!(
            "{AUTH_COOKIE_NAME}=; Path=/; Max-Age=0; HttpOnly; SameSite=Strict{}",
            if self.cookie_secure { "; Secure" } else { "" }
        )
    }

    pub async fn revoke(&self, jti: String) {
        self.revoked_jti.write().await.insert(jti);
    }
}

fn read_auth_cookie_token(headers: &HeaderMap) -> Option<String> {
    let cookie_value = headers.get(header::COOKIE)?.to_str().ok()?;

    cookie_value
        .split(';')
        .map(str::trim)
        .find_map(|part| part.strip_prefix(&format!("{AUTH_COOKIE_NAME}=")))
        .map(ToOwned::to_owned)
}

fn role_to_claim(role: &UserRole) -> &'static str {
    match role {
        UserRole::Admin => "admin",
        UserRole::User => "user",
    }
}

fn role_from_claim(role: &str) -> Option<UserRole> {
    match role {
        "admin" => Some(UserRole::Admin),
        "user" => Some(UserRole::User),
        _ => None,
    }
}

fn now_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_secs()
}