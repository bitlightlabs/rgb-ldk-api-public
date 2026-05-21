// Generated from rgb-ldk-node/src/http/dto/common.rs. Do not edit.

//! HTTP API request/response types (v1).
//!
//! These are kept in-tree to ensure `ldk-node` remains self-contained.

#![allow(missing_docs)]

use super::*;
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
	pub locked: bool,
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
	pub locked: bool,
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

