"""Generate the Starknet Pedersen doubling table and Hades permutation round keys."""
from pathlib import Path
from hashlib import sha256

P = 2**251 + 17*2**192 + 1
# StarkWare's four fixed Pedersen bases for the low 248 and high 4 input bits.
BASES = [
    (0x0234287dcbaffe7f969c748655fca9e58fa8120b6d56eb0c1080d17957ebe47b,0x03b056f100f96fb21e889527d41f4e39940135dd7a6c94cc6ed0268ee89e5615,248),
    (0x04fa56f376c83db33f9dab2656558f3399099ec1de5e3018b7a6932dba8aa378,0x03fa0984c931c9e38113e0c0e47e4401562761f92a7a23b45168f4e80ff5b54d,4),
    (0x04ba4cc166be8dec764910f75b45f74b40c690c74709e90f3aa372f0bd2d6997,0x0040301cf5c1751f4b971e46c4ede85fcac5c59a5ce5ae7c48151f27b24b219c,248),
    (0x054302dcb0e6cc1c6e44cca8f61a63bb2ca65048d53fb325d36ff12c49a58202,0x01b77b3e37d13504b348046268d8ae25ce98ad783c25561a879dcc77e99c2426,4),
]
out = Path(__file__).resolve().parents[1] / "src"
table = bytearray()
for x,y,count in BASES:
    for _ in range(count):
        table.extend(x.to_bytes(32,"big") + y.to_bytes(32,"big"))
        slope = (3*x*x+1)*pow(2*y,-1,P) % P
        xx = (slope*slope-2*x) % P
        y, x = (slope*(x-xx)-y) % P, xx
(out / "stark-pedersen.bin").write_bytes(table)
keys = [int.from_bytes(sha256(f"Hades{i}".encode()).digest(),"big") % P for i in range(3*91)]
(out / "stark-poseidon.bin").write_bytes(b"".join(n.to_bytes(32,"big") for n in keys))
