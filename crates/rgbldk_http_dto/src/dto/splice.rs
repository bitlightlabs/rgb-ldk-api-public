// Generated from rgb-ldk-node/src/http/dto/splice.rs. Do not edit.

//! Splice request DTOs (experimental, mirrors `Node::splice_in` / `Node::splice_out`).

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::serde_u64_decimal_string;

/// Request to splice additional on-chain funds into an existing channel (splice-in).
///
/// **Experimental**: this API mirrors `Node::splice_in` and the underlying
/// behavior may change in future releases. Currently only supported on pure
/// BTC channels — RGB asset channels are not yet covered.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SpliceInRequest {
	pub user_channel_id: String,
	pub counterparty_node_id: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub splice_amount_sats: u64,
}

/// Request to splice funds out of an existing channel to an on-chain address (splice-out).
///
/// **Experimental**: see [`SpliceInRequest`].
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SpliceOutRequest {
	pub user_channel_id: String,
	pub counterparty_node_id: String,
	pub address: String,
	#[serde(with = "serde_u64_decimal_string")]
	pub splice_amount_sats: u64,
}

