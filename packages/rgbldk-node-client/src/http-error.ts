/** HTTP failure from the node. Transport failures remain the original fetch errors. */
export class HttpError extends Error {
  readonly body: unknown;

  constructor(
    readonly status: number,
    readonly rawBody: string,
  ) {
    super(`HTTP ${status}: ${rawBody}`);
    this.name = "HttpError";

    try {
      this.body = JSON.parse(rawBody);
    } catch {
      this.body = rawBody;
    }
  }
}
