import type { LegAsset, SwapExecuteRequest } from "@bitlightlabs/rgbldk-node-api-types";

const asset: LegAsset = { Rgb: { asset_id: [22, 22] } };
const execute: SwapExecuteRequest = { execution_id: "ab".repeat(32) };
const replacement: SwapExecuteRequest = {
  execution_id: "ab".repeat(32),
  maker_to_taker: [],
  taker_to_maker: [],
};

// @ts-expect-error An RGB asset must wrap its asset_id inside an Rgb property.
const untaggedAsset: LegAsset = { asset_id: [22] };

void [asset, execute, replacement, untaggedAsset];
