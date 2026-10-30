// Generated from rgb-ldk-node/crates/node-http/src/dto/lsps2.rs. Do not edit.

//! LSPS2 (JIT channel) DTOs: receiving through a channel the LSP opens on payment.
//!
//! Wire conventions match the rest of the HTTP API: `u64` amounts and unix-second
//! timestamps are decimal strings.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::*;

/// `POST /lsps2/quote`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2QuoteRequest {
	/// LSP node pubkey, hex-encoded; must be in the LSP list.
	pub lsp_node_id: String,
	/// Payment amount to price, if any.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub amount_msat: Option<u64>,
}

/// One fee offer of the LSP for plain BTC JIT channels.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2FeeOfferDto {
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub min_fee_msat: u64,
	pub proportional_ppm: u32,
	pub valid_until: String,
	pub min_lifetime: u32,
	pub max_client_to_self_delay: u32,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub min_payment_size_msat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub max_payment_size_msat: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2QuoteResponse {
	pub offers: Vec<Lsps2FeeOfferDto>,
	/// Fee of the cheapest offer covering `amount_msat`; `null` without an amount or when
	/// no offer covers it.
	#[serde(with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub fee_msat: Option<u64>,
}

/// `POST /lsps2/invoice`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2InvoiceRequest {
	pub lsp_node_id: String,
	/// Invoice amount; omit for a variable-amount invoice.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub amount_msat: Option<u64>,
	/// Fixed amount: the most the LSP may take in total. Variable amount: the most it may
	/// take in ppm of the payment.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub max_fee: Option<u64>,
	#[serde(default)]
	pub description: String,
	pub expiry_secs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2InvoiceResponse {
	pub invoice: String,
}

/// What a client wants from an RGB JIT channel.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2RgbQuoteRequest {
	pub lsp_node_id: String,
	pub contract_id: String,
	/// Asset amount of the payment to receive.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub asset_amount: u64,
	/// Asset amount the LSP puts into the channel; at least `asset_amount`. What is left
	/// after the first payment stays as inbound asset capacity for later payments.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub lsp_asset_balance: u64,
	/// BTC capacity the LSP puts into the channel.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub lsp_balance_sat: u64,
	/// BTC carrier of the payment, excluding the fee; at least 354000 (the holder reserve
	/// of the new channel). Defaults to 354000.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub btc_carrier_amount_msat: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2RgbFeeDto {
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub fee_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub onchain_cost_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub btc_rent_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub asset_rent_sat: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2RgbAssetOfferDto {
	pub asset_id: String,
	pub ticker: String,
	pub precision: u8,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub asset_unit_price_sat: u64,
	pub asset_rent_ppm_per_year: u32,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub min_lsp_asset_balance: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub max_lsp_asset_balance: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2RgbOfferingDto {
	pub assets: Vec<Lsps2RgbAssetOfferDto>,
	pub btc_capacity_ppm_per_year: u32,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub onchain_cost_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub min_fee_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub min_lsp_balance_sat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub max_lsp_balance_sat: u64,
	pub min_lifetime: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2RgbQuoteResponse {
	pub offering: Lsps2RgbOfferingDto,
	/// The fee the LSP would charge now, estimated locally; the actual fee is set by
	/// `POST /lsps2/rgb_invoice`.
	pub estimated_fee: Lsps2RgbFeeDto,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub min_btc_carrier_amount_msat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub max_btc_carrier_amount_msat: u64,
}

/// `POST /lsps2/rgb_invoice`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2RgbInvoiceRequest {
	#[serde(flatten)]
	pub quote: Lsps2RgbQuoteRequest,
	/// No invoice is created if the LSP's fee is above this.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub max_fee_sat: u64,
	#[serde(default)]
	pub description: String,
	pub expiry_secs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2RgbInvoiceResponse {
	pub invoice: String,
	pub payment_hash: String,
	pub fee: Lsps2RgbFeeDto,
	/// What the payer pays: the carrier plus the fee.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub amount_msat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub expires_at_unix_secs: u64,
	pub min_lifetime: u32,
}

/// Runtime options of this node's LSPS2 service (`PUT /lsps2/options`, full replace).
/// Changes apply to fee offers made afterwards; offers already given out keep their terms
/// until they expire.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2OptionsDto {
	/// Token clients must send; `null` or empty accepts any client.
	#[serde(default)]
	pub require_token: Option<String>,
	pub channel_opening_fee_ppm: u32,
	/// Extra channel capacity on top of the forwarded amount, in ppm.
	pub channel_over_provisioning_ppm: u32,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub min_channel_opening_fee_msat: u64,
	/// Blocks the LSP keeps a paid channel open after confirmation.
	pub min_channel_lifetime: u32,
	pub max_client_to_self_delay: u32,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub min_payment_size_msat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub max_payment_size_msat: u64,
	/// Channels whose opening fee is still outstanding one client may have at once.
	pub max_unpaid_channels_per_peer: u32,
	/// Channels whose opening fee is still outstanding all clients may have at once.
	pub max_unpaid_channels_total: u32,
	/// Seconds after the offer expired when a channel whose fee is still outstanding is closed.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub unpaid_channel_close_secs: u64,
}

/// `GET`/`PUT /lsps2/options` response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2OptionsResponse {
	/// Set at startup only.
	pub advertise_service: bool,
	#[serde(flatten)]
	pub options: Lsps2OptionsDto,
}
