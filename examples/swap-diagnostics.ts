import type { SwapFailureDetails } from "@bitlightlabs/rgbldk-node-api-types";

/** Use after validating the response body against the generated OpenAPI schema. */
export function describeSwapDiagnostic(details: SwapFailureDetails): string {
  switch (details.kind) {
    case "channel_policy":
      return `Hop ${details.hop_index} on channel ${details.policy.channel_scid}: ${details.btc_amount_msat} msat; channel range ${details.policy.htlc_minimum_msat}–${details.policy.htlc_maximum_msat} msat`;
    case "channel_capacity":
      return `Channel ${details.channel_scid}: ${details.outbound_capacity_msat} msat available, ${details.required_outbound_msat ?? "unspecified"} msat required`;
    case "cltv_limit":
      return `CLTV ${details.actual_total_cltv_delta}; accepted maximum ${details.max_total_cltv_delta}`;
    case "route_constraints":
      return `${details.leg}: required channels ${details.required.required_channels.join(", ")}`;
    default:
      return `Validation context: ${details.kind}`;
  }
}
