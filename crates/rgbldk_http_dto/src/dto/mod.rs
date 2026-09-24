// Generated from rgb-ldk-node/crates/node-http/src/dto/mod.rs. Do not edit.

//! HTTP API request/response types (v1).
//!
//! These are kept in-tree to ensure `ldk-node` remains self-contained.

#![allow(missing_docs)]

mod async_payments;
mod common;
mod core;
mod lsps1;
mod network_graph;
mod rgb;
mod splice;
mod swap;

pub use core::*;

pub use async_payments::*;
pub use common::*;
pub use lsps1::*;
pub use network_graph::*;
pub use rgb::*;
pub use splice::*;
pub use swap::*;

mod serde_u64_decimal_string {
	use serde::de::Error;
	use serde::{Deserialize, Deserializer, Serializer};

	pub fn serialize<S>(v: &u64, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		s.serialize_str(&v.to_string())
	}

	pub fn deserialize<'de, D>(d: D) -> Result<u64, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(d)?;
		s.parse::<u64>().map_err(D::Error::custom)
	}
}

mod serde_opt_u64_decimal_string {
	use serde::de::Error;
	use serde::{Deserialize, Deserializer, Serializer};

	pub fn serialize<S>(v: &Option<u64>, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match v {
			Some(n) => s.serialize_some(&n.to_string()),
			None => s.serialize_none(),
		}
	}

	pub fn deserialize<'de, D>(d: D) -> Result<Option<u64>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let opt = Option::<String>::deserialize(d)?;
		match opt {
			Some(s) => Ok(Some(s.parse::<u64>().map_err(D::Error::custom)?)),
			None => Ok(None),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn wallet_send_requests_use_decimal_amount_and_require_retain_reserves() {
		let send: WalletSendRequest = serde_json::from_value(serde_json::json!({
			"address": "bcrt1qtest",
			"amount_sats": "54321",
		}))
		.expect("valid send request should deserialize");
		assert_eq!(send.amount_sats, 54_321);
		assert!(send.fee_rate_sats_per_vb.is_none());

		assert!(serde_json::from_value::<WalletSendAllRequest>(serde_json::json!({
			"address": "bcrt1qtest",
		}))
		.is_err());

		let send_all: WalletSendAllRequest = serde_json::from_value(serde_json::json!({
			"address": "bcrt1qtest",
			"retain_reserves": true,
		}))
		.expect("valid send_all request should deserialize");
		assert!(send_all.retain_reserves);
		assert!(send_all.fee_rate_sats_per_vb.is_none());
	}

	#[test]
	fn rgb_onchain_invoice_create_request_allows_missing_blinding_utxo() {
		let req: RgbOnchainInvoiceCreateRequest = serde_json::from_value(serde_json::json!({
			"contract_id": "contract:dummy",
			"amount": "1",
			"use_witness_utxo": true,
		}))
		.expect("valid request should deserialize");
		assert!(req.blinding_utxo.is_none());

		let req: RgbOnchainInvoiceCreateRequest = serde_json::from_value(serde_json::json!({
			"contract_id": "contract:dummy",
			"amount": "1",
			"use_witness_utxo": false,
		}))
		.expect("valid request should deserialize");
		assert!(req.blinding_utxo.is_none());
	}
}
