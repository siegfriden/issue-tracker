use issue_tracker_api::openapi::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let yaml = ApiDoc::openapi()
        .to_yaml()
        .expect("failed to serialize OpenAPI spec");

    let dest = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "openapi.yaml".into());

    std::fs::write(&dest, &yaml).expect("failed to write OpenAPI spec");
    eprintln!("wrote {dest}");
}
