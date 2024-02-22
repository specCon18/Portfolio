use super::templates;
use axum::response::IntoResponse;

pub async fn blog_post() -> impl IntoResponse {
    let template = templates::PostTemplate {
        title:"Test Tile".to_string(),
        description:"this is a test description of a test blog post for rendering in its post template".to_string(),
        post_date:"02/21/2024".to_string()
    };
    templates::HtmlTemplate(template)
}
pub async fn root() -> impl IntoResponse {
    let template = templates::HomeTemplate {};
    templates::HtmlTemplate(template)
}
pub async fn blog() -> impl IntoResponse {
    let template = templates::BlogTemplate {};
    templates::HtmlTemplate(template)
}
pub async fn resume() -> impl IntoResponse {
    let template = templates::ResumeTemplate {};
    templates::HtmlTemplate(template)
}
pub async fn projects() -> impl IntoResponse {
    let template = templates::ProjectsTemplate {};
    templates::HtmlTemplate(template)
}
