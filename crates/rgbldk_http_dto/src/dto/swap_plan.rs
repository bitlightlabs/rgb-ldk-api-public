// Generated from rgb-ldk-node/crates/application/src/swap_plan.rs. Do not edit.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct RouteEdge {
	pub node_id: String,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub channel_scid: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct LegConstraints {
	/// When set, no node/channel substitutions or insertions are authorized.
	#[serde(default)]
	pub fixed_route: Option<Vec<RouteEdge>>,
	/// Ordered mandatory nodes, including any required LSPs.
	pub required_nodes: Vec<String>,
	#[serde(with = "decimal_vec")]
	#[schema(value_type = Vec<String>)]
	pub required_channels: Vec<u64>,
	pub required_lsps: Vec<String>,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub max_fee_msat: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct RoutingConstraints {
	pub maker_to_taker: LegConstraints,
	pub taker_to_maker: LegConstraints,
	pub max_total_cltv_delta: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum SettlementMode {
	CircularHtlc,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum FeePayer {
	Maker,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum LegDirection {
	MakerToTaker,
	TakerToMaker,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum LegAsset {
	Btc,
	Rgb { asset_id: Vec<u8> },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SwapQuote {
	/// Stable identity, equal to the payment hash in hexadecimal. Requotes get a new identity.
	pub execution_id: String,
	pub maker: String,
	pub taker: String,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub taker_outbound_channel_scid: u64,
	pub asset_id: Vec<u8>,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub asset_amount: u64,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub btc_amount_msat: u64,
	/// Carrier at the exchange boundary; return routing fees are deducted before maker receipt.
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub btc_carrier_amount_msat: u64,
	pub maker_gives_rgb: bool,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub created_at_unix_secs: u64,
	pub expiry_secs: u32,
	pub settlement_mode: SettlementMode,
	pub maker_to_taker_fee_payer: FeePayer,
	pub taker_to_maker_fee_payer: FeePayer,
	pub constraints: RoutingConstraints,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct DirectionalPolicy {
	/// RGB contract carried by this channel, if any.
	#[serde(default)]
	pub asset_id: Option<Vec<u8>>,
	pub source: String,
	pub destination: String,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub channel_scid: u64,
	pub base_fee_msat: u32,
	pub fee_proportional_millionths: u32,
	pub cltv_expiry_delta: u32,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub htlc_minimum_msat: u64,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub htlc_maximum_msat: u64,
	#[serde(with = "decimal_option")]
	#[schema(value_type = Option<String>)]
	pub rgb_htlc_maximum: Option<u64>,
	pub enabled: bool,
	/// Source of the snapshot (local channel state or gossip).
	pub provenance: String,
	pub last_update: Option<u32>,
}

/// A node reached over a channel, with its incoming and outgoing HTLC. The final maker hop
/// has no outgoing HTLC; the last hop of maker_to_taker contains the asset exchange.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct HopPlan {
	pub source: String,
	pub node_id: String,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub channel_scid: u64,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub btc_input_msat: u64,
	#[serde(with = "decimal_option")]
	#[schema(value_type = Option<String>)]
	pub btc_output_msat: Option<u64>,
	#[serde(with = "decimal_option")]
	#[schema(value_type = Option<String>)]
	pub rgb_input: Option<u64>,
	#[serde(with = "decimal_option")]
	#[schema(value_type = Option<String>)]
	pub rgb_output: Option<u64>,
	pub incoming_cltv_delta: u32,
	pub outgoing_cltv_delta: Option<u32>,
	/// Policy for source -> node_id, not the reverse channel direction.
	pub policy: DirectionalPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct LegPlan {
	pub direction: LegDirection,
	pub asset: LegAsset,
	pub fee_payer: FeePayer,
	pub constraints: LegConstraints,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub fee_msat: u64,
	pub hops: Vec<HopPlan>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ExecutionPlan {
	pub format_version: u32,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub version: u64,
	pub quote: SwapQuote,
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub policy_snapshot_at_unix_secs: u64,
	pub maker_to_taker: LegPlan,
	pub taker_to_maker: LegPlan,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum AttemptStatus {
	/// Persisted before send. Recovery must reconcile this attempt before any resend.
	Prepared,
	Submitted,
	SendRejected,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SendAttempt {
	#[serde(with = "decimal")]
	#[schema(value_type = String)]
	pub plan_version: u64,
	pub status: AttemptStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SwapExecution {
	pub format_version: u32,
	#[serde(default)]
	pub policy_hints: Vec<DirectionalPolicy>,
	pub quote: SwapQuote,
	pub maker_to_taker: Vec<RouteEdge>,
	pub taker_to_maker: Vec<RouteEdge>,
	/// None until the first plan is calculated. The paths above are then its route selection.
	#[serde(with = "decimal_option")]
	#[schema(value_type = Option<String>)]
	pub selected_plan: Option<u64>,
	pub plans: Vec<ExecutionPlan>,
	pub attempts: Vec<SendAttempt>,
}

// Decimal strings preserve SCIDs and amounts through JavaScript clients. Numeric input remains
// accepted for compatibility with existing offers and Rust clients.
#[derive(Deserialize)]
#[serde(untagged)]
enum DecimalInput {
	Text(String),
	Number(u64),
}

impl DecimalInput {
	fn value<E: serde::de::Error>(self) -> Result<u64, E> {
		match self {
			Self::Text(s) => s.parse().map_err(E::custom),
			Self::Number(n) => Ok(n),
		}
	}
}

mod decimal {
	use super::*;

	pub fn serialize<S: serde::Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.collect_str(value)
	}

	pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
		DecimalInput::deserialize(d)?.value()
	}
}

mod decimal_option {
	use super::*;

	pub fn serialize<S: serde::Serializer>(v: &Option<u64>, s: S) -> Result<S::Ok, S::Error> {
		v.map(|n| n.to_string()).serialize(s)
	}

	pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<u64>, D::Error> {
		Option::<DecimalInput>::deserialize(d)?.map(DecimalInput::value).transpose()
	}
}

mod decimal_vec {
	use super::*;

	pub fn serialize<S: serde::Serializer>(v: &[u64], s: S) -> Result<S::Ok, S::Error> {
		v.iter().map(u64::to_string).collect::<Vec<_>>().serialize(s)
	}

	pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<u64>, D::Error> {
		Vec::<DecimalInput>::deserialize(d)?.into_iter().map(DecimalInput::value).collect()
	}
}
