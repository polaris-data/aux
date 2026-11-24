#ifndef AUX_JSON_H
#define AUX_JSON_H
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

enum aux_json_token {
    AUX_OBJECT, AUX_END_OBJECT, AUX_ARRAY, AUX_END_ARRAY,
    AUX_STRING, AUX_NUMBER, AUX_TRUE, AUX_FALSE, AUX_NULL
};
enum aux_json_status { AUX_JSON_OK, AUX_JSON_INVALID, AUX_JSON_OOM, AUX_JSON_VISITOR_ERROR };
/* Input and context must remain valid throughout this synchronous call. Each
 * token view is borrowed until the visitor returns. On any error, discard the
 * partial result. Return false from the visitor to stop decoding. */
typedef bool (*aux_v1_json_visitor)(void *, uint32_t, const uint8_t *, size_t);
uint32_t aux_v1_json_visit(const uint8_t *, size_t, void *, aux_v1_json_visitor);
#endif
