# Protocol-neutral UTXO status migration

The protocol-neutral UTXO status update changes the public enum vocabulary. This
is a wire compatibility change, even though the DTO names and field names remain
the same.

| Public enum | Previous value | Updated value |
| --- | --- | --- |
| `WalletUtxoAvailabilityDto` | `blocked_by_l2` / Rust `BlockedByL2` | `blocked_by_spend` / Rust `BlockedBySpend` |
| `WalletUtxoOperationPhaseDto` | `awaiting_l2_finality` / Rust `AwaitingL2Finality` | Removed; no direct replacement |

Update Rust variant references and TypeScript comparisons or exhaustive switches
for the renamed availability value. Remove assumptions that the operation phase
can report L2 finality. Applications that need protocol-specific finality must
obtain it from the relevant protocol API rather than infer it from this field.

Coordinate server and client upgrades: old Rust DTOs cannot deserialize the new
availability value, and the updated DTOs no longer accept either removed value.
TypeScript unions describe the new contract but do not validate incoming JSON at
runtime. Consumers of cached responses or mixed server versions must account for
the vocabulary change at their own compatibility boundary.

The API mirror derives these enums from the node's exported OpenAPI schemas. No
compatibility aliases, fallback mapping, or independently maintained variant list
are added to the generator. The final sync must record the matching node commit
in `generated/spec/source.json`.
