use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let assets_path = std::env::current_dir().unwrap();
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "portfolio=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("hello, web server!");
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/blog", get(blog))
        .route("/resume", get(resume))
        .route("/projects", get(projects))
        .nest_service("/assets", ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn root() -> impl IntoResponse {
    let template = HelloTemplate {};
    HtmlTemplate(template)
}
async fn blog() -> impl IntoResponse {
    let template = BlogTemplate {};
    HtmlTemplate(template)
}
async fn resume() -> impl IntoResponse {
    let template = ResumeTemplate {};
    HtmlTemplate(template)
}
async fn projects() -> impl IntoResponse {
    let template = ProjectsTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate;

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate;

#[derive(Template)]
#[template(path = "resume.html")]
struct ResumeTemplate;

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate;

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}