#ifndef AUX_ZIG_HERMES_H
#define AUX_ZIG_HERMES_H

#include <stddef.h>
#include <stdint.h>

typedef struct {
    uint8_t *ptr;
    size_t len;
} aux_v1_hermes_buffer;

typedef struct aux_v1_hermes_sse aux_v1_hermes_sse;

typedef struct {
    const uint8_t *ptr;
    size_t len;
} aux_v1_hermes_view;

typedef struct {
    const aux_v1_hermes_view *ids;
    size_t ids_len;
    aux_v1_hermes_view encoding;
    uint8_t parsed;
    uint8_t ignore_invalid_price_ids;
    uint8_t allow_unordered;
    uint8_t benchmarks_only;
    aux_v1_hermes_view asset_type;
    aux_v1_hermes_view query;
    uint64_t path_number;
} aux_v1_hermes_request_view;

enum {
    AUX_HERMES_OK = 0,
    AUX_HERMES_NEED_INPUT = 1,
    AUX_HERMES_INVALID_REQUEST = 2,
    AUX_HERMES_INVALID_JSON = 3,
    AUX_HERMES_VENDOR_ERROR = 4,
    AUX_HERMES_LIMIT = 5,
    AUX_HERMES_TRUNCATED = 6,
    AUX_HERMES_OUT_OF_MEMORY = 7,
};

enum aux_v1_hermes_operation {
    AUX_HERMES_LATEST, AUX_HERMES_STREAM, AUX_HERMES_LIVE, AUX_HERMES_READY,
    AUX_HERMES_FEEDS, AUX_HERMES_AT_TIMESTAMP, AUX_HERMES_TWAPS, AUX_HERMES_STAKE_CAPS
};

/* Input pointers are non-null and borrowed for the call. Supply a valid output
 * pointer. The library initializes outputs even on failure. Free a successful request buffer exactly once with
 * aux_v1_hermes_buffer_free. Optional flags are 0 (absent), 1 (false), or 2 (true). */
uint32_t aux_v1_hermes_request(uint8_t operation, const aux_v1_hermes_request_view *request, aux_v1_hermes_buffer *out);
void aux_v1_hermes_buffer_free(aux_v1_hermes_buffer buffer);

/* A stream has exclusive ownership and may move between threads. Calls on a
 * single stream must be serialized. Destroy each non-null handle exactly once. */
aux_v1_hermes_sse *aux_v1_hermes_sse_create(size_t limit);
void aux_v1_hermes_sse_destroy(aux_v1_hermes_sse *stream);
uint32_t aux_v1_hermes_sse_push(aux_v1_hermes_sse *stream, const uint8_t *ptr, size_t len);
/* The event view is valid until another call on this stream. */
uint32_t aux_v1_hermes_sse_next(aux_v1_hermes_sse *stream, aux_v1_hermes_view *out);
/* Call after draining all complete records and receiving EOF. */
uint32_t aux_v1_hermes_sse_finish(const aux_v1_hermes_sse *stream);

#endif
