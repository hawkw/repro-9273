use hyper::{service, Body, Response};
use std::{future::Future, net::SocketAddr, time::Duration};
use tracing::Instrument;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 42069));
    let timeout = Duration::from_secs(1);

    // let server = hyper::server::Server::try_bind(&addr)?
    //     .http1_header_read_timeout(timeout)
    //     .executor(SpanExecutor(tracing::info_span!("server")))
    //     .serve(service::make_service_fn(|_| async {
    //         Ok::<_, hyper::Error>(service::service_fn(|_| async {
    //             Ok::<_, hyper::Error>(Response::new(Body::from("Hello World")))
    //         }))
    //     }));

    // tokio::spawn(server.instrument(tracing::info_span!("server")));

    let client = hyper::Client::builder()
        .retry_canceled_requests(false)
        .executor(SpanExecutor(tracing::info_span!("client")))
        .build_http::<Body>();

    // let header = (0..100_000_000)
    //     .map(|_| 'a')
    //     .collect::<String>()
    //     .parse::<hyper::header::HeaderValue>()?;

    let mut interval = tokio::time::interval(timeout);
    let url = format!("http://{}/", addr).parse::<hyper::Uri>()?;
    // let req = hyper::Request::get(url.clone())
    //     .header("x-big-header", header.clone())
    //     .body(Body::empty())
    //     .unwrap();
    // let response = client.request(req).await;
    // tracing::info!(?response);
    loop {
        interval.tick().await;
        let response = client.get(url.clone()).await;
        tracing::info!(?response);
        response.expect("response should succeed");
    }

    // // }

    // Ok(())
}

#[derive(Clone)]
struct SpanExecutor(tracing::Span);

impl<F> hyper::rt::Executor<F> for SpanExecutor
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, f: F) {
        tokio::spawn(f.instrument(self.0.clone()));
    }
}
