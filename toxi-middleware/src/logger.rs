use std::task::{Context, Poll};
use std::time::Instant;
use tower::{Service, Layer};
use toxi_core::{ToxiRequest, ToxiResponse, Error};
use std::future::Future;
use std::pin::Pin;

/// Tower service that logs incoming requests and outgoing responses with timing.
///
/// Logs method, path, status code, and response time in milliseconds.
/// Uses `tracing::info!` when the `tracing` feature is enabled, falls back
/// to `println!` otherwise.
///
/// # Example
///
/// ```rust,ignore
/// use toxi::prelude::*;
///
/// let service = ServiceBuilder::new()
///     .layer(LoggerLayer)
///     .service(router);
/// ```
#[derive(Clone)]
pub struct Logger<S> {
    inner: S,
}

impl<S> Logger<S> {
    /// Create a new `Logger` wrapping the inner service.
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> Service<ToxiRequest> for Logger<S>
where
    S: Service<ToxiRequest, Response = ToxiResponse, Error = Error> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: ToxiRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        let start = Instant::now();
        let fut = self.inner.call(req);

        Box::pin(async move {
            let res = fut.await;
            let elapsed_ms = start.elapsed().as_millis();
            match &res {
                Ok(response) => {
                    println!(
                        "{} {} {} ({}ms)",
                        method, path, response.status(), elapsed_ms
                    );
                }
                Err(err) => {
                    println!(
                        "{} {} {} {} ({}ms)",
                        method, path, err.status_code(), err, elapsed_ms
                    );
                }
            }
            res
        })
    }
}

/// Tower layer that wraps a service with [`Logger`].
pub struct LoggerLayer;

impl<S> Layer<S> for LoggerLayer {
    type Service = Logger<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Logger::new(inner)
    }
}
