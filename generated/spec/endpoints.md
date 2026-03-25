# Endpoint List (Prefix: `/api/v1`)

*This file is exported by `scripts/rgbldk_api_sync.py`.*

### GET `/api/v1/balances`

* **Summary:** List balances
* **Description:** Returns the aggregated BTC and RGB balances tracked by the node.
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

### GET `/api/v1/node_id`

* **Summary:** Node public key
* **Description:** Returns the node's public key in hex format.
* **Response (200):** `NodeIdResponse`

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
* **Description:** Returns a stored consignment by key. The response body is binary and can be encoded as raw, gzip, or zip.
* **Response (200):** `binary`

### GET `/api/v1/rgb/contract/{contract_id}/balance`

* **Summary:** Get RGB contract balance
* **Description:** Returns the aggregated L1 balance view for a single RGB contract.
* **Response (200):** `RgbContractBalanceResponse`

### GET `/api/v1/rgb/contract/{contract_id}/known`

* **Summary:** Check RGB contract known
* **Description:** Returns whether the node currently knows a given RGB contract id.
* **Response (200):** `RgbContractKnownResponse`

### GET `/api/v1/rgb/contracts`

* **Summary:** List RGB contracts
* **Description:** Returns RGB contracts known to the node together with best-effort metadata.
* **Response (200):** `RgbContractsResponse`

### POST `/api/v1/rgb/contracts/export`

* **Summary:** Export RGB contract
* **Description:** Exports a contract consignment. By default it returns JSON with a consignment key; when `download=true` it returns the encoded consignment bytes directly.
* **Request Body:** `RgbContractsExportRequest`
* **Response (200):** `RgbContractsExportResponse`

### POST `/api/v1/rgb/contracts/import`

* **Summary:** Import RGB contract
* **Description:** Imports a contract consignment from the raw request body. The body may be raw, gzip, or zip depending on the `format` query parameter.
* **Request Body:** `binary`
* **Response (200):** `RgbContractsImportResponse`

### POST `/api/v1/rgb/contracts/issue`

* **Summary:** Issue RGB contract
* **Description:** Issues a new RGB contract using an imported issuer and an RGB wallet UTXO.
* **Request Body:** `RgbContractsIssueRequest`
* **Response (200):** `RgbContractsIssueResponse`

### POST `/api/v1/rgb/ln/invoice/create`

* **Summary:** Create RGB Lightning invoice
* **Description:** Creates an RGB-aware Lightning invoice using a contract id and asset amount.
* **Request Body:** `RgbLnInvoiceCreateRequest`
* **Response (200):** `RgbLnInvoiceResponse`

### POST `/api/v1/rgb/ln/invoice/create_for_hash`

* **Summary:** Create RGB hold invoice
* **Description:** Creates an RGB-aware Lightning invoice bound to an explicit payment hash.
* **Request Body:** `RgbLnInvoiceCreateForHashRequest`
* **Response (200):** `RgbLnInvoiceResponse`

### POST `/api/v1/rgb/ln/invoice/decode`

* **Summary:** Decode RGB Lightning invoice
* **Description:** Decodes an RGB-aware Lightning invoice and returns both carrier and asset fields.
* **Request Body:** `RgbLnInvoiceDecodeRequest`
* **Response (200):** `RgbLnInvoiceDecodeResponse`

### POST `/api/v1/rgb/ln/pay`

* **Summary:** Pay RGB Lightning invoice
* **Description:** Pays an RGB Lightning invoice, optionally accepting explicit contract and asset values when the invoice does not carry them.
* **Request Body:** `RgbLnPayRequest`
* **Response (200):** `SendResponse`

### POST `/api/v1/rgb/new_address`

* **Summary:** New RGB address
* **Description:** Generates a new RGB wallet receive address.
* **Response (200):** `RgbNewAddressResponse`

### POST `/api/v1/rgb/onchain/invoice/create`

* **Summary:** Create RGB on-chain invoice
* **Description:** Creates an RGB on-chain invoice using either witness-out or blinded beneficiary mode.
* **Request Body:** `RgbOnchainInvoiceCreateRequest`
* **Response (200):** `RgbOnchainInvoiceResponse`

### POST `/api/v1/rgb/onchain/invoice/decode`

* **Summary:** Decode RGB on-chain invoice
* **Description:** Parses an RGB on-chain invoice and returns beneficiary and amount details.
* **Request Body:** `RgbOnchainInvoiceDecodeRequest`
* **Response (200):** `RgbOnchainInvoiceDecodeResponse`

### GET `/api/v1/rgb/onchain/payments`

* **Summary:** List RGB on-chain payments
* **Description:** Returns RGB on-chain payment history, optionally filtered by contract id.
* **Response (200):** `RgbOnchainPaymentsResponse`

### POST `/api/v1/rgb/onchain/receive`

* **Summary:** Receive RGB on-chain payment
* **Description:** Accepts an RGB on-chain consignment. The request can be either JSON metadata or raw/gzip/zip binary consignment bytes. Binary uploads require `payment_id` in the query string.
* **Request Body:** `RgbOnchainReceiveRequest`
* **Response (200):** `RgbOnchainReceiveResponse`

### POST `/api/v1/rgb/onchain/send`

* **Summary:** Send RGB on-chain payment
* **Description:** Builds and broadcasts an RGB on-chain payment transaction for an invoice.
* **Request Body:** `RgbOnchainSendRequest`
* **Response (200):** `RgbOnchainSendResponse`

### POST `/api/v1/rgb/sync`

* **Summary:** Sync RGB runtime
* **Description:** Synchronizes the RGB runtime state with the configured chain data and local wallet.
* **Response (200):** `OkResponse`

### GET `/api/v1/rgb/utxos`

* **Summary:** List RGB UTXOs
* **Description:** Returns the RGB wallet outpoints known to the node.
* **Response (200):** `RgbUtxosResponse`

### POST `/api/v1/rgb/utxos/release`

* **Summary:** Release RGB UTXO reservation
* **Description:** Releases an RGB UTXO reservation by reservation id or outpoint.
* **Request Body:** `RgbUtxosReleaseRequest`
* **Response (200):** `RgbUtxosReleaseResponse`

### POST `/api/v1/rgb/utxos/reserve`

* **Summary:** Reserve RGB UTXO
* **Description:** Reserves an RGB wallet outpoint for temporary exclusive use.
* **Request Body:** `RgbUtxosReserveRequest`
* **Response (200):** `RgbUtxosReserveResponse`

### GET `/api/v1/rgb/utxos/summary`

* **Summary:** Summarize RGB UTXOs
* **Description:** Returns RGB wallet UTXOs with BTC value, reservation state, and per-contract allocations.
* **Response (200):** `RgbUtxosSummaryResponse`

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
* **Description:** Generates a new Bitcoin on-chain receive address from the wallet.
* **Response (200):** `WalletNewAddressResponse`

### POST `/api/v1/wallet/sync`

* **Summary:** Sync wallet
* **Description:** Synchronizes the wallet state with the configured chain source.
* **Response (200):** `OkResponse`
