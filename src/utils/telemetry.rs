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

        let version = opentelemetry::KeyValue::new("service.version", app_version.to_string());
        let resource = opentelemetry_sdk::Resource::builder()
            .with_attribute(version)
            .build();
        let end_point = settings::Settings::instance().jaeger.agent_endpoint.clone();

        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(end_point)
            .with_timeout(std::time::Duration::from_secs(3))
            .build()
            .expect("failed to build opentelemetry exporter");

        let provider = trace::SdkTracerProvider::builder()
            .with_batch_exporter(exporter)
            .with_sampler(trace::Sampler::AlwaysOn)
            .with_id_generator(trace::RandomIdGenerator::default())
            .with_max_events_per_span(64)
            .with_max_attributes_per_span(16)
            .with_max_events_per_span(16)
            .with_resource(resource)
            .build();

        let tracer = provider.tracer(app_name.to_string());

        let jaeger_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        // metrics with opentelemetry + prometheus
        Self { jaeger_layer }
    }
}
