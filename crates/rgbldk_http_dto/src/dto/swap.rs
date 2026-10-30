// Generated from rgb-ldk-node/crates/node-http/src/dto/swap.rs. Do not edit.

//! Swap (BTC <-> RGB) request/response DTOs, mirroring `Node::swap()` (`rgb_ldk_node::Swap`).

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::serde_u64_decimal_string;
use super::swap_plan::*;

/// Request to create a single-hop swap offer (maker side).
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SwapCreateOfferRequest {
	/// Counterparty node public key, hex-encoded.
	pub counterparty_node_id: String,
	/// The short_channel_id (or alias) used in the outgoing onion for the forwarding hop.
	#[serde(with = "serde_u64_decimal_string")]
	pub channel_scid: u64,
	/// RGB contract id.
	pub contract_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_amount: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub btc_amount_msat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub btc_carrier_amount_msat: u64,
	/// If true: maker gives RGB, taker gives BTC. If false: maker gives BTC, taker gives RGB.
	pub maker_gives_rgb: bool,
	pub expiry_secs: u32,
}

/// One hop in a multi-hop swap path: a node id and the channel SCID used to reach it.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SwapHopDto {
	/// Node public key, hex-encoded.
	pub node_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub channel_scid: u64,
}

/// Request to create a multi-hop swap offer (maker side).
///
/// Supply both directed halves of the circular route. Asset direction follows `maker_gives_rgb`.
/// The first return channel is bound as the taker's intercepted outgoing channel.
/// `rgb_path` and `btc_path` remain input aliases for older clients.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SwapCreateMultihopOfferRequest {
	/// Directed policy snapshots for private remote channels absent from gossip.
	#[serde(default)]
	pub policy_hints: Vec<DirectionalPolicy>,
	/// Fee caps and mandatory nodes, LSPs and channels. Omission fixes the supplied route with zero routing fees.
	#[serde(default)]
	pub constraints: Option<RoutingConstraints>,
	#[serde(alias = "rgb_path")]
	pub maker_to_taker: Vec<SwapHopDto>,
	#[serde(alias = "btc_path")]
	pub taker_to_maker: Vec<SwapHopDto>,
	pub contract_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_amount: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub btc_amount_msat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub btc_carrier_amount_msat: u64,
	pub maker_gives_rgb: bool,
	pub expiry_secs: u32,
}

/// Response to a successful offer creation (single- or multi-hop).
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SwapOfferResponse {
	pub execution_id: String,
	/// Out-of-band offer string (`rgb-swap:v1:...` or `rgb-swap:v3:...`) to hand to the
	/// counterparty (e.g. via QR code or chat).
	pub swap_string: String,
	/// The swap's primary key, hex-encoded.
	pub payment_hash: String,
	/// The freshly created swap's details.
	pub info: SwapInfoDto,
}

/// Request to execute a previously created offer (maker side). Provide either `swap_string`
/// or a stable `execution_id` (`payment_hash` is a compatibility alias), for either route type.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SwapExecuteRequest {
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub policy_hints: Vec<DirectionalPolicy>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub swap_string: Option<String>,
	/// Hex-encoded 32-byte payment hash.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[serde(alias = "payment_hash")]
	pub execution_id: Option<String>,
	/// Optional replacement route within the accepted constraints. Both legs must be supplied.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub maker_to_taker: Option<Vec<SwapHopDto>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub taker_to_maker: Option<Vec<SwapHopDto>>,
	/// Skip the "taker has accepted" gate. By default execute refuses until a `SWAP_ACCEPT` has
	/// advanced the swap to `Accepted`; set this when acceptance was coordinated out-of-band or the
	/// taker runs an older node that does not send the message. Defaults to `false`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub force: Option<bool>,
}

/// Response to `POST /swap/execute`.
///
/// **Important:** `ok: true` only means the circular payment was successfully *initiated*. It does
/// NOT mean the swap has settled or that any channel balance has moved yet. Settlement is
/// asynchronous — poll `GET /swap/{payment_hash}` until `status` reaches `"Settled"` (success) or
/// `"Failed"` (see `last_error` there for the reason).
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SwapExecuteResponse {
	pub execution_id: String,
	/// True when the payment was accepted for sending. Not a settlement guarantee — see the type
	/// docs.
	pub ok: bool,
	/// The swap's primary key, hex-encoded.
	pub payment_hash: String,
	/// Swap status immediately after initiation — normally `"InFlight"`. Poll the swap until it
	/// reaches `"Settled"` / `"Failed"` to learn the real outcome.
	pub status: String,
}

/// Request carrying an out-of-band offer string — used to accept an offer (taker side) or to
/// preview one without persisting it.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SwapStringRequest {
	pub swap_string: String,
}

/// A swap's full details, as returned by the create/accept/decode/list/get endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct SwapInfoDto {
	/// Stable execution identifier; accepted by execute and the swap lookup endpoint.
	pub execution_id: String,
	/// Bound quote, selected paths and all versioned execution plans, including policy snapshots.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub execution: Option<SwapExecution>,
	/// Hex-encoded 32-byte payment hash — the swap's primary key.
	pub payment_hash: String,
	/// "Maker" or "Taker".
	pub role: String,
	/// One of "Offered", "Accepted", "InFlight", "Forwarded", "Settled", "Failed".
	///
	/// `InFlight` means work was accepted for processing — including a taker enqueueing an
	/// intercepted HTLC — not that it succeeded. `Forwarded` is retained for legacy persisted
	/// records. Only `Settled` proves the circular payment completed.
	pub status: String,
	/// Counterparty node public key, hex-encoded.
	pub counterparty_node_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub channel_scid: u64,
	/// RGB contract id (RGB-native `contract:…` string). Feed this into contract-info / precision
	/// lookups (e.g. `GET /rgb/contracts`) to resolve the asset's divisibility before formatting
	/// `asset_amount`.
	pub contract_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_amount: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub btc_amount_msat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub btc_carrier_amount_msat: u64,
	/// If true: maker gives RGB, taker gives BTC. If false: maker gives BTC, taker gives RGB.
	pub maker_gives_rgb: bool,
	pub expiry_secs: u32,
	#[serde(with = "serde_u64_decimal_string")]
	pub created_at_unix_secs: u64,
	/// Whether the quote uses an explicitly supplied circular route.
	pub is_multihop: bool,
	/// Human-readable reason for the most recent failure (e.g. "RetriesExhausted"); `null` unless
	/// the swap has failed. Lets a client show *why* a swap failed without scraping logs.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub last_error: Option<String>,
}
