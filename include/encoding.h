#ifndef AUX_ENCODING_H
#define AUX_ENCODING_H
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

/* Input and output are nonoverlapping, caller-owned buffers. Hex encoding
 * writes 2*len ASCII bytes. Decoding writes preceding valid pairs and reports
 * 0 on success, 1 for odd input, 2 for wrong capacity, or 3 and a bad index.
 * URL encoding requires capacity >= 3*len and returns the bytes written. */
bool aux_v1_hex_encode(const uint8_t *input, size_t len, uint8_t *out, size_t capacity);
uint8_t aux_v1_hex_decode(const uint8_t *input, size_t len, uint8_t *out, size_t capacity, size_t *bad);
size_t aux_v1_url_component(const uint8_t *input, size_t len, uint8_t *out, size_t capacity);
#endif
