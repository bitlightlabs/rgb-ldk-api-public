// Generated from rgb-ldk-node/crates/node-http/src/dto/rgb.rs. Do not edit.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOpenChannelRequest {
	/// RGB contract ID.
	pub contract_id: String,
	/// RGB asset amount to commit into the channel.
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_amount: u64,
	/// RGB color context data (e.g. a consignment endpoint like `file://...`).
	pub color_context_data: String,
	/// Optional per-open override of the node's automatic funding UTXO policy.
	///
	/// Mutually exclusive with `funding_utxos`. When both are omitted, the node-level policy
	/// applies.
	#[serde(default)]
	pub funding_utxo_policy: Option<RgbFundingUtxoPolicyDto>,
	/// Optional exact set of funding UTXOs to spend for this channel.
	///
	/// Mutually exclusive with `funding_utxo_policy`. Outpoints must belong to the RGB wallet
	/// domain as listed by `/rgb/utxos`.
	#[serde(default)]
	pub funding_utxos: Option<Vec<RgbFundingUtxoDto>>,
}

/// Automatic funding UTXO selection policy for opening an RGB channel.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum RgbFundingUtxoPolicyDto {
	/// Fund from exactly one RGB-carrying UTXO; fail if none covers the requested amount.
	SingleRgbAnchor,
	/// Prefer one RGB-carrying UTXO, adding plain BTC inputs for fees when needed.
	RgbAnchorWithBtcSupport,
	/// Merge multiple RGB-carrying UTXOs when no single one covers the requested amount.
	MergeRgbAnchorsWithBtcSupport,
}

/// One caller-selected RGB channel funding UTXO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbFundingUtxoDto {
	/// Transaction id of the UTXO, as listed by `/rgb/utxos`.
	pub txid: String,
	/// Output index of the UTXO.
	pub vout: u32,
	/// Role the UTXO plays in the funding transaction.
	pub role: RgbFundingUtxoRoleDto,
}

/// Role of a caller-selected RGB channel funding UTXO.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum RgbFundingUtxoRoleDto {
	/// The UTXO carries RGB state of the channel's asset that will be spent into the channel.
	RgbState,
	/// The UTXO contributes only plain BTC value for fees or carrier support.
	FeeSupport,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbNewAddressResponse {
	/// Address owned by the dedicated RGB wallet descriptor.
	///
	/// Use this for RGB wallet outputs created via `/rgb/utxos/fund`, `/rgb/utxos/top_up`, or as
	/// RGB invoice beneficiaries. Outputs sent here appear in `/rgb/utxos` after `/rgb/sync`, not
	/// in `/wallet/utxos`.
	pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbDerivedDescriptorDto {
	/// BIP32 master fingerprint associated with this derived descriptor.
	pub fingerprint: String,
	/// BIP32 derivation path used for this derived public descriptor.
	pub derivation_path: String,
	/// Derived extended public key.
	pub xpub: String,
	/// Descriptor string that can derive the corresponding public scripts.
	pub descriptor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbDescriptorResponse {
	/// Public RGB wallet root descriptor.
	pub descriptor: String,
	/// Derived public descriptors exposed for audits and interoperability tooling.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub derived_descriptors: Vec<RgbDerivedDescriptorDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RgbSignMessageAlgorithmDto {
	BitcoinSignedMessage,
	Ecdsa,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RgbSignMessageEncodingDto {
	Hex,
	Base64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbSignMessageRequest {
	/// Message string to sign.
	///
	/// For `bitcoin_signed_message`, the server signs this UTF-8 string directly. For `ecdsa`,
	/// this field must contain bytes encoded with the selected `encoding`.
	pub message: String,
	/// Signature mode. Defaults to `bitcoin_signed_message`.
	#[serde(default)]
	pub algorithm: Option<RgbSignMessageAlgorithmDto>,
	/// When `algorithm=ecdsa`, request a compact ECDSA signature if supported.
	#[serde(default)]
	pub compact: Option<bool>,
	/// Required when `algorithm=ecdsa`; ignored for `bitcoin_signed_message`.
	#[serde(default)]
	pub encoding: Option<RgbSignMessageEncodingDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbSignMessageResponse {
	/// Original request message.
	pub message: String,
	/// Signature algorithm actually used.
	pub algorithm: String,
	/// Signature encoded according to `encoding` (or the Bitcoin signed-message format).
	pub signature: String,
	/// Encoding used for `signature`, when applicable.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encoding: Option<String>,
	/// Whether the returned ECDSA signature is compact.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub compact: Option<bool>,
	/// Public key that produced the signature.
	pub pubkey: String,
	/// RGB descriptor derivation path used for signing.
	pub derivation_path: String,
	/// SHA-256 digest hex used by `ecdsa` mode.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub digest_hex: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbIssuersResponse {
	/// Valid issuer names currently present in the local issuer registry.
	pub issuers: Vec<String>,
	/// Corrupt or unloadable issuer files skipped by the registry scan.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub invalid_issuers: Vec<RgbInvalidIssuerDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbInvalidIssuerDto {
	/// Issuer file base name.
	pub name: String,
	/// Error returned when attempting to load the issuer.
	pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbIssuersImportResponse {
	/// Always `true` on success.
	pub ok: bool,
	/// Stored issuer name.
	pub issuer_name: String,
	/// Validation and storage checks performed during import.
	#[serde(default)]
	pub checks: Vec<HealthCheckDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbContractsImportResponse {
	pub ok: bool,
	pub contract_id: String,
	/// Storage key used by the RGB wallet to read the imported consignment.
	pub consignment_key: String,
	/// Validation and import checks performed during the request.
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
	/// Validation and issuance checks performed during the request.
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
	/// Validation and export checks performed during the request.
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
	/// RGB asset amount to request from the payer.
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_amount: u64,
	/// Human-readable description.
	pub description: String,
	/// Invoice expiry in seconds. If omitted, the BOLT11 default expiry is used.
	#[serde(default)]
	pub expiry_secs: Option<u32>,
	/// Optional BTC carrier amount in msat embedded into the BOLT11 invoice.
	///
	/// Explicit values are allowed down to the hard 1-sat minimum. They are still checked against
	/// the receiver's current channel safety constraints and may be raised or rejected if the
	/// channel cannot safely host RGB.
	///
	/// When omitted, the server starts from the active RGB carrier default floor and, if needed,
	/// raises the invoice carrier to satisfy inbound HTLC limits or the 354-sat non-dust holder
	/// reserve threshold so the receiver can host the incoming RGB without extra client-side logic.
	///
	/// A trimmed 1-sat carrier only remains valid when the selected receiving channel already holds
	/// a non-dust BTC holder reserve locally. Otherwise the server auto-raises the carrier or
	/// rejects the request with `HolderReserveTooLow`.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub btc_carrier_amount_msat: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnInvoiceCreateForHashRequest {
	/// RGB contract ID.
	pub contract_id: String,
	/// RGB asset amount to request from the payer.
	#[serde(with = "serde_u64_decimal_string")]
	pub asset_amount: u64,
	/// Payment hash (64 hex chars) provided by caller.
	pub payment_hash: String,
	/// Human-readable description.
	pub description: String,
	/// Invoice expiry in seconds. If omitted, the BOLT11 default expiry is used.
	#[serde(default)]
	pub expiry_secs: Option<u32>,
	/// Optional BTC carrier amount in msat embedded into the BOLT11 invoice.
	///
	/// Explicit values are allowed down to the hard 1-sat minimum. They are still checked against
	/// the receiver's current channel safety constraints and may be raised or rejected if the
	/// channel cannot safely host RGB.
	///
	/// When omitted, the server starts from the active RGB carrier default floor and, if needed,
	/// raises the invoice carrier to satisfy inbound HTLC limits or the 354-sat non-dust holder
	/// reserve threshold so the receiver can host the incoming RGB without extra client-side logic.
	///
	/// A trimmed 1-sat carrier only remains valid when the selected receiving channel already holds
	/// a non-dust BTC holder reserve locally. Otherwise the server auto-raises the carrier or
	/// rejects the request with `HolderReserveTooLow`.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub btc_carrier_amount_msat: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnCarrierEstimateChannelDto {
	/// Channel id (32-byte hex).
	pub channel_id: String,
	/// Local user channel id (hex-encoded 16 bytes BIG-ENDIAN).
	pub user_channel_id: String,
	/// Whether this channel can currently send/receive HTLCs.
	pub is_usable: bool,
	/// Available inbound capacity in millisatoshis.
	#[serde(with = "serde_u64_decimal_string")]
	pub inbound_capacity_msat: u64,
	/// Smallest inbound HTLC this channel currently accepts.
	#[serde(with = "serde_u64_decimal_string")]
	pub inbound_htlc_minimum_msat: u64,
	/// Largest inbound HTLC this channel currently accepts, if known.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub inbound_htlc_maximum_msat: Option<u64>,
	/// Estimated local BTC output on this channel that can host RGB, in satoshis.
	#[serde(with = "serde_u64_decimal_string")]
	pub local_balance_output_sats: u64,
	/// Whether this channel already satisfies the non-dust RGB holder reserve locally.
	pub has_holder_reserve: bool,
	/// Whether this channel can currently receive at least the hard minimum RGB carrier.
	pub receive_available: bool,
	/// Whether invoice creation can currently use this channel when the carrier is omitted.
	#[serde(default)]
	pub can_receive_rgb_invoice: bool,
	/// Why default invoice creation is currently blocked on this channel.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub blocking_reason: Option<String>,
	/// Carrier this channel would need for default invoice creation, if known.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub required_carrier_msat: Option<u64>,
	/// Why `required_carrier_msat` was selected, if known.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required_carrier_reason: Option<String>,
	/// Available inbound BTC carrier capacity in millisatoshis.
	#[serde(default)]
	#[serde(with = "serde_u64_decimal_string")]
	pub available_inbound_capacity_msat: u64,
	/// Suggested action when default invoice creation is blocked on this channel.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub suggested_action: Option<String>,
	/// Suggested minimum viable carrier for this channel, if currently receive-capable.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub minimum_viable_carrier_amount_msat: Option<u64>,
	/// Why `minimum_viable_carrier_amount_msat` was selected.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub minimum_viable_reason: Option<String>,
	/// Suggested carrier for this channel when invoice creation omits `btc_carrier_amount_msat`.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub default_create_carrier_amount_msat: Option<u64>,
	/// Why `default_create_carrier_amount_msat` was selected.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_create_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnCarrierEstimateResponse {
	/// Whether at least one channel can currently receive an RGB LN carrier.
	pub receive_available: bool,
	/// Whether invoice creation can currently succeed when the carrier is omitted.
	#[serde(default)]
	pub can_create_rgb_invoice: bool,
	/// Why default invoice creation is currently blocked.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub blocking_reason: Option<String>,
	/// Carrier required for default invoice creation, if known.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub required_carrier_msat: Option<u64>,
	/// Why `required_carrier_msat` was selected, if known.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required_carrier_reason: Option<String>,
	/// Best available inbound BTC carrier capacity from the current snapshot, if known.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub available_inbound_capacity_msat: Option<u64>,
	/// Suggested action when default invoice creation is blocked.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub suggested_action: Option<String>,
	/// Lowest carrier that looks viable from the node's current channel snapshot.
	///
	/// This top-level estimate is conservative for invoices that do not bind a specific receiving
	/// channel. Use `channels[]` to show per-channel suggestions.
	#[serde(with = "serde_u64_decimal_string")]
	pub minimum_viable_carrier_amount_msat: u64,
	/// Why `minimum_viable_carrier_amount_msat` was selected.
	pub minimum_viable_reason: String,
	/// Carrier the backend would currently use if invoice creation omitted `btc_carrier_amount_msat`.
	#[serde(with = "serde_u64_decimal_string")]
	pub default_create_carrier_amount_msat: u64,
	/// Why `default_create_carrier_amount_msat` was selected.
	pub default_create_reason: String,
	/// Default RGB carrier floor in msat used only when invoice creation omits the carrier field.
	#[serde(with = "serde_u64_decimal_string")]
	pub carrier_admission_threshold_msat: u64,
	/// Hard minimum explicit RGB carrier amount in msat.
	#[serde(with = "serde_u64_decimal_string")]
	pub minimum_allowed_carrier_amount_msat: u64,
	/// Non-dust holder reserve threshold in msat.
	#[serde(with = "serde_u64_decimal_string")]
	pub holder_reserve_threshold_msat: u64,
	/// Per-channel estimates from the same channel snapshot.
	pub channels: Vec<RgbLnCarrierEstimateChannelDto>,
	/// Whether this response is only a current-state estimate.
	pub estimate_only: bool,
	/// Human-readable warning about the estimate boundary.
	pub warning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnInvoiceResponse {
	/// Serialized BOLT11 invoice string.
	pub invoice: String,
	/// Actual BTC carrier amount embedded into the BOLT11 invoice.
	///
	/// This is the authoritative value the payer must send. It may be higher than the omitted or
	/// requested amount when the server auto-raises the carrier to satisfy the receiver's holder
	/// reserve requirement.
	#[serde(with = "serde_u64_decimal_string")]
	pub btc_carrier_amount_msat: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnInvoiceDecodeRequest {
	/// Serialized BOLT11 invoice string to parse.
	pub invoice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnInvoiceDecodeResponse {
	/// Payment hash embedded in the invoice.
	pub payment_hash: String,
	/// Payee node id recovered from the invoice signature.
	pub destination: String,
	/// BTC carrier amount, if the invoice specifies one.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub carrier_amount_msat: Option<u64>,
	/// Invoice expiry in seconds.
	#[serde(with = "serde_u64_decimal_string")]
	pub expiry_secs: u64,
	/// RGB contract id embedded in the invoice, if present.
	#[serde(default)]
	pub contract_id: Option<String>,
	/// RGB asset amount embedded in the invoice, if present.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub asset_amount: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbLnPayRequest {
	/// Serialized BOLT11 invoice string.
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
	/// RGB contract id to receive.
	pub contract_id: String,
	/// RGB amount to receive.
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
	/// Serialized RGB on-chain invoice string.
	pub invoice: String,
	/// Outpoint actually used for blinding when creating a blinded invoice (`use_witness_utxo=false`).
	#[serde(default)]
	pub blinding_utxo_used: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainInvoiceDecodeRequest {
	/// Serialized RGB on-chain invoice string to parse.
	pub invoice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainInvoiceDecodeResponse {
	/// Contract id embedded in the invoice.
	pub contract_id: String,
	/// RGB amount embedded in the invoice.
	#[serde(with = "serde_u64_decimal_string")]
	pub amount: u64,
	/// Beneficiary string (`wout:...` or `at:...`).
	pub beneficiary: String,
	/// Whether the beneficiary is witness-output based.
	pub use_witness_utxo: bool,
	/// Absolute expiry timestamp, if present.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub expiry_unix_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainSendRequest {
	/// RGB on-chain invoice to fulfill.
	pub invoice: String,
	/// Optional explicit BTC amount to provide for fee payment and carrier outputs.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub sats_for_fee_and_outputs: Option<u64>,
	/// Positive fee rate in sat/vB.
	pub fee_rate_sats_per_vb: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainSendResponse {
	/// Broadcast Bitcoin transaction id.
	pub txid: String,
	/// Consignment cache key produced for the transfer.
	pub consignment_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainReceiveRequest {
	/// Stored consignment key previously returned by an RGB send/export endpoint.
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
	/// Contract id accepted from the consignment.
	pub contract_id: String,
	/// RGB amount accepted from the consignment.
	#[serde(with = "serde_u64_decimal_string")]
	pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainPaymentsResponse {
	/// Recorded RGB on-chain payment entries.
	pub payments: Vec<RgbOnchainPaymentDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbOnchainPaymentDto {
	/// Stable payment id.
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
	/// Internal maintenance marker: `utxo_merge` | `utxo_top_up`; absent for ordinary payments.
	/// Frontends can use it to keep self-transfers out of user-facing payment history.
	#[serde(default)]
	pub purpose: Option<String>,
}

/// Canonical RGB-wallet UTXO view.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosResponse {
	/// RGB spend-domain UTXOs with semantic and lock metadata.
	pub utxos: Vec<RgbUtxoDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosFundInputDto {
	/// Explicit ordinary BTC-wallet input to spend, formatted as `txid:vout`.
	///
	/// Source these from `GET /wallet/utxos` after `POST /wallet/sync`.
	pub outpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosFundOutputDto {
	/// RGB wallet address returned by `/rgb/address/new`.
	pub address: String,
	/// BTC capacity to assign to the created RGB wallet output.
	#[serde(with = "serde_u64_decimal_string")]
	pub value_sats: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosFundRequest {
	/// Exact ordinary BTC-wallet inputs to consume.
	///
	/// This endpoint does not auto-select inputs.
	pub inputs: Vec<RgbUtxosFundInputDto>,
	/// Exact RGB wallet outputs to create.
	///
	/// Generate each destination with `/rgb/address/new`.
	pub outputs: Vec<RgbUtxosFundOutputDto>,
	/// Explicit ordinary BTC-wallet change address returned by `/wallet/address/new`.
	///
	/// This must stay distinct from all RGB output scripts.
	pub change_address: String,
	/// Positive fee rate in sat/vB.
	pub fee_rate_sats_per_vb: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosFundCreatedOutputDto {
	/// RGB wallet address script used for the created output.
	pub address: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub value_sats: u64,
	/// Output index inside the broadcast transaction.
	pub vout: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosFundChangeDto {
	/// Ordinary BTC-wallet change address.
	pub address: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub value_sats: u64,
	/// Output index inside the broadcast transaction.
	pub vout: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosFundResponse {
	/// Broadcast Bitcoin transaction id.
	pub txid: String,
	/// Currently `broadcast` when the signed transaction has been handed to the broadcaster.
	pub status: String,
	/// Created RGB wallet outputs, in transaction order.
	pub outputs: Vec<RgbUtxosFundCreatedOutputDto>,
	/// Ordinary BTC-wallet change output, if any.
	#[serde(default)]
	pub change: Option<RgbUtxosFundChangeDto>,
	/// Network fee paid by the transaction in sats.
	#[serde(with = "serde_u64_decimal_string")]
	pub fee_sats: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosSweepInputDto {
	/// RGB wallet outpoint to spend, formatted as `txid:vout`.
	///
	/// Select this from `GET /rgb/utxos`.
	pub outpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosSweepRequest {
	/// Empty RGB wallet outpoint to sweep.
	pub input: RgbUtxosSweepInputDto,
	/// Explicit ordinary BTC-wallet address returned by `/wallet/address/new`.
	pub destination_address: String,
	/// Positive fee rate in sat/vB.
	pub fee_rate_sats_per_vb: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosSweepDestinationDto {
	/// Ordinary BTC-wallet destination address.
	pub address: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub value_sats: u64,
	/// Output index inside the broadcast transaction.
	pub vout: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosSweepResponse {
	/// Broadcast Bitcoin transaction id.
	pub txid: String,
	/// Currently `broadcast` when the signed transaction has been handed to the broadcaster.
	pub status: String,
	/// Swept RGB-wallet input.
	pub input: RgbUtxosSweepInputDto,
	/// Ordinary BTC-wallet destination created by the sweep.
	pub destination: RgbUtxosSweepDestinationDto,
	/// Network fee paid by the transaction in sats.
	#[serde(with = "serde_u64_decimal_string")]
	pub fee_sats: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosTopUpRgbInputDto {
	/// RGB wallet outpoint to replace, formatted as `txid:vout`.
	///
	/// Select this from `GET /rgb/utxos`.
	pub outpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosTopUpL1InputDto {
	/// Explicit ordinary BTC-wallet input to consume, formatted as `txid:vout`.
	///
	/// Source these from `GET /wallet/utxos` after `POST /wallet/sync`.
	pub outpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosTopUpRgbOutputDto {
	/// RGB wallet address returned by `/rgb/address/new`.
	pub address: String,
	/// Target BTC capacity for the replacement RGB UTXO.
	///
	/// This must be greater than the old RGB UTXO's BTC value.
	#[serde(with = "serde_u64_decimal_string")]
	pub target_value_sats: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosTopUpRequest {
	/// Confirmed unlocked RGB input to replace.
	pub rgb_input: RgbUtxosTopUpRgbInputDto,
	/// Extra ordinary BTC-wallet inputs used to enlarge the RGB output.
	pub l1_inputs: Vec<RgbUtxosTopUpL1InputDto>,
	/// Replacement RGB wallet output.
	pub rgb_output: RgbUtxosTopUpRgbOutputDto,
	/// Explicit ordinary BTC-wallet change address returned by `/wallet/address/new`.
	pub change_address: String,
	/// Positive fee rate in sat/vB.
	pub fee_rate_sats_per_vb: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosTopUpCreatedOutputDto {
	/// Replacement RGB wallet address.
	pub address: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub value_sats: u64,
	/// Output index inside the broadcast transaction.
	pub vout: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosTopUpChangeDto {
	/// Ordinary BTC-wallet change address.
	pub address: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub value_sats: u64,
	/// Output index inside the broadcast transaction.
	pub vout: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosTopUpResponse {
	/// Broadcast Bitcoin transaction id.
	pub txid: String,
	/// Currently `broadcast` when the signed transaction has been handed to the broadcaster.
	pub status: String,
	/// Replaced RGB outpoint.
	pub old_outpoint: String,
	/// Newly created replacement RGB output.
	pub new_rgb_output: RgbUtxosTopUpCreatedOutputDto,
	/// Ordinary BTC-wallet change output, if any.
	#[serde(default)]
	pub change: Option<RgbUtxosTopUpChangeDto>,
	/// Network fee paid by the transaction in sats.
	#[serde(with = "serde_u64_decimal_string")]
	pub fee_sats: u64,
	/// Consignment cache key produced for the replacement transfer.
	pub consignment_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosMergeRequest {
	/// Contract whose spendable RGB UTXOs will be consolidated.
	pub contract_id: String,
	/// Existing RGB-wallet outpoint (`txid:vout`) that receives the full merged balance.
	///
	/// Must be deeply confirmed, unlocked, and hold either no RGB assets or only the
	/// target contract. It is never spent by the merge transaction.
	pub destination_utxo: String,
	/// Also spend UTXOs bound to pending receive invoices. Defaults to `false`.
	///
	/// RGB invoices cannot be revoked: a payer that still pays one of the affected
	/// invoices afterwards burns their own assets. Callers must list the affected
	/// invoices and ask for explicit user confirmation before setting this.
	#[serde(default)]
	pub include_invoice_bound_utxos: Option<bool>,
	/// Positive fee rate in sat/vB. Defaults to 2.0.
	#[serde(default)]
	pub fee_rate_sats_per_vb: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosMergeResponse {
	/// UTXO operation id tracking the merge (`rgb-utxo-merge-...`).
	pub operation_id: String,
	/// Broadcast Bitcoin transaction id.
	pub txid: String,
	/// Consolidated inputs (`txid:vout`), spent by the merge transaction.
	pub merged_inputs: Vec<String>,
	/// Total asset amount moved onto the destination by this call.
	#[serde(with = "serde_u64_decimal_string")]
	pub total_amount: u64,
	/// Mergeable UTXOs left for a follow-up call (a single call caps at 256 inputs).
	pub remaining_count: u32,
	/// `confirming` right after broadcast; poll `GET /rgb/utxos/merge/status`.
	pub status: String,
	/// Consignment cache key produced for the self-transfer.
	pub consignment_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosMergeStatusResponse {
	/// Merges whose destination reservation was still held when this call ran, newest first.
	/// Entries reported `done` had their reservation released by this very call, so they
	/// disappear from subsequent responses.
	pub merges: Vec<RgbUtxosMergeStatusEntryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosMergeStatusEntryDto {
	/// Merge transaction id.
	pub txid: String,
	/// Destination outpoint (`txid:vout`) receiving the merged balance.
	pub destination_utxo: String,
	/// Contract that was consolidated, when recorded.
	#[serde(default)]
	pub contract_id: Option<String>,
	/// `confirming` until the merge transaction is deeply confirmed, then `done`.
	pub status: String,
	/// Confirmation depth of the merge transaction (0 while unconfirmed).
	pub confirmations: u32,
	/// Whether this call released the destination reservation.
	pub released: bool,
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
pub struct RgbUtxoDto {
	/// Outpoint formatted as `txid:vout`.
	pub outpoint: String,
	/// BTC value of the output in sats.
	#[serde(with = "serde_u64_decimal_string")]
	pub value_sats: u64,
	/// Confirmation status from the node's wallet/chain view.
	pub confirmation: RgbUtxoConfirmationDto,
	/// RGB allocations and spend-role metadata anchored on this outpoint.
	pub rgb: RgbUtxoRgbDto,
	/// txoscope lock state for this RGB-wallet outpoint.
	pub lock: RgbUtxoLockDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxoConfirmationDto {
	/// One of: confirmed | mempool.
	pub status: RgbUtxoConfirmationStatusDto,
	/// Confirmation block height, if known.
	#[serde(default)]
	pub height: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RgbUtxoConfirmationStatusDto {
	Confirmed,
	Mempool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxoRgbDto {
	/// RGB allocations grouped by contract/layer.
	pub allocations: Vec<RgbAllocationDto>,
	/// Whether this outpoint holds allocations from multiple contracts.
	pub has_mixed_asset_allocations: bool,
	/// Semantic spend roles advertised by the RGB runtime for this outpoint.
	pub spend_roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbAllocationDto {
	/// Contract id that owns this allocation.
	pub contract_id: String,
	/// Amount anchored on this UTXO for the given contract/layer.
	#[serde(with = "serde_u64_decimal_string")]
	pub amount: u64,
	/// RGB state layer name (for example `base`, `active`, or `pending`).
	pub layer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxoLockDto {
	/// Whether this outpoint is currently unavailable for new flows.
	pub locked: bool,
	/// One of: none | manual_reservation | operation.
	pub kind: RgbUtxoLockKindDto,
	/// Reservation or selected operation id, if locked.
	#[serde(default)]
	pub operation_id: Option<String>,
	/// Lock expiry in unix seconds, if txoscope exposes one.
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub expires_at_unix_secs: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RgbUtxoLockKindDto {
	None,
	ManualReservation,
	Operation,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosReserveRequest {
	/// Optional explicit outpoint to reserve (`txid:vout`).
	///
	/// If omitted, the node auto-selects one available RGB-wallet UTXO.
	#[serde(default)]
	pub outpoint: Option<String>,
	/// Optional reservation TTL in seconds (default: 300).
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub ttl_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosReserveResponse {
	/// Stable reservation id returned by the orchestrator.
	pub reservation_id: String,
	/// Reserved RGB-wallet outpoint.
	pub outpoint: String,
	/// Reservation expiry timestamp in unix seconds.
	#[serde(with = "serde_u64_decimal_string")]
	pub reserved_until_unix_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosReleaseRequest {
	/// Release by reservation id (preferred).
	#[serde(default)]
	pub reservation_id: Option<String>,
	/// Release by outpoint (`txid:vout`) when the reservation id is not available.
	#[serde(default)]
	pub outpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbUtxosReleaseResponse {
	/// Whether a matching reservation existed and was released.
	pub released: bool,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rgb_open_channel_request_parses_funding_utxo_policy() {
		let json = r#"{
			"contract_id": "contract",
			"asset_amount": "15",
			"color_context_data": "file:///tmp",
			"funding_utxo_policy": "merge-rgb-anchors-with-btc-support"
		}"#;
		let req: RgbOpenChannelRequest = serde_json::from_str(json).unwrap();
		assert!(matches!(
			req.funding_utxo_policy,
			Some(RgbFundingUtxoPolicyDto::MergeRgbAnchorsWithBtcSupport)
		));
		assert!(req.funding_utxos.is_none());
	}

	#[test]
	fn rgb_open_channel_request_parses_funding_utxos() {
		let json = r#"{
			"contract_id": "contract",
			"asset_amount": "15",
			"color_context_data": "file:///tmp",
			"funding_utxos": [
				{"txid": "aa", "vout": 1, "role": "rgb-state"},
				{"txid": "bb", "vout": 0, "role": "fee-support"}
			]
		}"#;
		let req: RgbOpenChannelRequest = serde_json::from_str(json).unwrap();
		let utxos = req.funding_utxos.unwrap();
		assert_eq!(utxos.len(), 2);
		assert!(matches!(utxos[0].role, RgbFundingUtxoRoleDto::RgbState));
		assert_eq!(utxos[0].vout, 1);
		assert!(matches!(utxos[1].role, RgbFundingUtxoRoleDto::FeeSupport));
		assert!(req.funding_utxo_policy.is_none());
	}

	#[test]
	fn rgb_open_channel_request_parses_without_funding_fields() {
		let json = r#"{
			"contract_id": "contract",
			"asset_amount": "15",
			"color_context_data": "file:///tmp"
		}"#;
		let req: RgbOpenChannelRequest = serde_json::from_str(json).unwrap();
		assert!(req.funding_utxo_policy.is_none());
		assert!(req.funding_utxos.is_none());
	}
}
