mod http;
mod image_resizer;
mod logging;
mod settings;

#[tokio::main]
async fn main() {
    let settings = settings::initialize();
    let _logging_guard = logging::initialize(&settings.log);
    let (address, server) = http::start(&settings.http, settings.image_directory);
    tracing::info!("Now listening on: {}", address);
    server.await
}
