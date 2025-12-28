use opentelemetry::metrics::MeterProvider as _;
use opentelemetry::KeyValue;
use opentelemetry_prometheus::PrometheusExporterBuilder;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::Resource;
use prometheus::Encoder as PrometheusEncoder;
use prometheus::TextEncoder;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static METRICS_EXPORTER: Mutex<Option<Arc<prometheus::Registry>>> = Mutex::const_new(None);

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "screen_recorder=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    // Initialize Prometheus metrics
    let exporter = opentelemetry_prometheus::exporter()
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            "screen-recorder",
        )]))
        .build()?;

    let provider = SdkMeterProvider::builder()
        .with_reader(exporter)
        .build();

    opentelemetry::global::set_meter_provider(provider);

    // Create Prometheus registry
    let registry = Arc::new(prometheus::Registry::new());

    // Store registry for metrics endpoint
    tokio::spawn(async {
        let mut guard = METRICS_EXPORTER.lock().await;
        *guard = Some(registry);
    });

    tracing::info!("Observability initialized");

    Ok(())
}

pub fn record_event(event_name: &str, attributes: &[(&str, &str)]) {
    tracing::info!(
        event = event_name,
        ?attributes,
        "Recording event"
    );
}

pub async fn get_metrics() -> Result<String, Box<dyn std::error::Error>> {
    let guard = METRICS_EXPORTER.lock().await;
    if let Some(registry) = guard.as_ref() {
        let encoder = TextEncoder::new();
        let metric_families = registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    } else {
        Ok(String::new())
    }
}

