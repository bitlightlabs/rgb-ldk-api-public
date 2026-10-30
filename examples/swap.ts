import { HttpError, RgbLdkNodeClient } from "@bitlightlabs/rgbldk-node-client";
import type {
  SwapCreateMultihopOfferRequest,
  SwapExecuteRequest,
  SwapExecuteResponse,
  SwapInfoDto,
  SwapOfferResponse,
  SwapStringRequest,
} from "@bitlightlabs/rgbldk-node-api-types";

/** The application supplies real nodes, channels, a contract, amounts, and accepted fee limits. */
export async function createQuote(
  maker: RgbLdkNodeClient,
  request: SwapCreateMultihopOfferRequest,
  saveExecutionId: (id: string) => void,
) {
  const offer = await maker.postJson<SwapCreateMultihopOfferRequest, SwapOfferResponse>(
    "/api/v1/swap/offers/multihop",
    request,
  );
  saveExecutionId(offer.execution_id);
  return offer;
}

/** Run against the taker's node. The callback presents the terms for explicit confirmation. */
export async function previewAndAccept(
  taker: RgbLdkNodeClient,
  swapString: string,
  confirmTerms: (preview: SwapInfoDto) => Promise<boolean>,
) {
  const request: SwapStringRequest = { swap_string: swapString };
  const preview = await taker.postJson<SwapStringRequest, SwapInfoDto>(
    "/api/v1/swap/decode",
    request,
  );

  if (!(await confirmTerms(preview))) {
    return undefined;
  }

  return taker.postJson<SwapStringRequest, SwapInfoDto>("/api/v1/swap/accept", request);
}

/** Initial submission only. A lost acceptance notification must be resolved separately. */
export async function executeAccepted(maker: RgbLdkNodeClient, executionId: string) {
  const current = await maker.getJson<SwapInfoDto>(
    `/api/v1/swap/${encodeURIComponent(executionId)}`,
  );

  if (current.status !== "Accepted") {
    throw new Error(`Expected Accepted; current state is ${current.status}`);
  }

  return maker.postJson<SwapExecuteRequest, SwapExecuteResponse>("/api/v1/swap/execute", {
    execution_id: executionId,
  });
}

/** Polling only reads status. Reaching the polling limit does not establish payment failure. */
export async function waitForOutcome(
  client: RgbLdkNodeClient,
  executionId: string,
  onUpdate: (info: SwapInfoDto) => void,
  signal: AbortSignal,
  maxPolls = 60,
): Promise<SwapInfoDto> {
  for (let attempt = 0; attempt < maxPolls; attempt += 1) {
    signal.throwIfAborted();
    const current = await client.getJson<SwapInfoDto>(
      `/api/v1/swap/${encodeURIComponent(executionId)}`,
    );
    onUpdate(current);

    if (current.status === "Settled" || current.status === "Failed") {
      return current;
    }

    await new Promise<void>((resolve) => setTimeout(resolve, 1000));
  }

  throw new Error(`Swap ${executionId} is still pending; retain its ID and query again`);
}

/** Invoke explicitly after reconnecting/restarting; do not put this in a polling loop. */
export async function recoverPrepared(maker: RgbLdkNodeClient, executionId: string) {
  const recordPath = `/api/v1/swap/${encodeURIComponent(executionId)}`;
  const current = await maker.getJson<SwapInfoDto>(recordPath);
  const latest = current.execution?.attempts.at(-1);

  if (
    current.role === "Maker" &&
    current.status !== "Settled" &&
    current.status !== "Failed" &&
    (latest?.status === "Prepared" || latest?.status === "Submitted")
  ) {
    // Keep the identifier and routes. The server checks saved and recent payment results.
    await maker.postJson<SwapExecuteRequest, SwapExecuteResponse>("/api/v1/swap/execute", {
      execution_id: executionId,
    });

    return maker.getJson<SwapInfoDto>(recordPath);
  }

  return current;
}

export function planSummary(info: SwapInfoDto) {
  const execution = info.execution;
  const selectedPlan = execution?.selected_plan;

  if (!execution || selectedPlan == null) {
    return undefined; // No execution plan exists yet; display the quote's fee limits.
  }

  const plan = execution.plans.find((candidate) => candidate.version === selectedPlan);

  if (!plan) {
    return undefined;
  }

  const attempt = execution.attempts
    .filter((candidate) => candidate.plan_version === selectedPlan)
    .at(-1);
  const fees = BigInt(plan.maker_to_taker.fee_msat) + BigInt(plan.taker_to_maker.fee_msat);

  return {
    version: plan.version,
    formatVersion: plan.format_version,
    attemptStatus: attempt?.status,
    feePayer: plan.maker_to_taker.fee_payer,
    totalFeeMsat: fees.toString(),
    policySnapshotAt: plan.policy_snapshot_at_unix_secs,
    legs: [plan.maker_to_taker, plan.taker_to_maker],
  };
}

export function nodeErrorCode(error: unknown): string | undefined {
  if (!(error instanceof HttpError)) {
    return undefined;
  }

  const body = error.body;

  if (typeof body === "object" && body !== null && "error" in body && typeof body.error === "string") {
    return body.error;
  }

  return undefined;
}
