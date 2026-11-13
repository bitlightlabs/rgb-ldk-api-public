use rgbldk_http_dto::dto::SwapValidationErrorResponse;

#[test]
fn observed_limits_and_large_scids_roundtrip_without_precision_loss() {
    let wire: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/swap-validation-errors.json"
    ))
    .unwrap();
    let errors: Vec<SwapValidationErrorResponse> = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(serde_json::to_value(errors).unwrap(), wire);
}
