use rgbldk_http_dto::*;
use serde_json::{json, Value};

fn fixtures() -> Vec<SwapInfoDto> {
    serde_json::from_str(include_str!("../../../tests/fixtures/swap-executions.json")).unwrap()
}

#[test]
fn upstream_execution_fixtures_round_trip_without_losing_amounts_or_enum_shapes() {
    let expected: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/swap-executions.json")).unwrap();
    let decoded = fixtures();
    assert_eq!(serde_json::to_value(&decoded).unwrap(), expected);

    for info in decoded {
        let execution = info.execution.unwrap();
        assert_eq!(execution.format_version, 1);
        assert_eq!(execution.selected_plan, Some(1));
        assert_eq!(execution.plans.len(), 1);
        let plan = &execution.plans[0];
        let first = &plan.maker_to_taker.hops;
        let second = &plan.taker_to_maker.hops;
        assert_eq!(first[0].channel_scid, 9_007_199_254_740_993);
        assert_eq!(plan.version, 1);
        assert_eq!(execution.attempts[0].plan_version, 1);
        assert!(matches!(
            execution.attempts[0].status,
            AttemptStatus::Submitted
        ));
        assert_eq!(second.last().unwrap().btc_output_msat, None);
        assert_eq!(second.last().unwrap().rgb_output, None);
        assert_eq!(second.last().unwrap().outgoing_cltv_delta, None);

        let inbound = first.last().unwrap().btc_input_msat;
        let outbound = second[0].btc_input_msat;

        if info.maker_gives_rgb {
            assert!(matches!(plan.maker_to_taker.asset, LegAsset::Rgb { .. }));
            assert!(matches!(plan.taker_to_maker.asset, LegAsset::Btc));
            assert_eq!(outbound - inbound, info.btc_amount_msat);
        } else {
            assert!(matches!(plan.maker_to_taker.asset, LegAsset::Btc));
            assert!(matches!(plan.taker_to_maker.asset, LegAsset::Rgb { .. }));
            assert_eq!(inbound - outbound, info.btc_amount_msat);
        }
    }
}

#[test]
fn legacy_selectors_deserialize_but_serialize_as_execution_id() {
    let id = "ab".repeat(32);
    let request: SwapExecuteRequest =
        serde_json::from_value(json!({ "payment_hash": id })).unwrap();
    assert_eq!(request.execution_id.as_deref(), Some(id.as_str()));
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({ "execution_id": id })
    );
    assert!(serde_json::from_value::<SwapExecuteRequest>(
        json!({ "payment_hash": id, "execution_id": id })
    )
    .is_err());
}

#[test]
fn historical_routes_are_input_aliases_and_constraints_remain_typed() {
    let info = fixtures().remove(0);
    let execution = info.execution.unwrap();
    let request: SwapCreateMultihopOfferRequest = serde_json::from_value(json!({
        "rgb_path": execution.maker_to_taker,
        "btc_path": execution.taker_to_maker,
        "contract_id": info.contract_id,
        "asset_amount": "100",
        "btc_amount_msat": "5000",
        "btc_carrier_amount_msat": "10000",
        "maker_gives_rgb": true,
        "expiry_secs": 3600,
        "constraints": execution.quote.constraints
    }))
    .unwrap();
    assert_eq!(
        request.maker_to_taker[0].channel_scid,
        9_007_199_254_740_993
    );
    assert_eq!(
        request
            .constraints
            .as_ref()
            .unwrap()
            .maker_to_taker
            .max_fee_msat,
        1000
    );
    let wire = serde_json::to_value(request).unwrap();
    assert!(wire.get("rgb_path").is_none());
    assert!(wire.get("btc_path").is_none());
    assert!(wire.get("maker_to_taker").is_some());
}

#[test]
fn nested_wire_amounts_accept_legacy_numbers_but_emit_decimal_strings() {
    let edge: RouteEdge = serde_json::from_value(json!({
        "node_id": "fixture",
        "channel_scid": 9_007_199_254_740_993_u64
    }))
    .unwrap();
    assert_eq!(
        serde_json::to_value(edge).unwrap()["channel_scid"],
        "9007199254740993"
    );
}

#[test]
fn policy_rgb_limit_is_required_but_can_explicitly_be_null() {
    let info = fixtures().remove(0);
    let policy = &info.execution.unwrap().plans[0].maker_to_taker.hops[0].policy;
    let mut wire = serde_json::to_value(policy).unwrap();
    wire.as_object_mut().unwrap().remove("rgb_htlc_maximum");
    assert!(serde_json::from_value::<DirectionalPolicy>(wire.clone()).is_err());

    wire["rgb_htlc_maximum"] = Value::Null;
    let decoded: DirectionalPolicy = serde_json::from_value(wire).unwrap();
    assert_eq!(decoded.rgb_htlc_maximum, None);
}
