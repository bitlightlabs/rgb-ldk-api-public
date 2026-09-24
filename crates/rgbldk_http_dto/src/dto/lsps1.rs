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

/// The LSP this node buys channels from (`GET`/`PUT /lsps1/lsp`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1LspConfigDto {
	/// LSP node pubkey, hex-encoded.
	pub pubkey: String,
	/// LSP socket address, e.g. `203.0.113.5:9735`.
	pub address: String,
	/// Access token sent with orders, if the LSP requires one.
	#[serde(default)]
	pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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

/// The configured LSP's offering (`GET /lsps1/info`).
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1OrderCreateRequest {
	/// Sats the LSP provides on its side (the inbound liquidity being bought).
	#[serde(with = "serde_u64_decimal_string")]
	pub lsp_balance_sat: u64,
	/// Sats pushed to this node's side (paid on top of the fee). Default 0.
	#[serde(default, with = "serde_u64_decimal_string")]
	pub client_balance_sat: u64,
	/// Channel lease duration in blocks.
	pub channel_expiry_blocks: u32,
	#[serde(default)]
	pub announce_channel: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1RgbOrderCreateRequest {
	/// RGB contract id of the ordered asset.
	pub asset_id: String,
	/// Asset units the LSP provides on its side (leased inbound asset liquidity).
	#[serde(with = "serde_u64_decimal_string")]
	pub lsp_asset_balance: u64,
	/// Asset units pushed to this node's side (buy-out; LSPs may not offer it). Default 0.
	#[serde(default, with = "serde_u64_decimal_string")]
	pub client_asset_balance: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub lsp_balance_sat: u64,
	#[serde(default, with = "serde_u64_decimal_string")]
	pub client_balance_sat: u64,
	pub channel_expiry_blocks: u32,
	#[serde(default)]
	pub announce_channel: bool,
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
	pub order: Lsps1OrderParamsDto,
	pub payment: Lsps1PaymentOptionsDto,
	pub channel: Option<Lsps1ChannelInfoDto>,
	/// RGB section; `null` for plain BTC orders.
	pub rgb: Option<Lsps1RgbOrderDetailsDto>,
}

// --- operator side (this node *is* the LSP) ---

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
pub struct Lsps1PricingCoreDto {
	pub btc_capacity_ppm_per_year: u32,
	#[serde(with = "serde_u64_decimal_string")]
	pub onchain_cost_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub min_fee_sat: u64,
}

/// Operator pricing config (`GET`/`PUT /lsps1/pricing`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1PricingDto {
	pub pricing: Lsps1PricingCoreDto,
	pub assets: Vec<Lsps1AssetPricingDto>,
}

/// Service behavior knobs of this node's own LSPS1 service.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1ServiceBehaviorDto {
	/// Token clients must send with orders; `null` disables the check.
	#[serde(default)]
	pub require_token: Option<String>,
	/// Payment window per order (bolt11 invoice expiry; also bounds the onchain option).
	pub bolt11_invoice_expiry_secs: u32,
	/// Confirmations before an onchain deposit counts as paid. 0 is raised to 1;
	/// values above 6 have no practical effect (bLIP-51 forces acceptance at 6).
	pub min_onchain_payment_confirmations: u16,
	/// Channel-open retries before an order fails.
	pub max_fulfill_retries: u32,
	/// Cooperatively close leased channels once lease + grace expired.
	pub auto_close_expired_channels: bool,
	/// Blocks of grace before an expired lease is closed.
	pub channel_expiry_grace_blocks: u32,
	/// How long after payment expiry a late deposit is still auto-refunded.
	#[serde(with = "serde_u64_decimal_string")]
	pub late_deposit_refund_window_secs: u64,
}

/// Runtime service config (`GET`/`PUT /lsps1/options`). Changes apply to new orders
/// only; accepted orders keep the parameters they were created with.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
	/// Set once the expired lease was cooperatively closed.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub channel_closed_at_unix_secs: Option<u64>,
	pub rgb: Option<Lsps1ServiceRgbOrderDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps1ServiceOrdersResponse {
	pub orders: Vec<Lsps1ServiceOrderDto>,
}
