export * from "./generated.js";
import type { U64String } from "./generated.js";

const MAX_U64 = 18_446_744_073_709_551_615n;

export function u64ToBigInt(v: U64String): bigint {
  return BigInt(v);
}

export function bigIntToU64String(v: bigint): U64String {
  if (v < 0n || v > MAX_U64) {
    throw new Error("u64 must be between 0 and 18446744073709551615");
  }

  return v.toString(10);
}

export type U64Input = U64String | string | number | bigint;

export function u64(input: U64Input): U64String {
  if (typeof input === "bigint") {
    return bigIntToU64String(input);
  }

  if (typeof input === "number") {
    if (!Number.isSafeInteger(input)) {
      throw new Error(`unsafe u64 number: ${input}; use a decimal string or bigint`);
    }

    return bigIntToU64String(BigInt(input));
  }

  const s = String(input).trim();

  if (!/^\d+$/.test(s)) {
    throw new Error(`invalid u64: ${input}`);
  }

  return bigIntToU64String(BigInt(s));
}
