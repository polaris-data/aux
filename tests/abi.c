#include "native.h"
#include <stdio.h>
#include <string.h>

int main(void) {
    const char *args = "{\"venue\":\"extended\",\"operation\":\"get_asset_price\",\"params\":{\"asset\":\"ETH\"}}";
    const char *route = "/api/v1/info/assets/ETH/price";
    aux_buffer result = {0};
    if (aux_v1_invoke(AUX_HTTP_PREPARE, (const uint8_t *)args, strlen(args), &result) != 0)
        return 1;
    int found = 0;
    for (size_t i = 0; i + strlen(route) <= result.len; ++i)
        if (memcmp(result.ptr + i, route, strlen(route)) == 0) found = 1;
    aux_v1_result_free(result);
    if (!found) return 1;
    const char *wallet = "{\"owner\":\"GmaDrppBC7P5ARKV8g3djiwP89vz1jLK23V2GBjuAEGB\","
                         "\"mint\":\"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v\"}";
    const char *ata = "\"7woc3ajaGMMXczFYjxon4aQoHH3j126fMUR9c58eHRsK\"";
    if (aux_v1_invoke(AUX_SOLANA_ASSOCIATED_TOKEN_ADDRESS, (const uint8_t *)wallet,
                      strlen(wallet), &result) != 0) return 1;
    int matches = result.len == strlen(ata) && memcmp(result.ptr, ata, result.len) == 0;
    aux_v1_result_free(result);
    if (!matches) return 1;

    const char *settlement =
        "{\"chain_id\":\"SN_MAIN\",\"kind\":\"order\",\"public_key\":\"0x1ef15c18599971b7bec"
        "ed415a40f0c7deacfd9b0d1819e03d723d8bc943cfca\",\"message\":{\"expiration\":{\""
        "seconds\":100},\"salt\":\"123\",\"base_asset_id\":{\"value\":\"2\"},\"base_amount\":1"
        "00,\"quote_asset_id\":{\"value\":\"1\"},\"quote_amount\":-156,\"fee_asset_id\":{\"v"
        "alue\":\"1\"},\"fee_amount\":74,\"position_id\":{\"value\":100}}}";
    const char *digest = "{\"hash\":\"0x2ea4df899ff6f61f06de948fb534ebfa6c43472cb02cbe13a67dcd52c1e8288\"}";
    if (aux_v1_invoke(AUX_EXTENDED_SETTLEMENT, (const uint8_t *)settlement,
                      strlen(settlement), &result) != 0) return 1;
    matches = result.len == strlen(digest) && memcmp(result.ptr, digest, result.len) == 0;
    aux_v1_result_free(result);
    return matches ? 0 : 1;
}
