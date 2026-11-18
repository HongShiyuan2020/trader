use axum::{extract::{FromRef, FromRequestParts}, http::{Error, StatusCode, header}};

pub struct CurrentUser {
    pub uid: String,
}

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or((StatusCode::UNAUTHORIZED, "Not authorized".to_string()))?;

        let path = parts.uri.path().to_string();

        Ok(CurrentUser {
            uid: format!("{token}:{path}").to_string()
        })
    }
}
