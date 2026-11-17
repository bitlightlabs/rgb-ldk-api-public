// Generated from rgb-ldk-node/crates/node-http/src/dto/lsps1.rs. Do not edit.

//! LSPS1 channel-order DTOs: client-side ordering plus the operator surface of the
//! node's own LSPS1 service (pricing + order ledger).
//!
//! Wire conventions match the rest of the HTTP API: `u64` amounts and unix-second
//! timestamps are decimal strings.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::*;

// --- client side (this node buys a channel from its configured LSP) ---

/// Adds an LSP, or updates the entry with the same pubkey (`POST /lsps`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct LspAddRequest {
	/// LSP node pubkey, hex-encoded.
	pub pubkey: String,
	/// LSP socket address, e.g. `203.0.113.5:9735`.
	pub address: String,
	/// Access token sent to the LSP, if it requires one.
	#[serde(default)]
	pub token: Option<String>,
	/// Accept 0conf channels from this LSP: they are usable before the funding
	/// confirms, trusting the LSP not to double-spend it. Required for JIT channels.
	pub trust_peer_0conf: bool,
	/// Accept anchor channels from this LSP without keeping an on-chain reserve for
	/// force-close fees (default 25,000 sats per channel). A force close then relies
	/// on the LSP to get the transaction confirmed. Needed by clients with no on-chain BTC.
	pub trust_peer_no_reserve: bool,
}

/// One LSP in this node's LSP list (`GET /lsps`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LspDto {
	pub pubkey: String,
	pub address: String,
	pub token: Option<String>,
	pub trust_peer_0conf: bool,
	pub trust_peer_no_reserve: bool,
	/// LSPS protocol numbers from LSPS0 (`1` = LSPS1, `2` = LSPS2); `null` until a
	/// probe succeeds. Such an LSP is probed again when it is used.
	pub protocols: Option<Vec<u16>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LspListResponse {
	pub lsps: Vec<LspDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps1SupportedOptionsDto {
	pub min_required_channel_confirmations: u16,
	pub min_funding_confirms_within_blocks: u16,
	pub supports_zero_channel_reserve: bool,
	pub max_channel_expiry_blocks: u32,
	#[serde(with = "serde_u64_decimal_string")]
	pub min_initial_client_balance_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub max_initial_client_balance_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub min_initial_lsp_balance_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub max_initial_lsp_balance_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub min_channel_balance_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub max_channel_balance_sat: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1RgbAssetOfferDto {
	pub asset_id: String,
	pub ticker: String,
	pub precision: u8,
	/// Sale price per whole asset unit, in satoshi (balances are in smallest units).
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_unit_price_sat: u64,
	/// Yearly lease rate as ppm of the leased asset value.
	pub asset_rent_ppm_per_year: u32,
	#[serde(with = "serde_u64_decimal_string")]
	pub min_lsp_asset_balance: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub max_lsp_asset_balance: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub max_client_asset_balance: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1RgbOfferingDto {
	pub rgb_assets: Vec<Lsps1RgbAssetOfferDto>,
}

/// The LSP's fee parameters, shared by BTC and RGB orders (`GET /lsps1/info`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1LspPricingDto {
	/// Yearly lease rate for BTC capacity as ppm of the leased amount.
	pub btc_capacity_ppm_per_year: u32,
	/// Onchain cost component charged on every order; `null` = old-version LSP.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub onchain_cost_sat: Option<u64>,
	/// Minimum total fee per order; `null` = old-version LSP.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub min_fee_sat: Option<u64>,
}

/// One LSP's offering.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1InfoResponse {
	pub supported_options: Lsps1SupportedOptionsDto,
	/// Fee parameters; `null` when the LSP does not speak our extension (pricing
	/// rides on `lsps1.rgb_get_info`, so a foreign standard LSP advertises none).
	pub pricing: Option<Lsps1LspPricingDto>,
	/// `null` when the LSP does not speak the RGB extension at all. An LSP that does
	/// but currently offers no assets returns `rgb_assets: []` — check the roster,
	/// not just null-ness, to decide whether RGB channels can be ordered.
	pub rgb: Option<Lsps1RgbOfferingDto>,
}

/// One LSP's answer in `GET /lsps1/info`: either `info` or `error` is set.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1InfoEntryDto {
	pub lsp_node_id: String,
	pub info: Option<Lsps1InfoResponse>,
	/// Same shape as an error response body.
	#[schema(value_type = Option<Object>)]
	pub error: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1InfoListResponse {
	pub lsps: Vec<Lsps1InfoEntryDto>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Lsps1InfoQuery {
	/// Query only this LSP (hex pubkey); all LSPS1 LSPs when omitted.
	#[serde(default)]
	pub lsp_node_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps1OrderCreateRequest {
	/// The LSP to order from (hex pubkey); must be in `GET /lsps`.
	pub lsp_node_id: String,
	/// Sats the LSP provides on its side (the inbound liquidity being bought).
	#[serde(with = "serde_u64_decimal_string")]
	pub lsp_balance_sat: u64,
	/// Sats on this node's side of the new channel, paid on top of the fee. 0 when omitted. Must be
	/// between the LSP's `min_initial_client_balance_sat` and `max_initial_client_balance_sat` (see
	/// `supported_options`). So it can only be omitted when the LSP's minimum is 0.
	#[serde(default, with = "serde_u64_decimal_string")]
	pub client_balance_sat: u64,
	/// Channel lease duration in blocks.
	pub channel_expiry_blocks: u32,
	#[serde(default)]
	pub announce_channel: bool,
	/// Optional id of this request, chosen by the caller. 1 to 64 printable ASCII characters. It
	/// makes the request safe to repeat.
	///
	/// Same id, same parameters: returns the first order with its current state. No new order is
	/// placed. A request that arrives while the first is still running waits for it.
	///
	/// Same id, different parameters: 409 `idempotency_key_reused`.
	///
	/// An earlier request with this id was sent and got no answer (timeout, restart): 409
	/// `outcome_unknown`. The id is not sent again. Retry later, or use a new id to place a new
	/// order.
	///
	/// The id is kept until one day after the order's payment window ended. It is returned with the
	/// order. It is not sent to the LSP.
	#[serde(default)]
	pub client_request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps1RgbOrderCreateRequest {
	/// The LSP to order from (hex pubkey); must be in `GET /lsps`.
	pub lsp_node_id: String,
	/// RGB contract id of the ordered asset.
	pub asset_id: String,
	/// Asset units the LSP provides on its side (leased inbound asset liquidity).
	#[serde(with = "serde_u64_decimal_string")]
	pub lsp_asset_balance: u64,
	/// Asset units on this node's side of the new channel. Buying assets with the channel is not
	/// supported yet: omit it or send 0. Any other value is rejected.
	#[serde(default, with = "serde_u64_decimal_string")]
	pub client_asset_balance: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub lsp_balance_sat: u64,
	/// Sats on this node's side of the new channel, paid on top of the fee. 0 when omitted. Must be
	/// between the LSP's `min_initial_client_balance_sat` and `max_initial_client_balance_sat` (see
	/// `supported_options`). So it can only be omitted when the LSP's minimum is 0.
	#[serde(default, with = "serde_u64_decimal_string")]
	pub client_balance_sat: u64,
	pub channel_expiry_blocks: u32,
	#[serde(default)]
	pub announce_channel: bool,
	/// Optional id of this request; same meaning as on `POST /lsps1/order`.
	#[serde(default)]
	pub client_request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1OrderParamsDto {
	#[serde(with = "serde_u64_decimal_string")]
	pub lsp_balance_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub client_balance_sat: u64,
	pub required_channel_confirmations: u16,
	pub funding_confirms_within_blocks: u16,
	pub channel_expiry_blocks: u32,
	pub announce_channel: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1Bolt11PaymentDto {
	/// `expect_payment`, `hold`, `paid`, or `refunded`.
	pub state: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub expires_at_unix_secs: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub fee_total_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub order_total_sat: u64,
	pub invoice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1OnchainPaymentDto {
	/// `expect_payment`, `hold`, `paid`, or `refunded`.
	pub state: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub expires_at_unix_secs: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub fee_total_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub order_total_sat: u64,
	/// Per-order deposit address.
	pub address: String,
	pub min_onchain_payment_confirmations: Option<u16>,
	pub refund_onchain_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1PaymentOptionsDto {
	pub bolt11: Option<Lsps1Bolt11PaymentDto>,
	pub onchain: Option<Lsps1OnchainPaymentDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1ChannelInfoDto {
	#[serde(with = "serde_u64_decimal_string")]
	pub funded_at_unix_secs: u64,
	/// `txid:vout` of the channel funding output.
	pub funding_outpoint: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub expires_at_unix_secs: u64,
}

/// Itemization of `fee_total_sat`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1RgbFeeBreakdownDto {
	#[serde(with = "serde_u64_decimal_string")]
	pub onchain_cost_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub btc_rent_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_rent_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_sale_sat: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1RgbOrderDetailsDto {
	pub asset_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub lsp_asset_balance: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub client_asset_balance: u64,
	pub fee_breakdown: Lsps1RgbFeeBreakdownDto,
	/// `txid:vout` of the funding output carrying the RGB allocation, once opened.
	pub asset_funding_outpoint: Option<String>,
}

/// A placed order as reported by the LSP (`POST /lsps1/order`, `POST /lsps1/rgb_order`,
/// `GET /lsps1/order/{order_id}`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1OrderResponse {
	pub order_id: String,
	/// The `client_request_id` of the order. Present until one day after the payment window ended.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub client_request_id: Option<String>,
	pub order: Lsps1OrderParamsDto,
	pub payment: Lsps1PaymentOptionsDto,
	pub channel: Option<Lsps1ChannelInfoDto>,
	/// RGB section; `null` for plain BTC orders.
	pub rgb: Option<Lsps1RgbOrderDetailsDto>,
}

// --- operator side (this node *is* the LSP) ---

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps1AssetPricingDto {
	pub asset_id: String,
	pub ticker: String,
	pub precision: u8,
	/// Sale price per whole asset unit, in satoshi (balances are in smallest units).
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_unit_price_sat: u64,
	pub asset_rent_ppm_per_year: u32,
	#[serde(with = "serde_u64_decimal_string")]
	pub min_lsp_asset_balance: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub max_lsp_asset_balance: u64,
	/// Must be 0 for now: client-side asset buy-out is not offered yet.
	#[serde(with = "serde_u64_decimal_string")]
	pub max_client_asset_balance: u64,
	/// Consignment endpoint used when opening channels of this asset.
	pub color_context: String,
}

/// Fee parameters of this node's own LSPS1 service (operator side).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps1PricingCoreDto {
	pub btc_capacity_ppm_per_year: u32,
	#[serde(with = "serde_u64_decimal_string")]
	pub onchain_cost_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub min_fee_sat: u64,
}

/// Operator pricing config (`GET`/`PUT /lsps1/pricing`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps1PricingDto {
	pub pricing: Lsps1PricingCoreDto,
	pub assets: Vec<Lsps1AssetPricingDto>,
}

/// Service behavior knobs of this node's own LSPS1 service.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps1ServiceBehaviorDto {
	/// Token clients must send with orders; `null` disables the check.
	#[serde(default)]
	pub require_token: Option<String>,
	/// Payment window per order (bolt11 invoice expiry; also bounds the onchain option).
	pub bolt11_invoice_expiry_secs: u32,
	/// Confirmations before an onchain deposit counts as paid. 0 is raised to 1;
	/// values above 6 have no practical effect (bLIP-51 forces acceptance at 6).
	pub min_onchain_payment_confirmations: u16,
	/// Cooperatively close leased channels once lease + grace expired.
	pub auto_close_expired_channels: bool,
	/// Blocks of grace before an expired lease is closed.
	pub channel_expiry_grace_blocks: u32,
	/// How long after payment expiry a late deposit is still auto-refunded.
	#[serde(with = "serde_u64_decimal_string")]
	pub late_deposit_refund_window_secs: u64,
	/// Blocks an onchain-paid order keeps retrying the channel open while the client is
	/// unreachable before giving up and refunding (bolt11 orders use the HTLC claim
	/// deadline). Defaults to ~1 day if omitted.
	#[serde(default = "default_onchain_open_timeout_blocks")]
	pub onchain_open_timeout_blocks: u32,
	/// A leased or JIT channel that is still open this many blocks after it was due to close is
	/// force closed. This covers an offline client and a cooperative close that keeps failing. 0
	/// (default) = off, otherwise at least 144. A force close costs this node the onchain fee and
	/// locks its funds for the client's `to_self_delay`. The client's funds are not at risk.
	#[serde(default)]
	pub force_close_after_blocks: u32,
	/// `false` pauses the service: `create_order` is refused with code 1. Existing orders are still
	/// served.
	pub accept_new_orders: bool,
	/// Maximum unpaid orders per client. At least 1. Default 5.
	#[serde(default = "default_max_unpaid_orders_per_peer")]
	pub max_unpaid_orders_per_peer: u32,
	/// Maximum unpaid orders of all clients together. At least 1. Default 100.
	#[serde(default = "default_max_unpaid_orders_total")]
	pub max_unpaid_orders_total: u32,
}

fn default_max_unpaid_orders_per_peer() -> u32 {
	5
}
fn default_max_unpaid_orders_total() -> u32 {
	100
}

fn default_onchain_open_timeout_blocks() -> u32 {
	144
}

/// Runtime service config (`GET`/`PUT /lsps1/options`). Changes apply to new orders
/// only; accepted orders keep the parameters they were created with.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps1OptionsDto {
	/// Order limits advertised via `lsps1.get_info` and enforced on `create_order`.
	pub supported_options: Lsps1SupportedOptionsDto,
	pub service: Lsps1ServiceBehaviorDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1ServiceRgbOrderDto {
	pub asset_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub lsp_asset_balance: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub client_asset_balance: u64,
	pub fee_breakdown: Lsps1RgbFeeBreakdownDto,
}

/// One served order in the LSPS1 fulfillment ledger (`GET /lsps1/orders`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1ServiceOrderDto {
	/// Whether this order counts toward `max_unpaid_orders_per_peer` / `max_unpaid_orders_total`.
	/// True while it is `created` and `expect_payment`.
	pub counts_toward_unpaid_quota: bool,
	pub order_id: String,
	pub counterparty_node_id: String,
	/// `created`, `completed`, or `failed`.
	pub order_state: String,
	/// `expect_payment`, `hold`, `paid`, or `refunded`.
	pub payment_state: String,
	/// `bolt11` or `onchain`; `null` while unpaid.
	pub paid_via: Option<String>,
	#[serde(with = "serde_u64_decimal_string")]
	pub lsp_balance_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub client_balance_sat: u64,
	pub channel_expiry_blocks: u32,
	pub announce_channel: bool,
	#[serde(with = "serde_u64_decimal_string")]
	pub fee_total_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub order_total_sat: u64,
	pub onchain_address: Option<String>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub onchain_paid_sat: Option<u64>,
	pub refund_onchain_address: Option<String>,
	pub refund_txid: Option<String>,
	pub fulfill_retry_count: u32,
	#[serde(with = "serde_u64_decimal_string")]
	pub created_at_unix_secs: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub payment_expires_at_unix_secs: u64,
	/// `txid:vout` of the channel funding output, once published.
	pub funding_outpoint: Option<String>,
	/// Chain height when the funding tx was published (lease expiry baseline).
	pub funded_at_height: Option<u32>,
	/// Set when the expired lease was closed.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub channel_closed_at_unix_secs: Option<u64>,
	/// How this node closed the channel: `cooperative`, or `force` (see
	/// `force_close_after_blocks`).
	#[serde(default)]
	pub close_kind: Option<String>,
	/// Failed cooperative close attempts since the lease was due. An offline client is not counted.
	/// Kept in memory only, so reset by a restart.
	#[serde(default)]
	pub close_attempts: u32,
	#[serde(default)]
	pub last_close_error: Option<String>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub first_close_attempt_at_unix_secs: Option<u64>,
	/// When the next close attempt is made. The delay starts at 5 s and doubles up to one hour.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub next_close_attempt_at_unix_secs: Option<u64>,
	/// On a failed order: `expired`, `cancelled`, `underpaid` or `fulfill_failed`.
	#[serde(default)]
	pub failure_reason: Option<String>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub cancelled_at_unix_secs: Option<u64>,
	/// The note given when the order was cancelled.
	#[serde(default)]
	pub cancel_reason: Option<String>,
	pub rgb: Option<Lsps1ServiceRgbOrderDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1ServiceOrdersResponse {
	pub orders: Vec<Lsps1ServiceOrderDto>,
}

/// Optional body of `POST /lsps1/orders/{order_id}/cancel`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps1OrderCancelRequest {
	/// Operator note stored with the order; at most 128 characters.
	#[serde(default)]
	pub reason: Option<String>,
}

/// `POST /lsps1/orders/cancel`: cancel all unpaid orders of one client.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps1OrdersCancelRequest {
	/// The client's node id (hex pubkey).
	pub counterparty_node_id: String,
	/// Operator note stored with each order; at most 128 characters.
	#[serde(default)]
	pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1OrderCancelSkippedDto {
	pub order_id: String,
	pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1OrdersCancelResponse {
	pub cancelled: Vec<Lsps1ServiceOrderDto>,
	/// Orders that were skipped, for example because a deposit was already seen.
	pub skipped: Vec<Lsps1OrderCancelSkippedDto>,
}
