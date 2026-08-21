// Generated from rgb-ldk-node/crates/node-http/src/dto/network_graph.rs. Do not edit.

//! Network-graph DTOs.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{serde_opt_u64_decimal_string, serde_u64_decimal_string};

/// List of node IDs (hex) currently known in the network graph.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkGraphNodesResponse {
	pub nodes: Vec<String>,
}

/// List of short channel IDs currently known in the network graph.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkGraphChannelsResponse {
	pub channels: Vec<u64>,
}

/// Information about a node, derived from channel announcements and (optionally) a node
/// announcement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkGraphNodeInfoResponse {
	pub channels: Vec<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub announcement_info: Option<NodeAnnouncementInfoDto>,
}

/// Information from a node's most recent node_announcement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NodeAnnouncementInfoDto {
	pub last_update: u32,
	pub alias: String,
	pub addresses: Vec<String>,
}

/// Information about a channel (both directions), as observed via channel_announcement and
/// channel_update gossip messages.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkGraphChannelInfoResponse {
	pub node_one: String,
	pub node_two: String,
	#[serde(default, with = "serde_opt_u64_decimal_string")]
	pub capacity_sats: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub one_to_two: Option<ChannelUpdateInfoDto>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub two_to_one: Option<ChannelUpdateInfoDto>,
}

/// One direction's `channel_update` info.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelUpdateInfoDto {
	pub last_update: u32,
	pub enabled: bool,
	pub cltv_expiry_delta: u16,
	#[serde(with = "serde_u64_decimal_string")]
	pub htlc_minimum_msat: u64,
	#[serde(with = "serde_u64_decimal_string")]
	pub htlc_maximum_msat: u64,
	pub fee_base_msat: u32,
	pub fee_proportional_millionths: u32,
}
