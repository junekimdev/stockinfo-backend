use super::settings;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace;
use tracing_opentelemetry::OpenTelemetryLayer;

pub struct OpenTelemetry {
    pub jaeger_layer: OpenTelemetryLayer<tracing_subscriber::Registry, trace::Tracer>,
}

impl OpenTelemetry {
    #[tracing::instrument]
    pub fn new(app_name: &str, app_version: &str) -> OpenTelemetry {
        // tracing with opentelemetry + jaeger
        let jaeger_exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(settings::Settings::instance().jaeger.agent_endpoint.clone())
            .with_timeout(std::time::Duration::from_secs(3));

        let tracer_config = trace::Config::default()
            .with_sampler(trace::Sampler::AlwaysOn)
            .with_id_generator(trace::RandomIdGenerator::default())
            .with_max_events_per_span(64)
            .with_max_attributes_per_span(16)
            .with_max_events_per_span(16)
            .with_resource(opentelemetry_sdk::Resource::new(vec![
                opentelemetry::KeyValue::new("service.name", app_name.to_string()),
                opentelemetry::KeyValue::new("service.version", app_version.to_string()),
            ]));

        let jaeger_tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(jaeger_exporter)
            .with_trace_config(tracer_config)
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .expect("failed to install OpenTelemetry tracer")
            .tracer("opentelemetry-otlp");

        let jaeger_layer = tracing_opentelemetry::layer().with_tracer(jaeger_tracer);

        // metrics with opentelemetry + prometheus
        Self { jaeger_layer }
    }
}
