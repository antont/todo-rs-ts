use axum::http::StatusCode;
use axum::response::IntoResponse;
use todo_rs_ts::error::AppError;

#[test]
fn not_found_maps_to_404() {
    let response = AppError::NotFound.into_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn bad_request_maps_to_400() {
    let response = AppError::BadRequest("bad".into()).into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn internal_maps_to_500() {
    let response = AppError::Internal("oops".into()).into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
