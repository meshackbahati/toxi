use toxi_core::{ToxiRequest, ToxiResponse, Error as CoreError};
use tower::{Service, Layer};
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crate::JwtManager;

/// Auth middleware that validates JWT tokens.
///
/// Holds precomputed verification keys shared across requests, so token
/// verification performs no key derivation on the hot path.
#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    manager: Arc<JwtManager>,
}

impl<S> AuthMiddleware<S> {
    /// Create a new `AuthMiddleware` that validates JWT tokens with the given secret.
    pub fn new(inner: S, secret: String) -> Self {
        Self {
            inner,
            manager: Arc::new(JwtManager::new(secret)),
        }
    }
}

impl<S> Service<ToxiRequest> for AuthMiddleware<S>
where
    S: Service<ToxiRequest, Response = ToxiResponse, Error = CoreError> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: ToxiRequest) -> Self::Future {
        // Extract Authorization header before moving req
        let token = req
            .headers()
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|s| s.to_string());

        let manager = self.manager.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Verify token
            if let Some(token_str) = token {
                match manager.verify(&token_str) {
                    Ok(claims) => {
                        let mut req = req;
                        // Parse before moving so the claims insert needs no clone.
                        let user_id = claims.sub.parse::<i64>().ok();
                        req.extensions_mut().insert(claims);
                        if let Some(user_id) = user_id {
                            req.extensions_mut().insert(user_id);
                        }
                        // Token is valid, proceed with request
                        inner.call(req).await
                    }
                    Err(_) => {
                        // Invalid token
                        Err(CoreError::Unauthorized("Invalid token".to_string()))
                    }
                }
            } else {
                // No token provided
                Err(CoreError::Unauthorized("Missing authorization header".to_string()))
            }
        })
    }
}

/// Layer for Auth middleware
pub struct AuthLayer {
    secret: String,
}

impl AuthLayer {
    /// Create a new `AuthLayer` that validates JWT tokens with the given secret.
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware::new(inner, self.secret.clone())
    }
}
