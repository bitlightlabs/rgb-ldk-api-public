export * from "./generated.js";
import type { U64String } from "./generated.js";

export function u64ToBigInt(v: U64String): bigint {
  return BigInt(v);
}

export function bigIntToU64String(v: bigint): U64String {
  if (v < 0n) throw new Error("u64 must be non-negative");
  return v.toString(10);
}

export type U64Input = U64String | string | number | bigint;

export function u64(input: U64Input): U64String {
  if (typeof input === "bigint") return bigIntToU64String(input);
  if (typeof input === "number") {
    if (!Number.isFinite(input) || !Number.isInteger(input)) throw new Error(`invalid u64: ${input}`);
    if (input < 0) throw new Error(`u64 must be non-negative: ${input}`);
    return String(input);
  }
  const s = String(input).trim();
  if (!/^\d+$/.test(s)) throw new Error(`invalid u64: ${input}`);
  return s.replace(/^0+(?=\d)/, "");
}
