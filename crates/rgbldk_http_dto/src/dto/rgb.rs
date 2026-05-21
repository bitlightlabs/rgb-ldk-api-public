// Generated from rgb-ldk-node/src/http/dto/rgb.rs. Do not edit.

use super::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
pub struct RgbDerivedDescriptorDto {
	pub fingerprint: String,
	pub derivation_path: String,
	pub xpub: String,
	pub descriptor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbDescriptorResponse {
	pub descriptor: String,
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
	pub message: String,
	#[serde(default)]
	pub algorithm: Option<RgbSignMessageAlgorithmDto>,
	#[serde(default)]
	pub compact: Option<bool>,
	#[serde(default)]
	pub encoding: Option<RgbSignMessageEncodingDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbSignMessageResponse {
	pub message: String,
	pub algorithm: String,
	pub signature: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encoding: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub compact: Option<bool>,
	pub pubkey: String,
	pub derivation_path: String,
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
	/// RGB spend-domain UTXOs with semantic metadata.
	pub utxos: Vec<RgbUtxoDto>,
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
	#[serde(with = "serde_u64_decimal_string")]
	pub value_sats: u64,
	#[serde(default)]
	pub confirmed_height: Option<u32>,
	pub rgb_allocations: Vec<RgbAllocationDto>,
	pub has_mixed_asset_allocations: bool,
	pub spend_roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RgbAllocationDto {
	pub contract_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub amount: u64,
	pub layer: String,
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

