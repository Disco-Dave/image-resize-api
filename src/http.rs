use std::convert::Infallible;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use image::ImageError;
use uuid::Uuid;
use warp::reply::Response;
use warp::{Filter, Future, Reply};

use crate::image_resizer;
use crate::settings::HttpSettings;

#[derive(serde::Deserialize)]
struct ResizeQuery {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

async fn resize(
    path: PathBuf,
    query: ResizeQuery,
    image_directory: Arc<PathBuf>,
) -> Result<Response, Infallible> {
    let image = image_directory.join(&path);

    let resize_task = tokio::task::spawn_blocking(move || {
        image_resizer::resize(image, query.width, query.height)
    });

    let response = match resize_task.await {
        Ok(Err(ImageError::IoError(e))) if e.kind() == ErrorKind::NotFound => {
            warp::reply::with_status(warp::reply(), http::StatusCode::NOT_FOUND).into_response()
        }
        Err(_) | Ok(Err(_)) => {
            warp::reply::with_status(warp::reply(), http::StatusCode::INTERNAL_SERVER_ERROR)
                .into_response()
        }
        Ok(Ok(bytes)) => http::response::Builder::new()
            .status(200)
            .body(bytes)
            .into_response(),
    };

    Ok(response)
}

fn remaining_path() -> impl Filter<Extract = (PathBuf,), Error = Infallible> + Copy {
    use warp::path::Tail;
    warp::path::tail().map(|tail: Tail| tail.as_str().into())
}

fn with_data<T: Clone + Send>(data: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    warp::any().map(move || data.clone())
}

pub fn start(
    http_settings: &HttpSettings,
    image_directory: PathBuf,
) -> (SocketAddr, impl Future<Output = ()> + 'static) {
    let get_health_check = warp::path("health-check")
        .and(warp::get())
        .map(|| http::StatusCode::NO_CONTENT);

    let image_directory = Arc::new(image_directory);

    let get_resize = warp::get()
        .and(remaining_path())
        .and(warp::filters::query::query())
        .and(with_data(image_directory))
        .and_then(resize);

    let routes = get_health_check.or(get_resize);

    let filters = routes
        .with(warp::filters::trace::request())
        .with(warp::filters::trace::trace(|_info| {
            let request_id = Uuid::new_v4();
            tracing::info_span!("request", id = ?request_id)
        }));

    let address: SocketAddr = format!("{}:{}", http_settings.host, http_settings.port)
        .parse()
        .expect("Invalid host and/or port provided.");

    warp::serve(filters).bind_ephemeral(address)
}
