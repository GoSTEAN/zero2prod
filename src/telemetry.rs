use tracing::{span, Event};
use tracing::Subscriber as TracingSubscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer}; 
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

pub trait Subscriber {
    fn new_span(&self, span: &span::Attributes<'_>) -> span::Id; 
    fn event(&self, event: &Event<'_>);
    fn enter(&self, span: &span::Id);
    fn exit(&self, span: &span::Id);
    fn clone_span(&self, id: &span::Id) -> span::Id;
}

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl TracingSubscriber + Sync + Send 
where 
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(_subscriber: impl TracingSubscriber + Sync + Send) {

}
