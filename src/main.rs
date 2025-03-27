mod web_request;
mod lambda_request;

use aws_config::BehaviorVersion;
use crate::web_request::{make_alb_request, make_apigw_request, make_http_request};

use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;
use crate::lambda_request::make_lambda_request;

#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let lambda_client = aws_sdk_lambda::Client::new(&aws_config); 
    
    let provider = init_datadog_pipeline().await;
    let tracer = provider.tracer("rust-console");
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_target(false)
        .with_current_span(false)
        .without_time();

    Registry::default()
        .with(telemetry_layer)
        .with(fmt_layer)
        .with(tracing_subscriber::EnvFilter::new("momento_console_harness=info"))
        .init();

    make_apigw_request(&http_client).await;
    make_http_request(&http_client).await;
    make_alb_request(&http_client).await;
    make_lambda_request(&lambda_client).await;

    match provider.shutdown() {
        Ok(_) => println!("Tracer provider shut down successfully"),
        Err(err) => eprintln!("Error shutting down tracer provider: {}", err),
    }
}


async fn init_datadog_pipeline() -> SdkTracerProvider {
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://127.0.0.1:4317")
        .with_protocol(Protocol::Grpc)
        //.with_http_client(client)
        .build().unwrap();
    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .build();

    global::set_tracer_provider(provider.clone());
    provider
}
