# Endpoint List (Prefix: `/api/v1`)

*This file is exported by `scripts/rgbldk_api_sync.py`.*

### GET `/api/v1/balances`

* **Summary:** List balances
* **Description:** Returns the aggregated BTC and RGB balances currently tracked by the node. For the freshest wallet/runtime view, callers typically run `POST /wallet/sync` and `POST /rgb/sync` first.
* **Response (200):** `BalancesDto`

### POST `/api/v1/bolt11/claim_for_hash`

* **Summary:** Claim held payment
* **Description:** Claims a held BOLT11 payment using the original payment hash and preimage.
* **Request Body:** `Bolt11ClaimForHashRequest`
* **Response (200):** `OkResponse`

### POST `/api/v1/bolt11/decode`

* **Summary:** Decode BOLT11 invoice
* **Description:** Parses a BOLT11 invoice string and returns a normalized summary.
* **Request Body:** `Bolt11DecodeRequest`
* **Response (200):** `Bolt11DecodeResponse`

### POST `/api/v1/bolt11/fail_for_hash`

* **Summary:** Fail held payment
* **Description:** Fails a held BOLT11 payment identified by payment hash.
* **Request Body:** `Bolt11FailForHashRequest`
* **Response (200):** `OkResponse`

### POST `/api/v1/bolt11/pay`

* **Summary:** Pay BOLT11 invoice
* **Description:** Pays a BOLT11 invoice and waits for terminal completion, returning the preimage on success.
* **Request Body:** `Bolt11PayRequest`
* **Response (200):** `Bolt11PayResponse`
* **Response (408):** Timed out while waiting for payment completion

### POST `/api/v1/bolt11/receive`

* **Summary:** Create BOLT11 invoice
* **Description:** Creates a fixed-amount BOLT11 invoice.
* **Request Body:** `Bolt11ReceiveRequest`
* **Response (200):** `Bolt11ReceiveResponse`

### POST `/api/v1/bolt11/receive_for_hash`

* **Summary:** Create hold invoice
* **Description:** Creates a fixed-amount BOLT11 invoice bound to an explicit payment hash.
* **Request Body:** `Bolt11ReceiveForHashRequest`
* **Response (200):** `Bolt11ReceiveResponse`

### POST `/api/v1/bolt11/receive_var`

* **Summary:** Create variable-amount invoice
* **Description:** Creates a BOLT11 invoice whose amount is chosen by the payer.
* **Request Body:** `Bolt11ReceiveVarRequest`
* **Response (200):** `Bolt11ReceiveResponse`

### POST `/api/v1/bolt11/send`

* **Summary:** Send BOLT11 payment
* **Description:** Initiates payment for a fixed-amount BOLT11 invoice.
* **Request Body:** `Bolt11SendRequest`
* **Response (200):** `SendResponse`

### POST `/api/v1/bolt11/send_using_amount`

* **Summary:** Send variable-amount BOLT11 payment
* **Description:** Initiates payment for a BOLT11 invoice using an explicit amount.
* **Request Body:** `Bolt11SendUsingAmountRequest`
* **Response (200):** `SendResponse`

### POST `/api/v1/bolt12/offer/decode`

* **Summary:** Decode BOLT12 offer
* **Description:** Parses a BOLT12 offer and returns a normalized summary.
* **Request Body:** `Bolt12OfferDecodeRequest`
* **Response (200):** `Bolt12OfferDecodeResponse`

### POST `/api/v1/bolt12/offer/receive`

* **Summary:** Create BOLT12 offer
* **Description:** Creates a fixed-amount BOLT12 offer.
* **Request Body:** `Bolt12OfferReceiveRequest`
* **Response (200):** `Bolt12OfferResponse`

### POST `/api/v1/bolt12/offer/receive_var`

* **Summary:** Create variable-amount BOLT12 offer
* **Description:** Creates a BOLT12 offer whose amount is chosen by the payer.
* **Request Body:** `Bolt12OfferReceiveVarRequest`
* **Response (200):** `Bolt12OfferResponse`

### POST `/api/v1/bolt12/offer/send`

* **Summary:** Send to BOLT12 offer
* **Description:** Initiates payment to a BOLT12 offer.
* **Request Body:** `Bolt12OfferSendRequest`
* **Response (200):** `SendResponse`

### POST `/api/v1/bolt12/refund/decode`

* **Summary:** Decode BOLT12 refund
* **Description:** Parses a BOLT12 refund and returns a normalized summary.
* **Request Body:** `Bolt12RefundDecodeRequest`
* **Response (200):** `Bolt12RefundDecodeResponse`

### POST `/api/v1/bolt12/refund/initiate`

* **Summary:** Initiate BOLT12 refund
* **Description:** Creates a BOLT12 refund object and returns the associated payment id.
* **Request Body:** `Bolt12RefundInitiateRequest`
* **Response (200):** `Bolt12RefundInitiateResponse`

### POST `/api/v1/bolt12/refund/request_payment`

* **Summary:** Request payment for refund
* **Description:** Builds a BOLT12 invoice from a refund and returns both bech32 and hex forms.
* **Request Body:** `Bolt12RefundRequestPaymentRequest`
* **Response (200):** `Bolt12RefundRequestPaymentResponse`

### POST `/api/v1/channel/close`

* **Summary:** Close channel
* **Description:** Requests a cooperative close for a channel.
* **Request Body:** `CloseChannelRequest`
* **Response (200):** `OkResponse`

### POST `/api/v1/channel/force_close`

* **Summary:** Force-close channel
* **Description:** Force-closes a channel when a cooperative close is not desired or possible.
* **Request Body:** `CloseChannelRequest`
* **Response (200):** `OkResponse`

### POST `/api/v1/channel/open`

* **Summary:** Open channel
* **Description:** Opens a new Lightning channel, optionally with an RGB allocation.
* **Request Body:** `OpenChannelRequest`
* **Response (200):** `OpenChannelResponse`

### POST `/api/v1/channel/splice_in`

* **Summary:** Splice in funds (experimental)
* **Description:** Adds additional funds from the on-chain wallet into an existing channel without closing it. Pure BTC channels only — RGB asset channels are not yet supported.
* **Request Body:** `SpliceInRequest`
* **Response (200):** `OkResponse`

### POST `/api/v1/channel/splice_out`

* **Summary:** Splice out funds (experimental)
* **Description:** Withdraws funds from an existing channel to an on-chain address without closing it. Pure BTC channels only — RGB asset channels are not yet supported.
* **Request Body:** `SpliceOutRequest`
* **Response (200):** `OkResponse`

### GET `/api/v1/channels`

* **Summary:** List channels
* **Description:** Returns the current Lightning channels with extended BTC and RGB balance details.
* **Response (200):** `ChannelDetailsExtendedDto[]`

### POST `/api/v1/events/handled`

* **Summary:** Mark event handled
* **Description:** Marks the most recently delivered event as handled so processing can continue.
* **Response (200):** `OkResponse`

### POST `/api/v1/events/wait_next`

* **Summary:** Wait for next event
* **Description:** Blocks until the next node event becomes available and returns it as a DTO.
* **Response (200):** `EventDto`

### GET `/api/v1/healthz`

* **Summary:** Health check
* **Description:** Returns a basic liveness probe for the HTTP server itself.
* **Response (200):** `OkResponse`

### GET `/api/v1/listening_addresses`

* **Summary:** Listening addresses
* **Description:** Returns the list of socket addresses currently advertised by the node.
* **Response (200):** `ListeningAddressesResponse`

### GET `/api/v1/network_graph/channel/{scid}`

* **Summary:** Get info for a channel by short channel ID
* **Response (200):** `NetworkGraphChannelInfoResponse`
* **Response (400):** Invalid scid
* **Response (404):** Channel not found in graph

### GET `/api/v1/network_graph/channels`

* **Summary:** List all short channel IDs in the network graph
* **Response (200):** `NetworkGraphChannelsResponse`

### GET `/api/v1/network_graph/node/{node_id_hex}`

* **Summary:** Get info for a node by hex node_id
* **Response (200):** `NetworkGraphNodeInfoResponse`
* **Response (400):** Invalid node_id_hex
* **Response (404):** Node not found in graph

### GET `/api/v1/network_graph/nodes`

* **Summary:** List all node IDs in the network graph
* **Response (200):** `NetworkGraphNodesResponse`

### GET `/api/v1/node_id`

* **Summary:** Node public key
* **Description:** Returns the node's public key in hex format.
* **Response (200):** `NodeIdResponse`

### POST `/api/v1/payment/bolt12/async/blinded_paths_for_recipient`

* **Summary:** Compute blinded paths for an async recipient (experimental)
* **Description:** Returns BlindedMessagePaths for an async recipient identified by `recipient_id_hex`. Requires the node to be running in async-payments Server role.
* **Request Body:** `AsyncBlindedPathsRequest`
* **Response (200):** `AsyncBlindedPathsResponse`
* **Response (400):** Invalid recipient_id_hex or other failure
* **Response (503):** Node is not in async-payments Server role

### POST `/api/v1/payment/bolt12/async/receive_offer`

* **Summary:** Get an async-payment offer (experimental)
* **Description:** Returns a BOLT-12 offer that a static invoice server can serve invoices for on this node's behalf when offline. Available on any node; the offer is only produced after `set_static_invoice_server_paths` has been called and the static-invoice handshake completes.
* **Response (200):** `AsyncReceiveOfferResponse`
* **Response (400):** Offer not yet available (handshake incomplete)

### POST `/api/v1/payment/bolt12/async/set_static_invoice_server_paths`

* **Summary:** Configure paths to a static-invoice server (experimental)
* **Description:** Sets the BlindedMessagePaths used to interactively build offers with a static-invoice server. `paths_hex` is the Writeable serialization of `Vec<BlindedMessagePath>`, hex-encoded. Available on any node.
* **Request Body:** `AsyncSetStaticInvoiceServerPathsRequest`
* **Response (200):** `OkResponse`
* **Response (400):** Invalid hex or path encoding

### GET `/api/v1/payment/{payment_id}`

* **Summary:** Get payment
* **Description:** Returns the current state and details of a single payment by id.
* **Response (200):** `PaymentDetailsDto`

### POST `/api/v1/payment/{payment_id}/abandon`

* **Summary:** Abandon payment
* **Description:** Stops tracking a payment locally.
* **Response (200):** `OkResponse`

### POST `/api/v1/payment/{payment_id}/wait`

* **Summary:** Wait for payment completion
* **Description:** Waits until the payment reaches a terminal state or the timeout elapses.
* **Request Body:** `PaymentWaitRequest`
* **Response (200):** `PaymentWaitResponse`
* **Response (408):** Timed out while waiting for the payment

### GET `/api/v1/payments`

* **Summary:** List payments
* **Description:** Returns all known payments tracked by the node.
* **Response (200):** `PaymentDetailsDto[]`

### GET `/api/v1/peers`

* **Summary:** List peers
* **Description:** Returns the known peers together with connection and persistence state.
* **Response (200):** `PeerDetailsDto[]`

### POST `/api/v1/peers/connect`

* **Summary:** Connect peer
* **Description:** Connects to a peer and optionally persists it in the peer store.
* **Request Body:** `PeerConnectRequest`
* **Response (200):** `OkResponse`

### POST `/api/v1/peers/disconnect`

* **Summary:** Disconnect peer
* **Description:** Disconnects a currently known peer.
* **Request Body:** `PeerDisconnectRequest`
* **Response (200):** `OkResponse`

### GET `/api/v1/readyz`

* **Summary:** Readiness check
* **Description:** Returns readiness details for the node runtime and P2P listener.
* **Response (200):** `OkResponse`
* **Response (503):** `OkResponse`

### GET `/api/v1/rgb/consignments/{consignment_key}`

* **Summary:** Download consignment
* **Description:** Returns a previously stored consignment by key. This is the follow-up download endpoint for `consignment_key` values returned by RGB export/send operations. The response body is binary and can be encoded as raw, gzip, or zip.
* **Response (200):** `binary`

### GET `/api/v1/rgb/contract/{contract_id}/balance`

* **Summary:** Get RGB contract balance
* **Description:** Returns the aggregated L1 balance view for a single RGB contract. For the freshest view after transfers or confirmations, call `POST /rgb/sync` first.
* **Response (200):** `RgbContractBalanceResponse`

### GET `/api/v1/rgb/contract/{contract_id}/known`

* **Summary:** Check RGB contract known
* **Description:** Returns whether the node currently knows a given RGB contract id. This is a lightweight public probe endpoint useful before attempting imports, transfers, or UI contract lookups.
* **Response (200):** `RgbContractKnownResponse`

### GET `/api/v1/rgb/contracts`

* **Summary:** List RGB contracts
* **Description:** Returns RGB contracts currently known to the local RGB runtime/stockpile together with best-effort metadata. This is the catalog view to inspect after imports, issuance, or runtime sync.
* **Response (200):** `RgbContractsResponse`

### POST `/api/v1/rgb/contracts/export`

* **Summary:** Export RGB contract
* **Description:** Exports a contract consignment for transfer or archival. By default it returns JSON containing a stored `consignment_key`; when `download=true` it streams the encoded bytes directly so they can be forwarded or saved immediately.
* **Request Body:** `RgbContractsExportRequest`
* **Response (200):** `RgbContractsExportResponse`

### POST `/api/v1/rgb/contracts/import`

* **Summary:** Import RGB contract
* **Description:** Imports a contract consignment from the raw request body. Use this with bytes produced by `/rgb/contracts/export?download=true` or `/rgb/consignments/{consignment_key}`. The body may be raw, gzip, or zip depending on the `format` query parameter.
* **Request Body:** `binary`
* **Response (200):** `RgbContractsImportResponse`

### POST `/api/v1/rgb/contracts/issue`

* **Summary:** Issue RGB contract
* **Description:** Issues a new RGB contract using an imported issuer plus an RGB-wallet UTXO. Typical flow: `POST /rgb/issuers/import` -> fund the RGB wallet -> `POST /rgb/sync` -> `POST /rgb/contracts/issue`. If `utxo` is omitted, the node auto-selects a suitable RGB-wallet outpoint.
* **Request Body:** `RgbContractsIssueRequest`
* **Response (200):** `RgbContractsIssueResponse`

### GET `/api/v1/rgb/descriptor`

* **Summary:** Get RGB wallet descriptor
* **Description:** Returns the public RGB wallet root descriptor plus derived public descriptors used by address/signing workflows. This is primarily for audits, interoperability, or descriptor inspection; it does not spend funds or change wallet state.
* **Response (200):** `RgbDescriptorResponse`

### GET `/api/v1/rgb/issuers`

* **Summary:** List RGB issuers
* **Description:** Lists issuer files currently present in the local issuer registry. These names are used by `POST /rgb/contracts/issue`. Corrupt or unloadable issuer files are returned separately in `invalid_issuers` so callers can surface cleanup actions.
* **Response (200):** `RgbIssuersResponse`

### POST `/api/v1/rgb/issuers/import`

* **Summary:** Import RGB issuer
* **Description:** Imports an issuer archive into the local issuer registry. Typical flow before contract issuance: `POST /rgb/issuers/import` -> `POST /rgb/contracts/issue`. The raw request body may be a raw `.issuer` file or a gzip/zip archive carrying one issuer entry, selected via the `format` query parameter.
* **Request Body:** `binary`
* **Response (200):** `RgbIssuersImportResponse`

### POST `/api/v1/rgb/ln/invoice/create`

* **Summary:** Create RGB Lightning invoice
* **Description:** Creates an RGB-aware Lightning invoice using a contract id, asset amount, and BTC carrier amount. The invoice embeds both the RGB asset data and the BTC carrier value required to keep the RGB state spendable on Lightning.
* **Request Body:** `RgbLnInvoiceCreateRequest`
* **Response (200):** `RgbLnInvoiceResponse`

### POST `/api/v1/rgb/ln/invoice/create_for_hash`

* **Summary:** Create RGB hold invoice
* **Description:** Creates an RGB-aware hold invoice bound to an explicit payment hash. Use this when the caller must control the payment hash while still embedding RGB asset data and the BTC carrier amount.
* **Request Body:** `RgbLnInvoiceCreateForHashRequest`
* **Response (200):** `RgbLnInvoiceResponse`

### POST `/api/v1/rgb/ln/invoice/decode`

* **Summary:** Decode RGB Lightning invoice
* **Description:** Decodes an RGB-aware Lightning invoice and returns both BTC carrier fields and embedded RGB asset fields. Use this before `POST /rgb/ln/pay` when the caller needs to know whether the invoice already carries RGB metadata or whether explicit contract/asset fields must be supplied.
* **Request Body:** `RgbLnInvoiceDecodeRequest`
* **Response (200):** `RgbLnInvoiceDecodeResponse`

### POST `/api/v1/rgb/ln/pay`

* **Summary:** Pay RGB Lightning invoice
* **Description:** Pays an RGB Lightning invoice. When the invoice already embeds RGB fields, the request can contain just the invoice. When it does not, callers must also supply `contract_id` and `asset_amount`. The BTC carrier amount inside the invoice must still satisfy the RGB minimum carrier requirement.
* **Request Body:** `RgbLnPayRequest`
* **Response (200):** `SendResponse`

### POST `/api/v1/rgb/new_address`

* **Summary:** New RGB address
* **Description:** Generates a new address owned by the dedicated RGB wallet descriptor. Use this for `/rgb/utxos/fund`, `/rgb/utxos/top_up`, RGB invoice beneficiaries, and other outputs that should later appear in `/rgb/utxos` after `POST /rgb/sync`. Do not use it for ordinary BTC change; use `POST /wallet/new_address` for that.
* **Response (200):** `RgbNewAddressResponse`

### POST `/api/v1/rgb/onchain/invoice/create`

* **Summary:** Create RGB on-chain invoice
* **Description:** Creates an RGB on-chain invoice using either witness-out or blinded beneficiary mode. Set `use_witness_utxo=true` when you want an explicit witness-output beneficiary in the invoice. Set `use_witness_utxo=false` for blinded invoices; if `blinding_utxo` is omitted in that mode, the node auto-selects an available RGB-wallet outpoint, so callers should usually run `POST /rgb/sync` first.
* **Request Body:** `RgbOnchainInvoiceCreateRequest`
* **Response (200):** `RgbOnchainInvoiceResponse`

### POST `/api/v1/rgb/onchain/invoice/decode`

* **Summary:** Decode RGB on-chain invoice
* **Description:** Parses an RGB on-chain invoice and returns beneficiary, amount, and expiry details. Use this before send/receive flows when a caller needs to inspect whether the invoice uses witness-out or blinded beneficiary mode.
* **Request Body:** `RgbOnchainInvoiceDecodeRequest`
* **Response (200):** `RgbOnchainInvoiceDecodeResponse`

### GET `/api/v1/rgb/onchain/payments`

* **Summary:** List RGB on-chain payments
* **Description:** Returns RGB on-chain payment history, optionally filtered by contract id. This is the lifecycle log for invoices, sends, receives, and associated consignment keys/download paths.
* **Response (200):** `RgbOnchainPaymentsResponse`

### POST `/api/v1/rgb/onchain/receive`

* **Summary:** Receive RGB on-chain payment
* **Description:** Accepts an RGB on-chain consignment into the local wallet. JSON mode is best when the caller already has a `consignment_key` plus either `invoice` or `payment_id`. Binary mode is for raw/gzip/zip consignment uploads and requires `payment_id` in the query string so the node can map the upload to an existing payment record. When the invoice was created elsewhere, prefer JSON mode and pass the full invoice string.
* **Request Body:** `RgbOnchainReceiveRequest`
* **Response (200):** `RgbOnchainReceiveResponse`

### POST `/api/v1/rgb/onchain/send`

* **Summary:** Send RGB on-chain payment
* **Description:** Builds and broadcasts an RGB on-chain payment transaction for an invoice. Use this for contract-bearing RGB transfers, not for empty RGB-UTXO lifecycle management (`/rgb/utxos/fund`, `/rgb/utxos/top_up`, `/rgb/utxos/sweep`). The response returns a `consignment_key`; receivers or follow-up tooling typically need that consignment to complete the transfer.
* **Request Body:** `RgbOnchainSendRequest`
* **Response (200):** `RgbOnchainSendResponse`

### POST `/api/v1/rgb/sign_message`

* **Summary:** Sign message with RGB wallet key
* **Description:** Signs an arbitrary message using the RGB wallet descriptor key. `bitcoin_signed_message` uses the request `message` string directly. `ecdsa` interprets the request `message` as bytes encoded with the requested `encoding`, then signs the SHA-256 digest of those bytes. The response `signature` uses the same `encoding`.
* **Request Body:** `RgbSignMessageRequest`
* **Response (200):** `RgbSignMessageResponse`

### POST `/api/v1/rgb/sync`

* **Summary:** Sync RGB runtime
* **Description:** Synchronizes the RGB runtime/stockpile with the configured chain data and then refreshes the node's ordinary wallet view without triggering a second RGB refresh. Call this after funding the RGB wallet, importing or issuing contracts, broadcasting RGB on-chain operations, or when confirmations change and you want `/rgb/utxos`, `/rgb/contracts`, and RGB balances to reflect the latest state.
* **Response (200):** `OkResponse`

### GET `/api/v1/rgb/utxos`

* **Summary:** List RGB UTXOs
* **Description:** Returns the canonical RGB spend-domain outpoint view, including RGB allocations, semantic spend roles, and txoscope-backed lock metadata. Newly created or newly confirmed RGB outputs usually require `POST /rgb/sync` before they appear here. By default this uses a fast cached view; pass `refresh=true` to reconcile txoscope state and refresh confirmation heights for already-known RGB outpoints.
* **Response (200):** `RgbUtxosResponse`

### POST `/api/v1/rgb/utxos/fund`

* **Summary:** Fund RGB UTXOs
* **Description:** Low-level, stateless primitive that converts exact ordinary BTC-wallet inputs into one or more empty RGB-wallet outputs. Typical flow: `POST /wallet/sync` -> `GET /wallet/utxos` (pick exact inputs) -> `POST /rgb/new_address` (one per RGB output) -> `POST /wallet/new_address` (change) -> `POST /rgb/utxos/fund` -> `POST /rgb/sync`. The handler does not auto-select inputs or change outputs.
* **Request Body:** `RgbUtxosFundRequest`
* **Response (200):** `RgbUtxosFundResponse`

### POST `/api/v1/rgb/utxos/release`

* **Summary:** Release RGB UTXO reservation
* **Description:** Clears a manual reservation previously created by `POST /rgb/utxos/reserve`. Prefer releasing by `reservation_id`; `outpoint` is a convenience fallback when the id is unavailable.
* **Request Body:** `RgbUtxosReleaseRequest`
* **Response (200):** `RgbUtxosReleaseResponse`

### POST `/api/v1/rgb/utxos/reserve`

* **Summary:** Reserve RGB UTXO
* **Description:** Temporarily marks an RGB-wallet outpoint unavailable for other flows. Use this around multi-step caller-driven sequences when you first select an outpoint from `GET /rgb/utxos` and need it to stay stable until a later operation. If `outpoint` is omitted, the node auto-selects one available RGB-wallet UTXO.
* **Request Body:** `RgbUtxosReserveRequest`
* **Response (200):** `RgbUtxosReserveResponse`

### GET `/api/v1/rgb/utxos/summary`

* **Summary:** Summarize RGB UTXOs
* **Description:** Deprecated compatibility projection. Use `GET /rgb/utxos` for the canonical RGB UTXO view with allocations, spend roles, and lock metadata. By default this uses a fast cached view; pass `refresh=true` to reconcile txoscope state and refresh confirmation heights for already-known RGB outpoints.
* **Response (200):** `RgbUtxosSummaryResponse`

### POST `/api/v1/rgb/utxos/sweep`

* **Summary:** Sweep empty RGB UTXO
* **Description:** Low-level primitive that spends one empty, confirmed, unlocked RGB-wallet output back to an ordinary BTC-wallet address. Use this only for RGB outpoints with no allocations; otherwise use normal RGB payment flows or `/rgb/utxos/top_up`. Typical flow: `POST /rgb/sync` -> `GET /rgb/utxos` -> `POST /wallet/new_address` -> `POST /rgb/utxos/sweep`.
* **Request Body:** `RgbUtxosSweepRequest`
* **Response (200):** `RgbUtxosSweepResponse`

### POST `/api/v1/rgb/utxos/top_up`

* **Summary:** Increase RGB UTXO capacity
* **Description:** Low-level primitive that replaces one confirmed, unlocked RGB UTXO with a larger RGB-wallet output while preserving its anchored single-contract RGB state. Typical flow: `POST /rgb/sync` -> `GET /rgb/utxos` (pick the RGB input) -> `POST /wallet/sync` + `GET /wallet/utxos` (pick extra BTC inputs) -> `POST /rgb/new_address` (replacement output) -> `POST /wallet/new_address` (change) -> `POST /rgb/utxos/top_up`. The response includes a `consignment_key` for the replacement transfer.
* **Request Body:** `RgbUtxosTopUpRequest`
* **Response (200):** `RgbUtxosTopUpResponse`

### POST `/api/v1/spontaneous/send`

* **Summary:** Send spontaneous payment
* **Description:** Sends a keysend payment, optionally with custom TLV records.
* **Request Body:** `SpontaneousSendRequest`
* **Response (200):** `SendResponse`

### GET `/api/v1/status`

* **Summary:** Node status
* **Description:** Returns the current runtime, listener, and best-block status of the node.
* **Response (200):** `StatusDto`

### GET `/api/v1/version`

* **Summary:** API version
* **Description:** Returns HTTP API versioning metadata for compatibility checks.
* **Response (200):** `VersionResponse`

### POST `/api/v1/wallet/new_address`

* **Summary:** New on-chain address
* **Description:** Generates a new receive/change address from the ordinary BTC wallet account. Use this for plain BTC funding, explicit change on RGB UTXO-management calls, and outputs that should later appear in `/wallet/utxos`. Do not use it for RGB-owned outputs; use `POST /rgb/new_address` for those.
* **Response (200):** `WalletNewAddressResponse`

### POST `/api/v1/wallet/sync`

* **Summary:** Sync wallet
* **Description:** Synchronizes the ordinary BTC wallet with the configured chain source and persists newly observed wallet outputs. Call this before selecting inputs from `GET /wallet/utxos`, or before expecting fresh receive/change outputs to appear in the ordinary wallet view.
* **Response (200):** `OkResponse`

### GET `/api/v1/wallet/utxos`

* **Summary:** List ordinary L1 wallet UTXOs
* **Description:** Returns only ordinary BTC-account outpoints according to txoscope classification, together with txoscope-backed lock metadata. Newly discovered wallet outputs appear here only after `POST /wallet/sync`. Pass `refresh=true` to reconcile txoscope state and refresh confirmation heights for already-known outputs; it does not discover new wallet outputs on its own.
* **Response (200):** `WalletUtxosResponse`
