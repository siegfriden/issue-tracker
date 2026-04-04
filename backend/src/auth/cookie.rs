use axum::http::request::Parts;
use axum::http::{HeaderValue, header::COOKIE};

/// Build a `Set-Cookie` header value for an auth token.
///
/// Flags: HttpOnly, Secure, SameSite=Lax.
pub fn build_cookie(name: &str, value: &str, max_age_secs: i64, path: &str) -> HeaderValue {
    HeaderValue::from_str(&format!(
        "{name}={value}; HttpOnly; Secure; SameSite=Lax; Path={path}; Max-Age={max_age_secs}"
    ))
    .expect("cookie header value is always valid ASCII")
}

/// Build a `Set-Cookie` header that immediately expires an existing cookie.
/// The path must match the one used when the cookie was set.
pub fn clear_cookie(name: &str, path: &str) -> HeaderValue {
    HeaderValue::from_str(&format!(
        "{name}=; HttpOnly; Secure; SameSite=Lax; Path={path}; Max-Age=0"
    ))
    .expect("cookie header value is always valid ASCII")
}

/// Extract a single cookie value from request parts by name.
pub fn extract_cookie(parts: &Parts, name: &str) -> Option<String> {
    let header = parts.headers.get(COOKIE)?.to_str().ok()?;
    header.split(';').find_map(|pair| {
        let pair = pair.trim();
        pair.strip_prefix(&format!("{name}="))
            .map(|v| v.to_string())
    })
}
