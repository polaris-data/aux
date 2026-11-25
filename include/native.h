#ifndef AUX_NATIVE_H
#define AUX_NATIVE_H
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
typedef struct { uint8_t *ptr; size_t len; } aux_buffer;
enum aux_operation {
    AUX_CHAINLINK_DECODE_REPORT = 1,
    AUX_METRIC_ENCODE_CALL, AUX_METRIC_DECODE_INPUT, AUX_METRIC_DECODE_OUTPUT,
    AUX_METRIC_DECODE_ERROR, AUX_METRIC_DECODE_EVENT,
    AUX_POLYMARKET_ENCODE_CALL, AUX_POLYMARKET_DECODE_INPUT, AUX_POLYMARKET_DECODE_OUTPUT,
    AUX_HTTP_PREPARE, AUX_HTTP_DECODE, AUX_HTTP_EXECUTE,
    AUX_QFEX_SIGNATURE, AUX_DATABENTO_CRAM, AUX_ARCUS_SIGN, AUX_MONACO_SIGN, AUX_EVM_SIGN_TRANSACTION, AUX_DBN_DECODE_RECORD, AUX_PARADEX_DECODE_SBE,
    AUX_HTTP_PREPARE_DISPATCH, AUX_HTTP_FINISH_DISPATCH, AUX_KALSHI_HEADERS, AUX_DERIVE_LOGIN, AUX_DERIVE_SIGN_ACTION,
    AUX_EVM_PREPARE_RPC, AUX_EVM_FINISH_RPC, AUX_EVM_EXECUTE_RPC, AUX_HTTP_DEPENDENCY,
    AUX_SOLANA_ASSOCIATED_TOKEN_ADDRESS, AUX_SOLANA_PREPARE_DEPOSIT, AUX_SOLANA_SIGN_DEPOSIT,
    AUX_PASCAL_SIGN_PERMIT, AUX_PASCAL_BEGIN_SUBMISSION, AUX_PASCAL_FINISH_SUBMISSION, AUX_PASCAL_RECONCILE_WALLET, AUX_PARADEX_SIGN_TYPED, AUX_SOLANA_CHAIN, AUX_EVM_CHAIN, AUX_EXTENDED_SETTLEMENT, AUX_EXTENDED_ORDER, AUX_HYPERLIQUID_USER_ACTION, AUX_MONACO_LOGIN, AUX_ARCUS_SIGNING_PAYLOAD, AUX_METRIC_HELPER, AUX_HYPERLIQUID_SIGNING_PAYLOAD, AUX_POLYMARKET_HELPER, AUX_PHOENIX_TRANSACTION, AUX_PHOENIX_CHAIN, AUX_EXTENDED_ONBOARDING, AUX_PARADEX_ONBOARDING, AUX_METRIC_CHAIN,
    AUX_METRIC_PREPARE_CALL, AUX_KALSHI_REQUEST_PROOF, AUX_POLYMARKET_PREPARE_CALL, AUX_POLYMARKET_DECODE_ERROR, AUX_POLYMARKET_DECODE_EVENT, AUX_DERIVE_OFFLINE_ACTION, AUX_PASCAL_WALLET_PENDING, AUX_SIGNER
};
/* Borrow input for this call. Pass raw report hex for Chainlink or UTF-8 JSON
 * for the contract operations. Release successful JSON output exactly once.
 * Statuses are 0 for success, 1 for invalid input, 2 for allocation failure,
 * 3 for transport errors, 4 for exchange errors, 5 for missing authentication, and 7 for a submission awaiting reconciliation. */
uint32_t aux_v1_invoke(uint32_t operation, const uint8_t *input, size_t len, aux_buffer *out);
void aux_v1_result_free(aux_buffer out);
typedef struct aux_stream aux_stream;
/* One reader per handle. Cancel can interrupt a read on another thread.
 * Next returns 6 at EOF or after cancellation, 8 on a recoverable snapshot gap and 9 on a terminal sequence gap. Outputs use aux_v1_result_free.
 * Destroy exactly once after all concurrent calls finish. */
uint32_t aux_v1_stream_open(const uint8_t *config, size_t length, aux_stream **out);
/* connect may run on any reader thread, including on reconnect. It transfers a
 * connected blocking plaintext socket on success. For TLS, the application owns
 * verification, identity and encryption. context must live until stream destroy. */
typedef struct {
    void *context;
    uint32_t (*connect)(void *context, const uint8_t *host, size_t host_len,
                        uint16_t port, bool tls, int *out_fd);
} aux_stream_dial;
/* As invoke, with optional connection policy. On exchange status 4 a
 * nonempty buffer contains JSON {venue,status,body}. Status 7 may carry the
 * same envelope with a transaction hash in body. Free owned buffers as for success. */
uint32_t aux_v1_invoke_detailed(uint32_t operation, const uint8_t *input, size_t len,
                                const aux_stream_dial *dial, aux_buffer *out);
uint32_t aux_v1_stream_open_with_transport(const uint8_t *config, size_t length,
                                           const aux_stream_dial *dial, aux_stream **out);
/* On HTTP rejection the handle is null and status 4 carries an optional
 * {venue,status,body} buffer. Release the buffer with aux_v1_result_free. */
uint32_t aux_v1_stream_open_detailed(const uint8_t *config, size_t length,
                                     const aux_stream_dial *dial, aux_stream **out, aux_buffer *detail);
uint32_t aux_v1_http_execute_with_transport(const uint8_t *config, size_t length,
                                            const aux_stream_dial *dial, aux_buffer *out);
/* Copies a raw or Zstd-compressed DBN v1, v2 or v3 file. Read metadata first, then records. */
uint32_t aux_v1_dbn_open(const uint8_t *data, size_t length, aux_stream **out);
/* Raw bytes for Databento timeseries and file downloads, UTF-8 JSON for event streams. */
uint32_t aux_v1_stream_next(aux_stream *stream, aux_buffer *out);
/* Terminal venue frames return status 10 and an owned JSON {venue,body}. */
uint32_t aux_v1_stream_next_detailed(aux_stream *stream, aux_buffer *out);
/* Serialize commands and next calls per handle. Supports live-session control, batch reads and download_to. */
uint8_t aux_v1_stream_concurrent_commands(const void *stream);
uint32_t aux_v1_stream_command(aux_stream *stream, const uint8_t *json, size_t length, aux_buffer *out);
/* Status 10 carries the venue and complete correlated error frame in {venue,body}.
 * The caller owns this buffer and releases it with aux_v1_result_free. */
uint32_t aux_v1_stream_command_detailed(aux_stream *stream, const uint8_t *json, size_t length, aux_buffer *out);
void aux_v1_stream_cancel(aux_stream *stream);
void aux_v1_stream_destroy(aux_stream *stream);
#endif
