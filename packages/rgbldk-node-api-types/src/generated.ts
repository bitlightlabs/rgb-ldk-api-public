// Generated. Do not edit.
// Source: crates/rgbldk_http_dto/src/dto.rs
// Run: `pnpm gen` at repo root.

export type U64String = string;

export interface BalancesDto {
  btc: BtcBalancesDto;
  rgb: RgbBalancesDto;
}

export interface Bolt11ClaimForHashRequest {
  payment_hash: string;
  preimage: string;
  claimable_amount_msat: U64String;
}

export interface Bolt11DecodeRequest {
  invoice: string;
}

export interface Bolt11DecodeResponse {
  payment_hash: string;
  destination: string;
  amount_msat?: U64String;
  expiry_secs: U64String;
}

export interface Bolt11FailForHashRequest {
  payment_hash: string;
}

export interface Bolt11PayRequest {
  invoice: string;
  amount_msat?: U64String;
}

export interface Bolt11PayResponse {
  payment_id: string;
  preimage: string;
  amount_sats: U64String;
  destination: string;
  fee_paid_msat?: U64String;
}

export interface Bolt11ReceiveForHashRequest {
  amount_msat: U64String;
  description: string;
  expiry_secs: number;
  payment_hash: string;
}

export interface Bolt11ReceiveRequest {
  amount_msat: U64String;
  description: string;
  expiry_secs: number;
}

export interface Bolt11ReceiveResponse {
  invoice: string;
}

export interface Bolt11ReceiveVarRequest {
  description: string;
  expiry_secs: number;
}

export interface Bolt11SendRequest {
  invoice: string;
}

export interface Bolt11SendUsingAmountRequest {
  invoice: string;
  amount_msat: U64String;
}

export interface Bolt12OfferDecodeRequest {
  offer: string;
}

export interface Bolt12OfferDecodeResponse {
  offer_id: string;
  signing_pubkey?: string;
  description?: string;
  issuer?: string;
  amount_msat?: U64String;
  absolute_expiry_unix_secs?: U64String;
  chain_hashes?: string[];
  paths_count: number;
  expects_quantity: boolean;
}

export interface Bolt12OfferReceiveRequest {
  amount_msat: U64String;
  description: string;
  expiry_secs?: number;
  quantity?: U64String;
}

export interface Bolt12OfferReceiveVarRequest {
  description: string;
  expiry_secs?: number;
}

export interface Bolt12OfferResponse {
  offer: string;
}

export interface Bolt12OfferSendRequest {
  offer: string;
  amount_msat?: U64String;
  quantity?: U64String;
  payer_note?: string;
}

export interface Bolt12RefundDecodeRequest {
  refund: string;
}

export interface Bolt12RefundDecodeResponse {
  description: string;
  issuer?: string;
  amount_msat: U64String;
  absolute_expiry_unix_secs?: U64String;
  chain_hash: string;
  payer_signing_pubkey: string;
  payer_note?: string;
  quantity?: U64String;
  paths_count: number;
}

export interface Bolt12RefundInitiateRequest {
  amount_msat: U64String;
  expiry_secs: number;
  quantity?: U64String;
  payer_note?: string;
}

export interface Bolt12RefundInitiateResponse {
  refund: string;
  payment_id: string;
}

export interface Bolt12RefundRequestPaymentRequest {
  refund: string;
}

export interface Bolt12RefundRequestPaymentResponse {
  invoice: string;
  invoice_hex: string;
  payment_id: string;
}

export interface BtcBalancesDto {
  onchain_total_sats: U64String;
  onchain_spendable_sats: U64String;
  anchor_channels_reserve_sats: U64String;
  lightning_total_sats: U64String;
}

export interface ChannelDetailsExtendedDto {
  channel_id: string;
  user_channel_id: string;
  counterparty_node_id: string;
  channel_point?: string;
  channel_value_sats: U64String;
  outbound_capacity_msat: U64String;
  inbound_capacity_msat: U64String;
  local_balance_msat?: U64String;
  remote_balance_msat?: U64String;
  local_unspendable_punishment_reserve_sats?: U64String;
  remote_unspendable_punishment_reserve_sats: U64String;
  is_channel_ready: boolean;
  is_usable: boolean;
  is_announced: boolean;
  rgb_balance?: RgbChannelBalanceDto;
}

export interface CloseChannelRequest {
  user_channel_id: string;
  counterparty_node_id: string;
}

export interface ControlLockRequest {
  yes: boolean;
  force?: boolean;
}

export interface ControlStatusDto {
  ok: boolean;
  locked: boolean;
  running: boolean;
  lockable?: boolean;
  checks?: HealthCheckDto[];
}

export interface ControlUnlockRequest {
  passphrase?: string;
}

export interface ControlVersionDto {
  ok: boolean;
  protocol: string;
  daemon: string;
  daemon_version: string;
}

export interface CustomTlvDto {
  type: U64String;
  value_hex: string;
}

export interface ErrorResponse {
  ok: boolean;
  error: string;
  hint?: string;
  checks?: HealthCheckDto[];
}

export interface HealthCheckDto {
  name: string;
  ok: boolean;
  detail?: string;
  hint?: string;
}

export interface ListeningAddressesResponse {
  addresses: string[];
}

export interface LockedStatusDto {
  ok: boolean;
  locked: boolean;
  running: boolean;
  checks?: HealthCheckDto[];
}

export interface NodeIdResponse {
  node_id: string;
}

export interface OkResponse {
  ok: boolean;
  checks?: HealthCheckDto[];
}

export interface OpenChannelRequest {
  node_id: string;
  address: string;
  channel_amount_sats: U64String;
  push_to_counterparty_msat?: U64String;
  announce?: boolean;
  rgb?: RgbOpenChannelRequest;
}

export interface OpenChannelResponse {
  user_channel_id: string;
}

export interface OutPointDto {
  txid: string;
  vout: number;
}

export interface PaymentDetailsDto {
  id: string;
  direction: string;
  status: string;
  amount_msat?: U64String;
  kind: string;
  fee_paid_msat?: U64String;
  kind_details?: unknown;
}

export interface PaymentWaitRequest {
  timeout_secs?: number;
}

export interface PaymentWaitResponse {
  ok: boolean;
  payment: PaymentDetailsDto;
  checks?: HealthCheckDto[];
}

export interface PeerConnectRequest {
  node_id: string;
  address: string;
  persist?: boolean;
}

export interface PeerDetailsDto {
  node_id: string;
  address: string;
  is_persisted: boolean;
  is_connected: boolean;
}

export interface PeerDisconnectRequest {
  node_id: string;
}

export interface RgbBalancesDto {
  l1: RgbL1BalanceDto[];
  l2: RgbL2BalanceDto[];
}

export interface RgbChannelBalanceDto {
  asset_id: string;
  local_amount: U64String;
  remote_amount: U64String;
}

export interface RgbContractBalanceDto {
  mined: U64String;
  tentative: U64String;
  offchain: U64String;
  archived: U64String;
  total: U64String;
}

export interface RgbContractBalanceResponse {
  contract_id: string;
  balance: RgbContractBalanceDto;
}

export interface RgbContractDto {
  contract_id: string;
  asset_id: string;
  name?: string;
  ticker?: string;
  precision?: number;
  issued_supply?: U64String;
  details?: string;
}

export interface RgbContractKnownResponse {
  contract_id: string;
  known: boolean;
}

export interface RgbContractsExportRequest {
  contract_id: string;
}

export interface RgbContractsExportResponse {
  ok: boolean;
  contract_id: string;
  consignment_key: string;
  checks?: HealthCheckDto[];
}

export interface RgbContractsImportResponse {
  ok: boolean;
  contract_id: string;
  consignment_key: string;
  checks?: HealthCheckDto[];
}

export interface RgbContractsIssueRequest {
  issuer_name: string;
  contract_name: string;
  ticker?: string;
  precision?: number;
  issued_supply: U64String;
  utxo?: string;
}

export interface RgbContractsIssueResponse {
  ok: boolean;
  contract_id: string;
  asset_id: string;
  issued_supply: U64String;
  checks?: HealthCheckDto[];
}

export interface RgbContractsResponse {
  contracts: RgbContractDto[];
}

export interface RgbInvalidIssuerDto {
  name: string;
  error: string;
}

export interface RgbIssuersImportResponse {
  ok: boolean;
  issuer_name: string;
  checks?: HealthCheckDto[];
}

export interface RgbIssuersResponse {
  issuers: string[];
  invalid_issuers?: RgbInvalidIssuerDto[];
}

export interface RgbL1BalanceDto {
  contract_id: string;
  asset_id: string;
  mined: U64String;
  tentative: U64String;
  offchain: U64String;
  archived: U64String;
  total: U64String;
}

export interface RgbL2BalanceDto {
  channel_id: string;
  asset_id: string;
  local_amount: U64String;
  remote_amount: U64String;
}

export interface RgbLnInvoiceCreateForHashRequest {
  asset_id: string;
  asset_amount: U64String;
  payment_hash: string;
  description: string;
  expiry_secs?: number;
  btc_carrier_amount_msat: U64String;
}

export interface RgbLnInvoiceCreateRequest {
  asset_id: string;
  asset_amount: U64String;
  description: string;
  expiry_secs?: number;
  btc_carrier_amount_msat: U64String;
}

export interface RgbLnInvoiceDecodeRequest {
  invoice: string;
}

export interface RgbLnInvoiceDecodeResponse {
  payment_hash: string;
  destination: string;
  carrier_amount_msat?: U64String;
  expiry_secs: U64String;
  asset_id?: string;
  asset_amount?: U64String;
}

export interface RgbLnInvoiceResponse {
  invoice: string;
}

export interface RgbLnPayRequest {
  invoice: string;
  asset_id?: string;
  asset_amount?: U64String;
}

export interface RgbNewAddressResponse {
  address: string;
}

export interface RgbOnchainInvoiceCreateRequest {
  contract_id: string;
  amount: U64String;
  expiry_secs?: U64String;
  use_witness_utxo?: boolean;
  nonce?: U64String;
  blinding_utxo?: string;
}

export interface RgbOnchainInvoiceDecodeRequest {
  invoice: string;
}

export interface RgbOnchainInvoiceDecodeResponse {
  contract_id: string;
  amount: U64String;
  beneficiary: string;
  use_witness_utxo: boolean;
  expiry_unix_secs?: U64String;
}

export interface RgbOnchainInvoiceResponse {
  invoice: string;
  blinding_utxo_used?: string;
}

export interface RgbOnchainPaymentDto {
  id: string;
  kind: string;
  status: string;
  created_at_unix_secs: U64String;
  latest_update_timestamp: U64String;
  expires_at_unix_secs?: U64String;
  invoice?: string;
  contract_id?: string;
  amount?: U64String;
  txid?: string;
  consignment_key?: string;
  consignment_download_path?: string;
  asset_id?: string;
}

export interface RgbOnchainPaymentsResponse {
  payments: RgbOnchainPaymentDto[];
}

export interface RgbOnchainReceiveRequest {
  consignment_key: string;
  payment_id?: string;
  invoice?: string;
}

export interface RgbOnchainReceiveResponse {
  asset_id: string;
  amount: U64String;
}

export interface RgbOnchainSendRequest {
  invoice: string;
  sats_for_fee_and_outputs?: U64String;
  fee_rate_sats_per_vb: number;
}

export interface RgbOnchainSendResponse {
  txid: string;
  consignment_key: string;
}

export interface RgbOpenChannelRequest {
  asset_id: string;
  asset_amount: U64String;
  color_context_data: string;
}

export interface RgbPaymentContextDto {
  asset_id: string;
  asset_amount: U64String;
  direction: string;
  is_swap: boolean;
}

export interface RgbUtxoAssetAllocationDto {
  contract_id: string;
  amount: U64String;
}

export interface RgbUtxoSummaryDto {
  outpoint: string;
  value_sats?: U64String;
  confirmed_height?: number;
  reserved: boolean;
  reserved_until_unix_secs?: U64String;
  assets?: RgbUtxoAssetAllocationDto[];
}

export interface RgbUtxosReleaseRequest {
  reservation_id?: string;
  outpoint?: string;
}

export interface RgbUtxosReleaseResponse {
  released: boolean;
}

export interface RgbUtxosReserveRequest {
  outpoint?: string;
  ttl_secs?: U64String;
}

export interface RgbUtxosReserveResponse {
  reservation_id: string;
  outpoint: string;
  reserved_until_unix_secs: U64String;
}

export interface RgbUtxosResponse {
  utxos: string[];
}

export interface RgbUtxosSummaryResponse {
  utxos: RgbUtxoSummaryDto[];
}

export interface SendResponse {
  payment_id: string;
}

export interface SpontaneousSendRequest {
  counterparty_node_id: string;
  amount_msat: U64String;
  custom_tlvs?: CustomTlvDto[];
}

export interface StatusDto {
  is_running: boolean;
  is_listening: boolean;
  best_block_height: number;
}

export interface VersionResponse {
  api_version: string;
  api_crate_version: string;
  core_crate_version: string;
}

export interface WalletNewAddressResponse {
  address: string;
}

export type EventDto =
  | { type: "PaymentSuccessful"; data: {
      payment_id?: string;
      fee_paid_msat?: U64String;
    }; }
  | { type: "PaymentFailed"; data: {
      payment_id?: string;
    }; }
  | { type: "PaymentReceived"; data: {
      payment_id?: string;
      payment_hash: string;
      amount_msat: U64String;
      custom_records?: CustomTlvDto[];
      rgb?: RgbPaymentContextDto;
    }; }
  | { type: "ChannelPending"; data: {
      funding_txo: OutPointDto;
    }; }
  | { type: "ChannelReady"; data: {
      user_channel_id: string;
    }; }
  | { type: "ChannelClosed"; data: {
      channel_id: string;
      user_channel_id: string;
      counterparty_node_id?: string;
      reason?: string;
    }; }
  | { type: "Other"; data: {
      kind: string;
    }; }
;

export type MainStatusResponse =
  | StatusDto
  | LockedStatusDto
;
