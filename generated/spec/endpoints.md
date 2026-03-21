# Endpoint List (Prefix: `/api/v1`)

*This file is exported by `scripts/rgbldk_api_sync.py`.*

### POST `/api/v1/bolt11/receive`

* **Purpose:** Creates a fixed-amount Bolt11 invoice.
* **Request Body:** `Bolt11ReceiveRequest`
* **Response (200):** `Bolt11ReceiveResponse`

### POST `/api/v1/bolt11/receive_for_hash`

* **Request Body:** `Bolt11ReceiveForHashRequest`
* **Response (200):** `Bolt11ReceiveResponse`

### POST `/api/v1/bolt11/receive_var`

* **Purpose:** Creates a variable-amount Bolt11 invoice (payer specifies the amount).
* **Request Body:** `Bolt11ReceiveVarRequest`
* **Response (200):** `Bolt11ReceiveResponse`

### POST `/api/v1/bolt11/fail_for_hash`

* **Request Body:** `Bolt11FailForHashRequest`

### POST `/api/v1/bolt11/claim_for_hash`

* **Request Body:** `Bolt11ClaimForHashRequest`

### POST `/api/v1/bolt11/decode`

* **Purpose:** Decodes a Bolt11 invoice into a summary.
* **Request Body:** `Bolt11DecodeRequest`
* **Response (200):** `Bolt11DecodeResponse`

### POST `/api/v1/bolt11/send`

* **Purpose:** Pays a Bolt11 invoice (amount is read from the invoice).
* **Request Body:** `Bolt11SendRequest`
* **Response (200):** `SendResponse`

### POST `/api/v1/bolt11/send_using_amount`

* **Purpose:** Pays a variable-amount invoice with a specified amount.
* **Request Body:** `Bolt11SendUsingAmountRequest`
* **Response (200):** `SendResponse`

### POST `/api/v1/bolt11/pay` (preferred)

* **Purpose:** Pays a Bolt11 invoice and waits for completion (returns `preimage` on success).
* **Request Body:** `Bolt11PayRequest`
* **Response (200):** `Bolt11PayResponse`
* **Errors:**
  * `400 Bad Request`: invalid invoice / missing amount for variable invoice / payment failed
  * `408 Request Timeout`: payment timed out

### POST `/api/v1/bolt12/offer/receive`

* **Request Body:** `Bolt12OfferReceiveRequest`
* **Response (200):** `Bolt12OfferResponse`

### POST `/api/v1/bolt12/offer/receive_var`

* **Request Body:** `Bolt12OfferReceiveVarRequest`
* **Response (200):** `Bolt12OfferResponse`

### POST `/api/v1/bolt12/offer/decode`

* **Request Body:** `Bolt12OfferDecodeRequest`
* **Response (200):** `resp`

### POST `/api/v1/bolt12/offer/send`

* **Request Body:** `Bolt12OfferSendRequest`
* **Response (200):** `SendResponse`

### POST `/api/v1/bolt12/refund/initiate`

* **Request Body:** `Bolt12RefundInitiateRequest`
* **Response (200):** `Bolt12RefundInitiateResponse`

### POST `/api/v1/bolt12/refund/decode`

* **Request Body:** `Bolt12RefundDecodeRequest`
* **Response (200):** `resp`

### POST `/api/v1/bolt12/refund/request_payment`

* **Request Body:** `Bolt12RefundRequestPaymentRequest`
* **Response (200):** `Bolt12RefundRequestPaymentResponse`

### GET `/api/v1/channels`

* **Purpose:** Lists all channels with extended details, including RGB asset balances for RGB-enabled channels.
* **Response (200):** `ChannelDetailsExtendedDto[]`
* **Note:** The `rgb_balance` field is only present for channels that hold RGB assets. For regular BTC-only channels, this field is omitted.

### POST `/api/v1/channel/open`

* **Purpose:** Initiates a channel opening request with a peer.
* **Request Body:** `OpenChannelRequest`
* **Response (200):** `OpenChannelResponse`

### POST `/api/v1/channel/close`

* **Purpose:** Initiates a mutual/cooperative channel closure.
* **Request Body:** `CloseChannelRequest`
* **Response (200):** `OkResponse`

### POST `/api/v1/channel/force_close`

* **Purpose:** Force-closes a channel (used if the peer is offline or uncooperative).
* **Request Body:** `CloseChannelRequest`
* **Response (200):** `OkResponse`

### POST `/api/v1/events/wait_next`

* **Purpose:** **Long polling.** Blocks until a new node event (e.g., payment received) occurs.
* **Response (200):** `EventDto`

### POST `/api/v1/events/handled`

* **Purpose:** Acknowledges the "last event" was processed, allowing the node to advance the event queue.
* **Response (200):** `OkResponse`

### GET `/api/v1/healthz`

* **Purpose:** Process-level health check. Returns `ok=true` as long as the HTTP service is responsive (does not mean the node is ready).
* **Response (200):** `OkResponse`
* **Notes:** `checks[]` explains how `ok` was determined.

### GET `/api/v1/readyz`

* **Purpose:** Readiness check. Returns `200 ok=true` when the node runtime is running; otherwise returns `503 ok=false`.
* **Response (200/503):** `OkResponse`
* **Notes:** `checks[]` includes sub-checks like `node_is_running`, `p2p_is_listening`, and `best_block_height_known`.

### GET `/api/v1/version`

* **Purpose:** Retrieves API and crate versions for compatibility checks.
* **Response (200):** `VersionResponse`

### GET `/api/v1/status`

* **Purpose:** Summary of node status (running state, listening state, and current block height).
* **Response (200):** `StatusDto`

### GET `/api/v1/node_id`

* **Purpose:** Returns the node's public key (used for identification and networking).
* **Response (200):** `NodeIdResponse`

### GET `/api/v1/listening_addresses`

* **Purpose:** Returns the list of P2P listening addresses for this node.
* **Response (200):** `ListeningAddressesResponse`

### POST `/api/v1/spontaneous/send`

* **Purpose:** Sends a spontaneous/keysend payment (no invoice required).
* **Note:** `custom_tlvs[].value_hex` must be a hex string.
* **Request Body:** `SpontaneousSendRequest`
* **Response (200):** `SendResponse`

### GET `/api/v1/payments`

* **Response (200):** `Vec<PaymentDetailsDto`

### GET `/api/v1/payment/:payment_id`

* **Purpose:** Queries details of a specific payment by ID.
* **Path Param:** `payment_id` (64 hex chars).
* **Response (200):** `PaymentDetailsDto`

### POST `/api/v1/payment/:payment_id/wait`

* **Request Body:** `PaymentWaitRequest`
* **Response (200):** `PaymentWaitResponse`

### POST `/api/v1/payment/:payment_id/abandon`

* **Request:** Empty body.
* **Response (200):** `OkResponse`

### GET `/api/v1/peers`

* **Purpose:** Lists known peers and their connection status.
* **Response (200):** `PeerDetailsDto[]`

### POST `/api/v1/peers/connect`

* **Request Body:** `PeerConnectRequest`

### POST `/api/v1/peers/disconnect`

* **Request Body:** `PeerDisconnectRequest`

### GET `/api/v1/rgb/contracts`

* **Response (200):** `RgbContractsResponse`

### POST `/api/v1/rgb/contracts/import`

* **Request:** Empty body.
* **Response (200):** `RgbContractsImportResponse`

### POST `/api/v1/rgb/contracts/issue`

* **Request Body:** `RgbContractsIssueRequest`
* **Response (200):** `RgbContractsIssueResponse`

### POST `/api/v1/rgb/contracts/export`

* **Request Body:** `RgbContractsExportRequest`
* **Response (200):** `RgbContractsExportResponse`

### GET `/api/v1/rgb/consignments/:consignment_key`


### GET `/api/v1/rgb/contract/:contract_id/balance`

* **Response (200):** `RgbContractBalanceResponse`

### GET `/api/v1/rgb/contract/:contract_id/known`

* **Response (200):** `RgbContractKnownResponse`

### GET `/api/v1/rgb/issuers`

* **Response (200):** `RgbIssuersResponse`

### POST `/api/v1/rgb/issuers/import`

* **Request:** Empty body.
* **Response (200):** `RgbIssuersImportResponse`

### POST `/api/v1/rgb/ln/invoice/create`

* **Request Body:** `RgbLnInvoiceCreateRequest`
* **Response (200):** `RgbLnInvoiceResponse`

### POST `/api/v1/rgb/ln/invoice/create_for_hash`

* **Request Body:** `RgbLnInvoiceCreateForHashRequest`
* **Response (200):** `RgbLnInvoiceResponse`

### POST `/api/v1/rgb/ln/invoice/decode`

* **Request Body:** `RgbLnInvoiceDecodeRequest`
* **Response (200):** `RgbLnInvoiceDecodeResponse`

### POST `/api/v1/rgb/ln/pay`

* **Request Body:** `RgbLnPayRequest`
* **Response (200):** `SendResponse`

### POST `/api/v1/rgb/onchain/invoice/create`

* **Request Body:** `RgbOnchainInvoiceCreateRequest`
* **Response (200):** `RgbOnchainInvoiceResponse`

### POST `/api/v1/rgb/onchain/invoice/decode`

* **Request Body:** `RgbOnchainInvoiceDecodeRequest`
* **Response (200):** `RgbOnchainInvoiceDecodeResponse`

### POST `/api/v1/rgb/onchain/send`

* **Request Body:** `RgbOnchainSendRequest`
* **Response (200):** `RgbOnchainSendResponse`

### POST `/api/v1/rgb/onchain/receive`

* **Request:** Empty body.

### GET `/api/v1/rgb/onchain/payments`

* **Response (200):** `RgbOnchainPaymentsResponse`

### POST `/api/v1/rgb/sync`

* **Request:** Empty body.

### POST `/api/v1/rgb/new_address`

* **Request:** Empty body.
* **Response (200):** `RgbNewAddressResponse`

### GET `/api/v1/rgb/utxos`

* **Response (200):** `RgbUtxosResponse`

### GET `/api/v1/rgb/utxos/summary`

* **Response (200):** `RgbUtxosSummaryResponse`

### POST `/api/v1/rgb/utxos/reserve`

* **Request Body:** `RgbUtxosReserveRequest`
* **Response (200):** `RgbUtxosReserveResponse`

### POST `/api/v1/rgb/utxos/release`

* **Request Body:** `RgbUtxosReleaseRequest`
* **Response (200):** `RgbUtxosReleaseResponse`

### POST `/api/v1/wallet/new_address`

* **Purpose:** Generates a new on-chain address for funding the node wallet.
* **Request:** Empty body.
* **Response (200):** `WalletNewAddressResponse`

### POST `/api/v1/wallet/sync`

* **Purpose:** Triggers a wallet synchronization to update on-chain status/UTXOs (and syncs RGB runtime if RGB is enabled).
* **Request:** Empty body.
* **Response (200):** `OkResponse`

### GET `/api/v1/balances`

* **Purpose:** Summarizes BTC balances and RGB balances (L1 and L2).
* **Response (200):** `BalancesDto`
