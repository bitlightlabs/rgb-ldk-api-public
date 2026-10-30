import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
import { HttpError, RgbLdkNodeClient } from "../packages/rgbldk-node-client/dist/index.js";
import { u64 } from "../packages/rgbldk-node-api-types/dist/index.js";

const [info] = JSON.parse(await readFile(new URL("./fixtures/swap-executions.json", import.meta.url)));
const id = info.execution_id;

test("u64 conversion preserves large channel identifiers and rejects unsafe values", () => {
  assert.equal(u64("9007199254740993"), "9007199254740993");
  assert.equal(u64(18_446_744_073_709_551_615n), "18446744073709551615");
  assert.equal(u64("00042"), "42");
  assert.throws(() => u64(Number("9007199254740993")), /unsafe u64 number/);
  assert.throws(() => u64("18446744073709551616"), /u64 must be between/);
  assert.throws(() => u64(-1n), /u64 must be between/);
});

function clientWith(response) {
  const calls = [];
  const client = new RgbLdkNodeClient({
    baseUrl: "https://node.example/",
    token: { kind: "fixed", token: "test-token" },
    fetchImpl: async (url, init) => {
      calls.push({ url, ...init, body: init.body ? JSON.parse(init.body) : undefined });
      return response();
    },
  });
  return { client, calls };
}

test("POST preserves the execution identifier and the submission status", async () => {
  const { client, calls } = clientWith(() => Response.json({
    execution_id: id, payment_hash: id, ok: true, status: "InFlight",
  }));
  const result = await client.postJson("/api/v1/swap/execute", { execution_id: id });
  assert.equal(result.status, "InFlight");
  assert.equal(calls.length, 1);
  assert.equal(calls[0].url, "https://node.example/api/v1/swap/execute");
  assert.deepEqual(calls[0].body, { execution_id: id });
  assert.equal(calls[0].headers.Authorization, "Bearer test-token");
});

test("POST preserves both replacement routes and large channel identifiers", async () => {
  const { client, calls } = clientWith(() => Response.json({ ok: true }));
  const { maker_to_taker, taker_to_maker } = info.execution;
  await client.postJson("/api/v1/swap/execute", {
    execution_id: id,
    maker_to_taker,
    taker_to_maker,
    policy_hints: [],
  });
  assert.equal(calls[0].body.maker_to_taker[0].channel_scid, "9007199254740993");
  assert.deepEqual(calls[0].body.taker_to_maker, taker_to_maker);
  assert.equal(calls[0].body.execution_id, id);
});

test("quote requests preserve their fields and the original quote string", async () => {
  const { client, calls } = clientWith(() => Response.json(info));
  const terms = {
    contract_id: info.contract_id,
    asset_amount: info.asset_amount,
    btc_amount_msat: info.btc_amount_msat,
    btc_carrier_amount_msat: info.btc_carrier_amount_msat,
    maker_gives_rgb: info.maker_gives_rgb,
    expiry_secs: info.expiry_secs,
  };
  const direct = {
    ...terms, counterparty_node_id: info.counterparty_node_id, channel_scid: info.channel_scid,
  };
  const multihop = {
    ...terms,
    maker_to_taker: info.execution.maker_to_taker,
    taker_to_maker: info.execution.taker_to_maker,
    constraints: info.execution.quote.constraints,
  };
  await client.postJson("/api/v1/swap/offers", direct);
  await client.postJson("/api/v1/swap/offers/multihop", multihop);
  await client.postJson("/api/v1/swap/decode", { swap_string: "rgb-swap:v3:example" });
  await client.postJson("/api/v1/swap/accept", { swap_string: "rgb-swap:v3:example" });
  assert.deepEqual(calls.map(({ url }) => new URL(url).pathname), [
    "/api/v1/swap/offers", "/api/v1/swap/offers/multihop",
    "/api/v1/swap/decode", "/api/v1/swap/accept",
  ]);
  assert.deepEqual(calls[2].body, { swap_string: "rgb-swap:v3:example" });
  assert.deepEqual(calls[3].body, calls[2].body);
});

test("GET preserves complete plans, null outputs and precise amounts", async () => {
  const { client, calls } = clientWith(() => Response.json(info));
  const result = await client.getJson(`/api/v1/swap/${encodeURIComponent(id)}`);
  assert.deepEqual(result, info);
  assert.equal(result.execution.plans[0].taker_to_maker.hops.at(-1).btc_output_msat, null);
  assert.equal(calls[0].method, "GET");
});

test("GET and DELETE use the supplied paths and authenticate cancellation", async () => {
  const { client, calls } = clientWith(() => Response.json({ ok: true }));
  await client.getJson("/api/v1/swap/list");
  await client.deleteJson(`/api/v1/swap/${encodeURIComponent(id)}`);
  assert.equal(calls[0].url, "https://node.example/api/v1/swap/list");
  assert.equal(calls[1].method, "DELETE");
  assert.equal(calls[1].url, `https://node.example/api/v1/swap/${id}`);
  assert.equal(calls[1].headers.Authorization, "Bearer test-token");
  assert.equal(calls[1].body, undefined);
});

test("request failures retain structured errors without automatic resending", async () => {
  const body = { error: "PaymentSendingFailed", message: "outcome unresolved" };
  const { client, calls } = clientWith(() => Response.json(body, { status: 400 }));
  await assert.rejects(client.postJson("/api/v1/swap/execute", { execution_id: id }), (error) => {
    assert.ok(error instanceof HttpError);
    assert.equal(error.status, 400);
    assert.deepEqual(error.body, body);
    return true;
  });
  assert.equal(calls.length, 1);
});

test("connection errors and plain-text server errors remain distinguishable", async () => {
  const { client } = clientWith(() => new Response("upstream unavailable", { status: 502 }));
  await assert.rejects(client.getJson(`/api/v1/swap/${encodeURIComponent(id)}`), (error) => {
    assert.equal(error.status, 502);
    assert.equal(error.body, "upstream unavailable");
    return true;
  });

  let sends = 0;
  const disconnected = new RgbLdkNodeClient({
    baseUrl: "https://node.example",
    fetchImpl: async () => {
      sends += 1;
      throw new TypeError("connection lost");
    },
  });
  await assert.rejects(
    disconnected.postJson("/api/v1/swap/execute", { execution_id: id }),
    TypeError,
  );
  assert.equal(sends, 1);
});
