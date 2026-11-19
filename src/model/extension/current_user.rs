use axum::{extract::{FromRef, FromRequestParts}, http::{Error, StatusCode, header}};

pub struct CurrentUser {
    pub user_id: i64,
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
        
        let _path = parts.uri.path();
        
        Ok(CurrentUser {
            user_id: token.parse().unwrap_or(0),
        })
    }
}
