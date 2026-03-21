// Generated. Do not edit.
// Run: `pnpm gen` at repo root.

export type TokenSource =
  | { kind: "none" }
  | { kind: "fixed"; token: string }
  | { kind: "provider"; getToken: () => string | Promise<string> | null | undefined };

export type ClientOptions = {
  baseUrl: string;
  token?: TokenSource;
  fetchImpl?: typeof fetch;
};

export class RgbLdkNodeClient {
  private readonly baseUrl: string;
  private readonly token: TokenSource;
  private readonly fetchImpl: typeof fetch;

  constructor(opts: ClientOptions) {
    this.baseUrl = opts.baseUrl.replace(/\/+$/, "");
    this.token = opts.token ?? { kind: "none" };
    this.fetchImpl = opts.fetchImpl ?? fetch;
  }

  private async authHeader(): Promise<Record<string, string>> {
    if (this.token.kind === "none") return {};
    if (this.token.kind === "fixed") return { Authorization: `Bearer ${this.token.token}` };
    const token = await this.token.getToken();
    if (!token) return {};
    const trimmed = token.trim();
    if (!trimmed) return {};
    return { Authorization: `Bearer ${trimmed}` };
  }

  async getJson<T>(path: string): Promise<T> {
    const url = `${this.baseUrl}${path.startsWith("/") ? "" : "/"}${path}`;
    const headers = await this.authHeader();
    const res = await this.fetchImpl(url, { method: "GET", headers });
    const text = await res.text();
    if (!res.ok) throw new Error(`HTTP ${res.status}: ${text}`);
    return JSON.parse(text) as T;
  }

  async postJson<B, T>(path: string, body: B): Promise<T> {
    const url = `${this.baseUrl}${path.startsWith("/") ? "" : "/"}${path}`;
    const headers = {
      "content-type": "application/json",
      ...(await this.authHeader()),
    };
    const res = await this.fetchImpl(url, { method: "POST", headers, body: JSON.stringify(body) });
    const text = await res.text();
    if (!res.ok) throw new Error(`HTTP ${res.status}: ${text}`);
    return JSON.parse(text) as T;
  }
}

