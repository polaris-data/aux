// Development oracle only. Run from the pinned lighter-go module using
// lighter_vectors.py; never compiled into or invoked by the SDK.
package main

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"time"

	"github.com/elliottech/lighter-go/signer"
	"github.com/elliottech/lighter-go/types"
	"github.com/elliottech/lighter-go/types/txtypes"
	curve "github.com/elliottech/poseidon_crypto/curve/ecgfp5"
	g "github.com/elliottech/poseidon_crypto/field/goldilocks"
	fp5 "github.com/elliottech/poseidon_crypto/field/goldilocks_quintic_extension"
	p2 "github.com/elliottech/poseidon_crypto/hash/poseidon2_goldilocks"
	schnorr "github.com/elliottech/poseidon_crypto/signature/schnorr"
)

func must(err error) {
	if err != nil {
		panic(err)
	}
}
func bytes(s string) []byte { b, e := hex.DecodeString(s); must(e); return b }
func signature(digest []byte, key []byte, nonce uint64) []byte {
	m, e := fp5.FromCanonicalLittleEndianBytes(digest)
	must(e)
	k := make([]byte, 40)
	for i := 0; i < 8; i++ {
		k[i] = byte(nonce >> (8 * i))
	}
	return schnorr.SchnorrSignHashedMessage2(m, curve.ScalarElementFromLittleEndianBytes(key), curve.ScalarElementFromLittleEndianBytes(k)).ToBytes()
}
func verify(pub, digest, sig []byte) {
	p, e := fp5.FromCanonicalLittleEndianBytes(pub)
	must(e)
	m, e := fp5.FromCanonicalLittleEndianBytes(digest)
	must(e)
	s, e := schnorr.SigFromBytes(sig)
	must(e)
	if !schnorr.IsSchnorrSignatureValid(p, m, s) {
		panic("reference signature verification failed")
	}
}
func main() {
	if len(os.Args) > 1 {
		var items []map[string]string
		b, e := os.ReadFile(os.Args[1])
		must(e)
		must(json.Unmarshal(b, &items))
		for _, item := range items {
			verify(bytes(item["public_key"]), bytes(item["digest"]), bytes(item["signature"]))
		}
		fmt.Printf("Go accepted %d Rust signatures\n", len(items))
		return
	}
	keys := []string{
		"01" + strings.Repeat("00", 39),
		"1234567890abcdef" + strings.Repeat("0123456789abcdef", 3) + "0123456789abcd34",
		"e0ff8b9496d90fe89ca024d7395c88e83906b8cfe6ffff7f16000000f1ffff7f07000080fdffff7f",
	}
	var result []map[string]any
	for ki, keyHex := range keys {
		key := bytes(keyHex)
		km, e := signer.NewKeyManager(key)
		must(e)
		pub := km.PubKeyBytes()
		for _, chain := range []uint32{300, 304} {
			for variant := 0; variant < 5; variant++ {
				account := int64(37)
				api := uint8(3)
				nonce := int64(42)
				expiry := int64(1800000600000)
				attrs := txtypes.L2TxAttributes(nil)
				if variant == 1 {
					attrs = txtypes.L2TxAttributes{1: 42, 2: 50, 3: 25}
				}
				if variant == 2 {
					attrs = txtypes.L2TxAttributes{6: 2, 7: 1, 8: 1799999999000}
				}
				if variant == 3 {
					account = 281474976710654
					api = 254
					nonce = 9223372036854775807
					expiry = 281474976710655
				}
				order := &txtypes.OrderInfo{MarketIndex: 4096, ClientOrderIndex: 123, BaseAmount: 123456789, Price: 345678, IsAsk: 1, Type: 0, TimeInForce: 2, ReduceOnly: 0, TriggerPrice: 0, OrderExpiry: 1800086400000}
				if variant == 3 {
					order.ClientOrderIndex = 281474976710655
					order.BaseAmount = 281474976710655
					order.Price = 4294967295
					order.OrderExpiry = 9223372036854775807
				}
				if variant == 4 {
					order.Type = 2
					order.TimeInForce = 0
					order.TriggerPrice = 123456
					order.ReduceOnly = 1
				}
				tx := &txtypes.L2CreateOrderTxInfo{AccountIndex: account, ApiKeyIndex: api, OrderInfo: order, ExpiredAt: expiry, Nonce: nonce, L2TxAttributes: attrs}
				must(tx.Validate())
				digest, e := tx.Hash(chain)
				must(e)
				tx.Sig = signature(digest, key, uint64(ki*20+variant+7))
				verify(pub[:], digest, tx.Sig)
				info, e := tx.GetTxInfo()
				must(e)
				result = append(result, map[string]any{"kind": "create", "key": keyHex, "public_key": hex.EncodeToString(pub[:]), "chain_id": chain, "tx_info": json.RawMessage(info), "digest": hex.EncodeToString(digest), "signature": hex.EncodeToString(tx.Sig)})
				cancel := &txtypes.L2CancelOrderTxInfo{AccountIndex: account, ApiKeyIndex: api, MarketIndex: 4099, Index: 577305177233555425, ExpiredAt: expiry, Nonce: nonce, L2TxAttributes: attrs}
				must(cancel.Validate())
				digest, e = cancel.Hash(chain)
				must(e)
				cancel.Sig = signature(digest, key, uint64(ki*20+variant+100))
				verify(pub[:], digest, cancel.Sig)
				info, e = cancel.GetTxInfo()
				must(e)
				result = append(result, map[string]any{"kind": "cancel", "key": keyHex, "public_key": hex.EncodeToString(pub[:]), "chain_id": chain, "tx_info": json.RawMessage(info), "digest": hex.EncodeToString(digest), "signature": hex.EncodeToString(cancel.Sig)})
			}
		}
		for i, deadline := range []int64{1800000000, 1800028800} {
			account := int64(37)
			api := uint8(3)
			message := fmt.Sprintf("%d:%d:%d", deadline, account, api)
			words, e := g.ArrayFromCanonicalLittleEndianBytes([]byte(message))
			must(e)
			digest := p2.HashToQuinticExtension(words).ToLittleEndianBytes()
			sig := signature(digest, key, uint64(ki*20+i+200))
			verify(pub[:], digest, sig)
			token, e := types.ConstructAuthToken(km, time.Unix(deadline, 0), &types.TransactOpts{FromAccountIndex: &account, ApiKeyIndex: &api})
			must(e)
			parts := strings.Split(token, ":")
			verify(pub[:], digest, bytes(parts[3]))
			result = append(result, map[string]any{"kind": "auth", "key": keyHex, "public_key": hex.EncodeToString(pub[:]), "chain_id": 300, "account_index": account, "api_key_index": api, "deadline": deadline, "message": message, "digest": hex.EncodeToString(digest), "signature": hex.EncodeToString(sig), "token": message + ":" + hex.EncodeToString(sig)})
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	must(encoder.Encode(result))
}
