// Generated from rgb-ldk-node/crates/node-http/src/dto/swap.rs. Do not edit.

//! Swap (BTC <-> RGB) request/response DTOs, mirroring `Node::swap()` (`rgb_ldk_node::Swap`).

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::serde_u64_decimal_string;

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
/// Unlike [`SwapCreateOfferRequest`], the caller supplies the full circular route explicitly:
/// `rgb_path` (maker -> ... -> taker) and `btc_path` (taker -> ... -> maker). The first hop of
/// `btc_path` is marked as the swap-intercept hop on execution. These names describe the
/// historical route halves, not the channel type: with `maker_gives_rgb=false`, `rgb_path`
/// carries the BTC counter-value and `btc_path` carries RGB. In a direct one-hop sell,
/// `btc_path.channel_scid` must therefore identify the RGB channel.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SwapCreateMultihopOfferRequest {
	pub rgb_path: Vec<SwapHopDto>,
	pub btc_path: Vec<SwapHopDto>,
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
	/// Out-of-band offer string (`rgb-swap:v1:...` or `rgb-swap:v2:...`) to hand to the
	/// counterparty (e.g. via QR code or chat).
	pub swap_string: String,
	/// The swap's primary key, hex-encoded.
	pub payment_hash: String,
	/// The freshly created swap's details.
	pub info: SwapInfoDto,
}

/// Request to execute a previously created offer (maker side). Provide either `swap_string`
/// (required for multi-hop offers, and preferred in general) or `payment_hash` (single-hop only).
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SwapExecuteRequest {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub swap_string: Option<String>,
	/// Hex-encoded 32-byte payment hash.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub payment_hash: Option<String>,
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
	/// True if this is a multi-hop swap. `POST /swap/execute` cannot execute these by
	/// `payment_hash` alone — pass the original `swap_string` instead.
	pub is_multihop: bool,
	/// Human-readable reason for the most recent failure (e.g. "RetriesExhausted"); `null` unless
	/// the swap has failed. Lets a client show *why* a swap failed without scraping logs.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub last_error: Option<String>,
}
