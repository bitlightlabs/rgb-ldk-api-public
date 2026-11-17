// Generated from rgb-ldk-node/crates/node-http/src/dto/lsps2.rs. Do not edit.

//! DTOs of LSPS2 (JIT channels). The LSP opens the channel when the first payment arrives.
//!
//! `u64` amounts and unix-second timestamps are decimal strings, as in the rest of the HTTP API.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::*;

/// `POST /lsps2/quote`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps2QuoteRequest {
	/// Hex-encoded node pubkey of the LSP. The LSP must be in the LSP list.
	pub lsp_node_id: String,
	/// Payment amount to compute the fee for. Optional.
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
	/// Fee of the cheapest offer that covers `amount_msat`. `null` if the request has no amount
	/// or no offer covers it.
	#[serde(with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub fee_msat: Option<u64>,
}

/// `POST /lsps2/invoice`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps2InvoiceRequest {
	pub lsp_node_id: String,
	/// Invoice amount. Omit it to get an invoice without an amount.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub amount_msat: Option<u64>,
	/// The most the LSP may take as its fee. Invoice with an amount: a total in msat. Invoice
	/// without an amount: ppm of the payment.
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
	pub payment_hash: String,
	/// Amount the payer pays. Absent if the invoice has no amount.
	#[serde(
		default,
		with = "serde_opt_u64_decimal_string",
		skip_serializing_if = "Option::is_none"
	)]
	#[schema(value_type = Option<String>)]
	pub amount_msat: Option<u64>,
	/// Set when the invoice has an amount. The fee the LSP takes from the payment.
	#[serde(
		default,
		with = "serde_opt_u64_decimal_string",
		skip_serializing_if = "Option::is_none"
	)]
	#[schema(value_type = Option<String>)]
	pub fee_msat: Option<u64>,
	/// Set when the invoice has no amount. The LSP takes this share (ppm) of the payment as its
	/// fee. The fee is never below `min_fee_msat`.
	#[serde(
		default,
		with = "serde_opt_u64_decimal_string",
		skip_serializing_if = "Option::is_none"
	)]
	#[schema(value_type = Option<String>)]
	pub fee_ppm: Option<u64>,
	#[serde(
		default,
		with = "serde_opt_u64_decimal_string",
		skip_serializing_if = "Option::is_none"
	)]
	#[schema(value_type = Option<String>)]
	pub min_fee_msat: Option<u64>,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub expires_at_unix_secs: u64,
	/// Number of blocks the LSP keeps the channel open.
	pub min_lifetime: u32,
}

/// What a client wants from an RGB JIT channel.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps2RgbQuoteRequest {
	pub lsp_node_id: String,
	pub contract_id: String,
	/// Asset amount of the payment to receive.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub asset_amount: u64,
	/// Asset amount the LSP puts into the channel. Must be at least `asset_amount`.
	/// The first payment moves `asset_amount` to this node. The rest stays on the LSP's side.
	/// This node can receive the rest in later payments.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub lsp_asset_balance: u64,
	/// BTC the LSP holds on its side of the new channel.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub lsp_balance_sat: u64,
	/// BTC sent along with the asset payment, without the fee. Minimum and default: 355000.
	/// The received asset sits on this node's output in the new channel. With less than 355 sat
	/// on it, that output is dropped as dust when the channel closes.
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
	/// Fee the LSP would charge now. This node computes it from the offering. The LSP sets the
	/// actual fee in `POST /lsps2/rgb_invoice`.
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
#[serde(from = "Lsps2RgbInvoiceRequestWire")]
pub struct Lsps2RgbInvoiceRequest {
	#[serde(flatten)]
	pub quote: Lsps2RgbQuoteRequest,
	/// Highest fee to accept. If the LSP asks for more, no invoice is created.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub max_fee_sat: u64,
	#[serde(default)]
	pub description: String,
	pub expiry_secs: u32,
}

/// Wire form of [`Lsps2RgbInvoiceRequest`], with the quote fields written out. serde cannot
/// reject unknown fields on a struct that uses `flatten`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Lsps2RgbInvoiceRequestWire {
	lsp_node_id: String,
	contract_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	asset_amount: u64,
	#[serde(with = "serde_u64_decimal_string")]
	lsp_asset_balance: u64,
	#[serde(with = "serde_u64_decimal_string")]
	lsp_balance_sat: u64,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	btc_carrier_amount_msat: Option<u64>,
	#[serde(with = "serde_u64_decimal_string")]
	max_fee_sat: u64,
	#[serde(default)]
	description: String,
	expiry_secs: u32,
}

impl From<Lsps2RgbInvoiceRequestWire> for Lsps2RgbInvoiceRequest {
	fn from(wire: Lsps2RgbInvoiceRequestWire) -> Self {
		Self {
			quote: Lsps2RgbQuoteRequest {
				lsp_node_id: wire.lsp_node_id,
				contract_id: wire.contract_id,
				asset_amount: wire.asset_amount,
				lsp_asset_balance: wire.lsp_asset_balance,
				lsp_balance_sat: wire.lsp_balance_sat,
				btc_carrier_amount_msat: wire.btc_carrier_amount_msat,
			},
			max_fee_sat: wire.max_fee_sat,
			description: wire.description,
			expiry_secs: wire.expiry_secs,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2RgbInvoiceResponse {
	pub invoice: String,
	pub payment_hash: String,
	pub fee: Lsps2RgbFeeDto,
	/// Amount the payer pays: carrier plus fee.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub amount_msat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub expires_at_unix_secs: u64,
	pub min_lifetime: u32,
}

/// Runtime options of this node's LSPS2 service. `PUT /lsps2/options` replaces all of them.
/// A change applies to fee offers made after it. Offers already given out keep their terms
/// until they expire.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Lsps2OptionsDto {
	/// `false` pauses the service. Clients get no fee offers and cannot buy. Channels already
	/// bought can still be paid for until their offer expires. Open channels are not affected.
	pub accept_new_channels: bool,
	/// Token clients must send. `null` or empty: any client is accepted.
	#[serde(default)]
	pub require_token: Option<String>,
	pub channel_opening_fee_ppm: u32,
	/// Channel capacity added on top of the forwarded amount, in ppm of that amount.
	pub channel_over_provisioning_ppm: u32,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub min_channel_opening_fee_msat: u64,
	/// Number of blocks the LSP keeps a paid channel open after its funding confirms.
	pub min_channel_lifetime: u32,
	pub max_client_to_self_delay: u32,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub min_payment_size_msat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub max_payment_size_msat: u64,
	/// Most channels one client may have with an unpaid opening fee. Minimum 1.
	pub max_unpaid_channels_per_peer: u32,
	/// Most channels all clients together may have with an unpaid opening fee. Minimum 1.
	/// To stop selling JIT channels, set `accept_new_channels` to `false`.
	pub max_unpaid_channels_total: u32,
	/// A channel with an unpaid fee is closed this many seconds after its offer expired.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub unpaid_channel_close_secs: u64,
}

/// `GET`/`PUT /lsps2/options` response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2OptionsResponse {
	/// Fixed at startup. `PUT` does not accept this field.
	pub advertise_service: bool,
	#[serde(flatten)]
	pub options: Lsps2OptionsDto,
}

/// One JIT channel of this node's LSPS2 service (`GET /lsps2/jit_channels`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2JitChannelDto {
	/// Whether this channel counts toward `max_unpaid_channels_per_peer` and
	/// `max_unpaid_channels_total`. True while it is opening or open and its fee is not paid.
	pub counts_toward_unpaid_quota: bool,
	pub user_channel_id: String,
	pub counterparty_node_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub intercept_scid: u64,
	/// One of `awaiting_payment`, `opening`, `awaiting_fee`, `fee_paid`, `closed`.
	/// `awaiting_payment`: bought, no payment has arrived. `awaiting_fee`: open, fee not paid.
	pub state: String,
	pub channel_id: Option<String>,
	/// When the first payment arrived. The channel open starts at that moment.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub opened_at_unix_secs: Option<u64>,
	/// When the offer the client bought expires. A channel with an unpaid fee is closed
	/// `unpaid_channel_close_secs` after this time.
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub valid_until_unix_secs: u64,
	/// Number of blocks the channel stays open after the fee is paid. The channel is closed when
	/// its funding has more confirmations than `min_lifetime + channel_expiry_grace_blocks`.
	pub min_lifetime: u32,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub amt_to_forward_msat: Option<u64>,
	/// Opening fee collected so far.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub opening_fee_msat: Option<u64>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub fee_paid_at_unix_secs: Option<u64>,
	pub rgb: Option<Lsps2JitRgbDto>,
	/// How this node closed the channel: `cooperative` or `force`. `force` happens after
	/// `force_close_after_blocks`.
	pub close_kind: Option<String>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub channel_closed_at_unix_secs: Option<u64>,
	/// Number of failed cooperative close attempts since the channel was due. Attempts while
	/// the client is offline are not counted. A restart resets the count.
	pub close_attempts: u32,
	pub last_close_error: Option<String>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub first_close_attempt_at_unix_secs: Option<u64>,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	#[schema(value_type = Option<String>)]
	pub next_close_attempt_at_unix_secs: Option<u64>,
}

/// Terms an RGB JIT channel was sold with.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2JitRgbDto {
	pub contract_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub asset_amount: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub lsp_asset_balance: u64,
	#[serde(with = "serde_u64_decimal_string")]
	#[schema(value_type = String)]
	pub lsp_balance_sat: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Lsps2JitChannelsResponse {
	pub channels: Vec<Lsps2JitChannelDto>,
}
