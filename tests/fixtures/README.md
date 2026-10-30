# Swap response examples used by tests

[`swap-executions.json`](swap-executions.json) contains an example for each asset
direction: the maker supplies RGB or the maker supplies bitcoin. Each execution
plan was built with `application::swap_plan::ExecutionPlan::build`, checked with
`validate`, and serialized by the node implementation at commit
[`5c97e220`](https://github.com/bitlightlabs/rgb-ldk-node/commit/5c97e220489f81a5728121355ce1624fdc4d036e).
The fixtures place the plans in `execution.plans`, point `execution.selected_plan`
at their version, and reference that version from each send attempt.
The surrounding `SwapInfoDto` fields use illustrative values: `contract:fixture`,
the public keys, and the channels do not identify live resources.

These examples cover base and proportional fees, a channel identifier above
JavaScript's safe integer range, the nested object used to identify an RGB asset,
null outputs at the final channel, and the channel policies saved with a plan.
Rust tests check that parsing and serializing the complete responses preserves
their values. Client tests check that reading a response does not convert decimal
strings to numbers or lose precision.

All amounts below are millisatoshis, each worth one thousandth of a satoshi.
Read the sequence as maker sends → taker receives → taker sends → maker receives:

| Asset supplied by maker | Bitcoin amounts | First segment fee | Return segment fee |
| --- | --- | --- | --- |
| RGB | 10,110 → 10,000 → 15,000 → 14,886 | 110 | 114 |
| Bitcoin | 15,115 → 15,000 → 10,000 → 9,891 | 115 | 109 |

Both quotes use a price of 5,000 millisatoshis, an RGB quantity of 100 smallest
units, and a carrier of 10,000 millisatoshis. The carrier is the bitcoin that
accompanies an RGB transfer. Each forwarding policy uses a base fee of 100
millisatoshis and a proportional fee of 1,000 parts per million. The expiry
offsets are `360 → 288 → 216 → 144` blocks along the route.

See the [frontend guide](../../docs/swap-frontend-guide.md) for how these values
relate to quote confirmation, fee display, and execution recovery.
