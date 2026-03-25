// Generated from rgb-ldk-node/src/http/dto.rs. Do not edit.

//! HTTP API request/response types (v1).
//!
//! These are kept in-tree to ensure `ldk-node` remains self-contained.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A single readiness/health sub-check.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthCheckDto {
	pub name: String,
	pub ok: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hint: Option<String>,
}

/// Generic OK response (optionally with sub-checks).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OkResponse {
	pub ok: bool,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub checks: Vec<HealthCheckDto>,
}

/// Version metadata.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VersionResponse {
	pub api_version: String,
	pub api_crate_version: String,
	pub core_crate_version: String,
}

/// Response containing the node's public key.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NodeIdResponse {
	pub node_id: String,
}

/// Response containing the node's listening addresses.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ListeningAddressesResponse {
	pub addresses: Vec<String>,
}

/// Response containing a newly-generated on-chain address.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WalletNewAddressResponse {
	pub address: String,
}

/// Node status response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StatusDto {
	/// Whether the node runtime is running.
	pub is_running: bool,
	/// Whether the node is currently listening for inbound peers.
	pub is_listening: bool,
	/// Current best block height as seen by the node.
	pub best_block_height: u32,
}

/// Locked daemon status (served by rgbldkd locked HTTP).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedStatusDto {
	pub ok: bool,
	pub locked: bool,
	pub running: bool,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub checks: Vec<HealthCheckDto>,
}

/// Main status response, which can represent either an unlocked node or a locked daemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MainStatusResponse {
	Unlocked(StatusDto),
	Locked(LockedStatusDto),
}

/// Control HTTP status response (served by rgbldkd control HTTP).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlStatusDto {
	pub ok: bool,
	pub locked: bool,
	pub running: bool,
	/// Whether a safe lock can be performed immediately without forcing a stop.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lockable: Option<bool>,
	/// Explainable sub-checks for safe-lock readiness when the daemon is running.
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub checks: Vec<HealthCheckDto>,
}

/// Request body for `POST /control/unlock`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlUnlockRequest {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub passphrase: Option<String>,
}

/// Request body for `POST /control/lock`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlLockRequest {
	pub yes: bool,
	/// If `true`, stop immediately even when safe-lock checks fail.
	#[serde(default)]
	pub force: bool,
}

/// Generic error response with optional guidance and sub-checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
	pub ok: bool,
	pub error: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hint: Option<String>,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub checks: Vec<HealthCheckDto>,
}

/// Version metadata for the authenticated rgbldkd control API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlVersionDto {
	pub ok: bool,
	pub protocol: String,
	pub daemon: String,
	pub daemon_version: String,
}

/// Wallet and channel balance overview (BTC + RGB).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BalancesDto {
	pub btc: BtcBalancesDto,
	pub rgb: RgbBalancesDto,
}

/// Bitcoin (BTC) balance overview.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BtcBalancesDto {
	/// Total confirmed on-chain balance in satoshis.
	#[serde(with = "serde_u64_decimal_string")]
	pub onchain_total_sats: u64,
	/// Spendable on-chain balance in satoshis.
	#[serde(with = "serde_u64_decimal_string")]
	pub onchain_spendable_sats: u64,
	/// Sum of sats reserved for anchor channels.
	#[serde(with = "serde_u64_decimal_string")]
	pub anchor_channels_reserve_sats: u64,
	/// Total claimable Lightning balance in satoshis.
	#[serde(with = "serde_u64_decimal_string")]
	pub lightning_total_sats: u64,
}

/// RGB balance overview, split by asset location.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbBalancesDto {
	/// L1 (on-chain) RGB balances by contract.
	pub l1: Vec<RgbL1BalanceDto>,
	/// L2 (Lightning channel) RGB balances by channel.
	pub l2: Vec<RgbL2BalanceDto>,
}

/// L1 RGB balance for a single contract.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbL1BalanceDto {
	/// Contract ID (string like `contract:...`).
	pub contract_id: String,
	/// Confirmed on-chain balance.
	#[serde(with = "serde_u64_decimal_string")]
	pub mined: u64,
	/// Unconfirmed (tentative) on-chain balance.
	#[serde(with = "serde_u64_decimal_string")]
	pub tentative: u64,
	/// Off-chain balance tracked by the RGB wallet.
	#[serde(with = "serde_u64_decimal_string")]
	pub offchain: u64,
	/// Archived balance (historical, not spendable).
	#[serde(with = "serde_u64_decimal_string")]
	pub archived: u64,
	/// Total of mined + tentative + offchain.
	#[serde(with = "serde_u64_decimal_string")]
	pub total: u64,
}

/// L2 RGB balance for a single channel.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbL2BalanceDto {
	/// Channel id (32-byte hex).
	pub channel_id: String,
	/// Contract ID (string like `contract:...`).
	pub contract_id: String,
	/// Local (our) RGB balance in this channel.
	#[serde(with = "serde_u64_decimal_string")]
	pub local_amount: u64,
	/// Remote (counterparty) RGB balance in this channel.
	#[serde(with = "serde_u64_decimal_string")]
	pub remote_amount: u64,
}

/// Peer details entry.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PeerDetailsDto {
	/// Peer node id in hex.
	pub node_id: String,
	/// Selected address for the peer.
	pub address: String,
	/// Whether peer is persisted to peer store.
	pub is_persisted: bool,
	/// Whether peer is currently connected.
	pub is_connected: bool,
}

/// Request to connect to a peer.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PeerConnectRequest {
	/// Peer node id in hex.
	pub node_id: String,
	/// Socket address, e.g. `lnd1:9735`.
	pub address: String,
	/// Whether to persist the peer to disk.
	#[serde(default)]
	pub persist: bool,
}

/// Request to disconnect a peer.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PeerDisconnectRequest {
	/// Peer node id in hex.
	pub node_id: String,
}

/// Payment kind-specific details (machine-friendly).
///
/// This is optional and may grow over time; clients should treat unknown fields as opaque.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentDetailsDto {
	/// Payment id (hex-encoded 32 bytes).
	pub id: String,
	/// Direction, either Inbound or Outbound.
	pub direction: String,
	/// Status, one of Pending, Succeeded, Failed.
	pub status: String,
	/// Amount in millisatoshis, if known.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub amount_msat: Option<u64>,
	/// Payment kind label.
	pub kind: String,
	/// Fee paid in millisatoshis, if known.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub fee_paid_msat: Option<u64>,
	/// Kind-specific details (when available).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub kind_details: Option<serde_json::Value>,
}

/// RGB asset balance in a channel.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbChannelBalanceDto {
	/// Contract ID (string like `contract:...`).
	pub contract_id: String,
	/// Local (our) RGB balance in this channel.
	#[serde(with = "serde_u64_decimal_string")]
	pub local_amount: u64,
	/// Remote (counterparty) RGB balance in this channel.
	#[serde(with = "serde_u64_decimal_string")]
	pub remote_amount: u64,
}

/// Channel details entry (extended for control-plane integrations).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelDetailsExtendedDto {
	/// Channel id (32-byte hex).
	pub channel_id: String,
	/// User channel id (hex-encoded 16 bytes BIG-ENDIAN).
	pub user_channel_id: String,
	/// Counterparty node id (hex pubkey).
	pub counterparty_node_id: String,
	/// Funding outpoint formatted as `txid:vout` if known.
	pub channel_point: Option<String>,
	/// Total channel capacity in satoshis.
	#[serde(with = "serde_u64_decimal_string")]
	pub channel_value_sats: u64,
	/// Sendable capacity in millisatoshis.
	#[serde(with = "serde_u64_decimal_string")]
	pub outbound_capacity_msat: u64,
	/// Receivable capacity in millisatoshis.
	#[serde(with = "serde_u64_decimal_string")]
	pub inbound_capacity_msat: u64,
	/// Estimated local (our) total balance in millisatoshis.
	///
	/// This includes our `unspendable_punishment_reserve` and therefore may be higher than
	/// `outbound_capacity_msat`.
	///
	/// Returns `None` for outbound channels until the counterparty accepts the channel (as LDK
	/// will report `unspendable_punishment_reserve = None`).
	#[serde(
		default,
		with = "serde_opt_u64_decimal_string",
		skip_serializing_if = "Option::is_none"
	)]
	pub local_balance_msat: Option<u64>,
	/// Estimated remote (counterparty) total balance in millisatoshis.
	///
	/// This is derived as `channel_value_sats * 1000 - local_balance_msat` and is therefore also
	/// `None` if `local_balance_msat` is unavailable.
	#[serde(
		default,
		with = "serde_opt_u64_decimal_string",
		skip_serializing_if = "Option::is_none"
	)]
	pub remote_balance_msat: Option<u64>,
	/// Local (our) unspendable punishment reserve in satoshis.
	#[serde(
		default,
		with = "serde_opt_u64_decimal_string",
		skip_serializing_if = "Option::is_none"
	)]
	pub local_unspendable_punishment_reserve_sats: Option<u64>,
	/// Remote (counterparty) unspendable punishment reserve in satoshis.
	#[serde(with = "serde_u64_decimal_string")]
	pub remote_unspendable_punishment_reserve_sats: u64,
	/// Whether channel is ready.
	pub is_channel_ready: bool,
	/// Whether channel is usable.
	pub is_usable: bool,
	/// Whether channel is announced.
	pub is_announced: bool,
	/// RGB asset balance in this channel, if any.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rgb_balance: Option<RgbChannelBalanceDto>,
}

/// Request to decode a BOLT11 invoice.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt11DecodeRequest {
	pub invoice: String,
}

/// Decoded BOLT11 invoice summary.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt11DecodeResponse {
	pub payment_hash: String,
	pub destination: String,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub amount_msat: Option<u64>,
	#[serde(with = "serde_u64_decimal_string")]
	pub expiry_secs: u64,
}

/// Request to create a BOLT11 invoice.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt11ReceiveRequest {
	#[serde(with = "serde_u64_decimal_string")]
	pub amount_msat: u64,
	pub description: String,
	pub expiry_secs: u32,
}

/// Request to create a variable-amount BOLT11 invoice.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt11ReceiveVarRequest {
	pub description: String,
	pub expiry_secs: u32,
}

/// Request to create a BOLT11 invoice for a specific payment hash (hold invoice).
///
/// The resulting invoice will NOT be auto-claimed. The receiver must manually
/// call `/bolt11/claim_for_hash` or `/bolt11/fail_for_hash` after receiving
/// the `PaymentClaimable` event.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt11ReceiveForHashRequest {
	#[serde(with = "serde_u64_decimal_string")]
	pub amount_msat: u64,
	pub description: String,
	pub expiry_secs: u32,
	/// Payment hash (hex-encoded 32 bytes).
	pub payment_hash: String,
}

/// Request to fail a held payment by its payment hash.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt11FailForHashRequest {
	/// Payment hash (hex-encoded 32 bytes).
	pub payment_hash: String,
}

/// Manually claim a held payment created via `/bolt11/receive_for_hash`.
///
/// Requires the original preimage that corresponds to the payment hash.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt11ClaimForHashRequest {
	/// Payment hash (hex-encoded 32 bytes).
	pub payment_hash: String,
	/// Payment preimage (hex-encoded 32 bytes).
	pub preimage: String,
	/// The claimable amount in millisatoshis.
	#[serde(with = "serde_u64_decimal_string")]
	pub claimable_amount_msat: u64,
}

/// Response containing a newly created BOLT11 invoice.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt11ReceiveResponse {
	pub invoice: String,
}

/// Request to pay a BOLT11 invoice.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt11SendRequest {
	pub invoice: String,
}

/// Request to pay a BOLT11 invoice using a specified amount.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt11SendUsingAmountRequest {
	pub invoice: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub amount_msat: u64,
}

/// Preferred BOLT11 payment endpoint: waits for completion and returns preimage.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt11PayRequest {
	pub invoice: String,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub amount_msat: Option<u64>,
}

/// Response of a completed BOLT11 payment.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt11PayResponse {
	pub payment_id: String,
	pub preimage: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub amount_sats: u64,
	pub destination: String,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub fee_paid_msat: Option<u64>,
}

/// Request to open a channel to a peer.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct OpenChannelRequest {
	pub node_id: String,
	pub address: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub channel_amount_sats: u64,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub push_to_counterparty_msat: Option<u64>,
	pub announce: Option<bool>,
	#[serde(default)]
	pub rgb: Option<RgbOpenChannelRequest>,
}

/// Response of a successful channel open request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OpenChannelResponse {
	pub user_channel_id: String,
}

/// Request to close or force-close a channel.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct CloseChannelRequest {
	pub user_channel_id: String,
	pub counterparty_node_id: String,
}

/// Response containing the id of a sent payment.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SendResponse {
	pub payment_id: String,
}

/// TLV record for spontaneous payments.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CustomTlvDto {
	#[serde(rename = "type")]
	#[serde(with = "serde_u64_decimal_string")]
	pub r#type: u64,
	pub value_hex: String,
}

/// Request to send a spontaneous (keysend) payment.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SpontaneousSendRequest {
	pub counterparty_node_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub amount_msat: u64,
	#[serde(default)]
	pub custom_tlvs: Vec<CustomTlvDto>,
}

/// Outpoint for channel pending events.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OutPointDto {
	pub txid: String,
	pub vout: u32,
}

/// Event DTO (subset; others are mapped to `Other`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", content = "data")]
pub enum EventDto {
	PaymentSuccessful {
		payment_id: Option<String>,
		#[serde(default, with = "serde_opt_u64_decimal_string")]
		fee_paid_msat: Option<u64>,
	},
	PaymentFailed {
		payment_id: Option<String>,
	},
	PaymentReceived {
		payment_id: Option<String>,
		payment_hash: String,
		#[serde(with = "serde_u64_decimal_string")]
		amount_msat: u64,
		#[serde(default)]
		custom_records: Vec<CustomTlvDto>,
		#[serde(skip_serializing_if = "Option::is_none")]
		rgb: Option<RgbPaymentContextDto>,
	},
	ChannelPending {
		funding_txo: OutPointDto,
	},
	ChannelReady {
		user_channel_id: String,
	},
	ChannelClosed {
		channel_id: String,
		user_channel_id: String,
		#[serde(skip_serializing_if = "Option::is_none")]
		counterparty_node_id: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		reason: Option<String>,
	},
	Other {
		kind: String,
	},
}

/// RGB context for Lightning payments/events.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbPaymentContextDto {
	/// Contract ID (string like `contract:...`).
	pub contract_id: String,
	/// Asset amount.
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_amount: u64,
	/// RGB payment direction (Inbound/Outbound).
	pub direction: String,
	/// Whether this payment is part of an atomic swap.
	pub is_swap: bool,
}

/// ---- BOLT12 (offers + refunds) ----

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt12OfferReceiveRequest {
	#[serde(with = "serde_u64_decimal_string")]
	pub amount_msat: u64,
	pub description: String,
	/// Seconds from now; if omitted, offer does not expire.
	#[serde(default)]
	pub expiry_secs: Option<u32>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub quantity: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt12OfferReceiveVarRequest {
	pub description: String,
	/// Seconds from now; if omitted, offer does not expire.
	#[serde(default)]
	pub expiry_secs: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt12OfferResponse {
	pub offer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt12OfferDecodeRequest {
	pub offer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt12OfferDecodeResponse {
	pub offer_id: String,
	#[serde(default)]
	pub signing_pubkey: Option<String>,
	#[serde(default)]
	pub description: Option<String>,
	#[serde(default)]
	pub issuer: Option<String>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub amount_msat: Option<u64>,
	/// Seconds since Unix epoch.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub absolute_expiry_unix_secs: Option<u64>,
	#[serde(default)]
	pub chain_hashes: Vec<String>,
	pub paths_count: usize,
	pub expects_quantity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt12OfferSendRequest {
	pub offer: String,
	/// Required for zero-amount offers; may be used to overpay fixed-amount offers.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub amount_msat: Option<u64>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub quantity: Option<u64>,
	#[serde(default)]
	pub payer_note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt12RefundInitiateRequest {
	#[serde(with = "serde_u64_decimal_string")]
	pub amount_msat: u64,
	pub expiry_secs: u32,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub quantity: Option<u64>,
	#[serde(default)]
	pub payer_note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt12RefundInitiateResponse {
	pub refund: String,
	pub payment_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt12RefundDecodeRequest {
	pub refund: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt12RefundDecodeResponse {
	pub description: String,
	#[serde(default)]
	pub issuer: Option<String>,
	#[serde(with = "serde_u64_decimal_string")]
	pub amount_msat: u64,
	/// Seconds since Unix epoch.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub absolute_expiry_unix_secs: Option<u64>,
	pub chain_hash: String,
	pub payer_signing_pubkey: String,
	#[serde(default)]
	pub payer_note: Option<String>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub quantity: Option<u64>,
	pub paths_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt12RefundRequestPaymentRequest {
	pub refund: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bolt12RefundRequestPaymentResponse {
	/// Informational only (bech32-encoded BOLT12 invoice, HRP `lni`).
	pub invoice: String,
	/// Informational only (raw TLV bytes hex-encoded).
	pub invoice_hex: String,
	pub payment_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentWaitRequest {
	/// Default: 60 seconds.
	#[serde(default)]
	pub timeout_secs: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentWaitResponse {
	pub ok: bool,
	pub payment: PaymentDetailsDto,
	#[serde(default)]
	pub checks: Vec<HealthCheckDto>,
}

// ---- RGB endpoints ----

// /// Response for `GET /rgb/utxos`.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct RgbUtxoDto {
// 	pub outpoint: String,
// }

// /// Request for `POST /rgb/prepare_send`.
// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct RgbPrepareSendRequest {
// 	/// BOLT11 invoice string.
// 	pub invoice: String,
// 	/// Asset ID (hex-encoded bytes).
// 	pub asset_id: String,
// 	/// Amount of the asset to send.
// 	pub asset_amount: u64,
// }

/// ---- RGB (Lightning + on-chain) ----

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOpenChannelRequest {
	/// RGB contract ID.
	pub contract_id: String,
	/// RGB asset amount to commit into the channel.
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_amount: u64,
	/// RGB color context data (e.g. a consignment endpoint like `file://...`).
	pub color_context_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbNewAddressResponse {
	pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbContractDto {
	pub contract_id: String,
	/// Human-friendly asset name (if known).
	pub name: Option<String>,
	/// Short asset ticker (if known).
	pub ticker: Option<String>,
	/// Asset precision (if known).
	pub precision: Option<u8>,
	/// Total issued supply (if known).
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub issued_supply: Option<u64>,
	/// Optional human-readable details/description (if present in global state).
	pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbContractsResponse {
	pub contracts: Vec<RgbContractDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RgbIssuersResponse {
	pub issuers: Vec<String>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub invalid_issuers: Vec<RgbInvalidIssuerDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RgbInvalidIssuerDto {
	pub name: String,
	pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RgbIssuersImportResponse {
	pub ok: bool,
	pub issuer_name: String,
	#[serde(default)]
	pub checks: Vec<HealthCheckDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbContractsImportResponse {
	pub ok: bool,
	pub contract_id: String,
	/// Storage key used by the RGB wallet to read the imported consignment.
	pub consignment_key: String,
	#[serde(default)]
	pub checks: Vec<HealthCheckDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbContractsIssueRequest {
	/// Issuer name from the local issuer registry.
	pub issuer_name: String,
	/// RGB contract name.
	pub contract_name: String,
	/// Optional ticker (defaults to `contract_name`).
	#[serde(default)]
	pub ticker: Option<String>,
	/// Optional precision override.
	#[serde(default)]
	pub precision: Option<u8>,
	/// Total issued supply for the contract.
	#[serde(with = "serde_u64_decimal_string")]
	pub issued_supply: u64,
	/// Optional issuance UTXO outpoint (`txid:vout`). If omitted, a wallet UTXO is auto-selected.
	#[serde(default)]
	pub utxo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbContractsIssueResponse {
	pub ok: bool,
	pub contract_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub issued_supply: u64,
	#[serde(default)]
	pub checks: Vec<HealthCheckDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbContractsExportRequest {
	pub contract_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbContractsExportResponse {
	pub ok: bool,
	pub contract_id: String,
	/// Storage key used by the RGB wallet to read the exported consignment.
	pub consignment_key: String,
	#[serde(default)]
	pub checks: Vec<HealthCheckDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbContractBalanceDto {
	#[serde(with = "serde_u64_decimal_string")]
	pub mined: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub tentative: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub offchain: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub archived: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbContractBalanceResponse {
	pub contract_id: String,
	pub balance: RgbContractBalanceDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbContractKnownResponse {
	pub contract_id: String,
	pub known: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnInvoiceCreateRequest {
	/// RGB contract ID.
	pub contract_id: String,
	/// RGB asset amount.
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_amount: u64,
	/// Human-readable description.
	pub description: String,
	/// Invoice expiry in seconds. If omitted, the BOLT11 default expiry is used.
	#[serde(default)]
	pub expiry_secs: Option<u32>,
	/// BTC carrier amount in msat embedded into the BOLT11 invoice.
	#[serde(with = "serde_u64_decimal_string")]
	pub btc_carrier_amount_msat: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnInvoiceCreateForHashRequest {
	/// RGB contract ID.
	pub contract_id: String,
	/// RGB asset amount.
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_amount: u64,
	/// Payment hash (64 hex chars) provided by caller.
	pub payment_hash: String,
	/// Human-readable description.
	pub description: String,
	/// Invoice expiry in seconds. If omitted, the BOLT11 default expiry is used.
	#[serde(default)]
	pub expiry_secs: Option<u32>,
	/// BTC carrier amount in msat embedded into the BOLT11 invoice.
	#[serde(with = "serde_u64_decimal_string")]
	pub btc_carrier_amount_msat: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnInvoiceResponse {
	pub invoice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnInvoiceDecodeRequest {
	pub invoice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnInvoiceDecodeResponse {
	pub payment_hash: String,
	pub destination: String,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub carrier_amount_msat: Option<u64>,
	#[serde(with = "serde_u64_decimal_string")]
	pub expiry_secs: u64,
	#[serde(default)]
	pub contract_id: Option<String>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub asset_amount: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnPayRequest {
	pub invoice: String,
	/// Optional explicit contract ID for invoices that do not embed RGB fields.
	#[serde(default)]
	pub contract_id: Option<String>,
	/// Optional explicit asset amount for invoices that do not embed RGB fields.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub asset_amount: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainInvoiceCreateRequest {
	pub contract_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub amount: u64,
	/// Expiration time for the invoice in seconds.
	///
	/// If omitted, the server will apply a reasonable default.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub expiry_secs: Option<u64>,
	/// Controls whether the invoice uses a witness-output (`wout:`) beneficiary (`true`) or a
	/// blinded auth-token (`at:`) beneficiary (`false`).
	///
	/// This field is intentionally required at the API level to avoid ambiguous requests where
	/// callers accidentally rely on defaults and create an invoice type they didn't intend.
	#[serde(default, alias = "useWitnessUtxo")]
	pub use_witness_utxo: Option<bool>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub nonce: Option<u64>,
	/// Outpoint used for blinding when `use_witness_utxo=false`.
	///
	/// If omitted and `use_witness_utxo=false`, the node will auto-select a wallet UTXO.
	#[serde(default)]
	pub blinding_utxo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainInvoiceResponse {
	pub invoice: String,
	/// Outpoint actually used for blinding when creating a blinded invoice (`use_witness_utxo=false`).
	#[serde(default)]
	pub blinding_utxo_used: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainInvoiceDecodeRequest {
	pub invoice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainInvoiceDecodeResponse {
	pub contract_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub amount: u64,
	pub beneficiary: String,
	pub use_witness_utxo: bool,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub expiry_unix_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainSendRequest {
	pub invoice: String,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub sats_for_fee_and_outputs: Option<u64>,
	pub fee_rate_sats_per_vb: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainSendResponse {
	pub txid: String,
	pub consignment_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainReceiveRequest {
	pub consignment_key: String,
	/// Optional invoice id (hex-encoded 32 bytes, i.e. `sha256(invoice_str)`).
	///
	/// Callers must provide either `payment_id` or `invoice` so the node can correlate the
	/// consignment to a single stable payment record.
	#[serde(default)]
	pub payment_id: Option<String>,
	/// Optional invoice string this consignment is expected to satisfy.
	///
	/// Callers must provide either `payment_id` or `invoice` so the node can correlate the
	/// consignment to a single stable payment record.
	#[serde(default)]
	pub invoice: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainReceiveResponse {
	pub contract_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainPaymentsResponse {
	pub payments: Vec<RgbOnchainPaymentDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainPaymentDto {
	pub id: String,
	/// Constant `onchain` (RGB L1 non-channel payment lifecycle).
	pub kind: String,
	/// One of: pending | succeeded | failed | expired
	pub status: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub created_at_unix_secs: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub latest_update_timestamp: u64,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub expires_at_unix_secs: Option<u64>,

	// Common payload fields (vary by kind)
	#[serde(default)]
	pub invoice: Option<String>,
	#[serde(default)]
	pub contract_id: Option<String>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub amount: Option<u64>,
	#[serde(default)]
	pub txid: Option<String>,
	#[serde(default)]
	pub consignment_key: Option<String>,
	#[serde(default)]
	pub consignment_download_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosResponse {
	/// RGB wallet UTXOs (outpoints formatted as `txid:vout`).
	pub utxos: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosSummaryResponse {
	pub utxos: Vec<RgbUtxoSummaryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxoAssetAllocationDto {
	/// Contract ID (string like `contract:...`).
	pub contract_id: String,
	/// Amount anchored on this UTXO for the given contract.
	#[serde(with = "serde_u64_decimal_string")]
	pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxoSummaryDto {
	/// Outpoint formatted as `txid:vout`.
	pub outpoint: String,
	/// BTC value of the output (sats), if available from the configured chain source.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub value_sats: Option<u64>,
	/// Confirmation block height, if known (confirmed only).
	#[serde(default)]
	pub confirmed_height: Option<u32>,
	/// Whether this outpoint is currently reserved by the node.
	pub reserved: bool,
	/// Reservation expiry (unix seconds), if reserved.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub reserved_until_unix_secs: Option<u64>,
	/// RGB assets anchored on this UTXO.
	///
	/// Each element contains only `contract_id` and `amount`.
	#[serde(default)]
	pub assets: Vec<RgbUtxoAssetAllocationDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosReserveRequest {
	/// Optional explicit outpoint to reserve (`txid:vout`).
	#[serde(default)]
	pub outpoint: Option<String>,
	/// Optional reservation TTL in seconds (default: 300).
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub ttl_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosReserveResponse {
	pub reservation_id: String,
	pub outpoint: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub reserved_until_unix_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosReleaseRequest {
	/// Release by reservation id (preferred).
	#[serde(default)]
	pub reservation_id: Option<String>,
	/// Release by outpoint (`txid:vout`).
	#[serde(default)]
	pub outpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosReleaseResponse {
	pub released: bool,
}

mod serde_u64_decimal_string {
	use serde::de::Error;
	use serde::{Deserialize, Deserializer, Serializer};

	pub fn serialize<S>(v: &u64, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		s.serialize_str(&v.to_string())
	}

	pub fn deserialize<'de, D>(d: D) -> Result<u64, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(d)?;
		s.parse::<u64>().map_err(D::Error::custom)
	}
}

mod serde_opt_u64_decimal_string {
	use serde::de::Error;
	use serde::{Deserialize, Deserializer, Serializer};

	pub fn serialize<S>(v: &Option<u64>, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match v {
			Some(n) => s.serialize_some(&n.to_string()),
			None => s.serialize_none(),
		}
	}

	pub fn deserialize<'de, D>(d: D) -> Result<Option<u64>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let opt = Option::<String>::deserialize(d)?;
		match opt {
			Some(s) => Ok(Some(s.parse::<u64>().map_err(D::Error::custom)?)),
			None => Ok(None),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rgb_onchain_invoice_create_request_allows_missing_blinding_utxo() {
		let req: RgbOnchainInvoiceCreateRequest = serde_json::from_value(serde_json::json!({
			"contract_id": "contract:dummy",
			"amount": "1",
			"use_witness_utxo": true,
		}))
		.expect("valid request should deserialize");
		assert!(req.blinding_utxo.is_none());

		let req: RgbOnchainInvoiceCreateRequest = serde_json::from_value(serde_json::json!({
			"contract_id": "contract:dummy",
			"amount": "1",
			"use_witness_utxo": false,
		}))
		.expect("valid request should deserialize");
		assert!(req.blinding_utxo.is_none());
	}
}

