// Generated from rgb-ldk-node/src/http/dto/async_payments.rs. Do not edit.

//! BOLT-12 async payment DTOs (experimental).

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response of `/payment/bolt12/async/receive_offer`.
///
/// **Experimental**: BOLT-12 async payments support is experimental.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AsyncReceiveOfferResponse {
	/// The async-payment offer as a `lno1...` string.
	pub offer: String,
}

/// Request body for `/payment/bolt12/async/set_static_invoice_server_paths`.
///
/// `paths_hex` is the hex-encoded `Writeable` serialization of `Vec<BlindedMessagePath>`,
/// matching the wire format used by the UniFFI bindings.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct AsyncSetStaticInvoiceServerPathsRequest {
	pub paths_hex: String,
}

/// Request body for `/payment/bolt12/async/blinded_paths_for_recipient`.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct AsyncBlindedPathsRequest {
	/// Hex-encoded recipient identifier; opaque bytes chosen by the caller.
	pub recipient_id_hex: String,
}

/// Response of `/payment/bolt12/async/blinded_paths_for_recipient`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AsyncBlindedPathsResponse {
	/// Hex-encoded `Writeable` serialization of the resulting `Vec<BlindedMessagePath>`.
	pub paths_hex: String,
}

