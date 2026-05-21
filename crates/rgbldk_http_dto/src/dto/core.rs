// Generated from rgb-ldk-node/src/http/dto/core.rs. Do not edit.

use super::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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

