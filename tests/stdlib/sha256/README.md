# SHA-256 oracle vectors

The NIST Cryptographic Algorithm Validation Program (CAVP) byte-oriented
SHA-256 response files, used by `SL-02` (SPEC.md §17.22) as an oracle for the
`LexLeanStd.Sha256` module. They are copied verbatim from the archive
`shabytetestvectors.zip` published at

    https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/shs/shabytetestvectors.zip

whose SHA-256 is `929ef80b7b3418aca026643f6f248815913b60e01741a44bba9e118067f4c9b8`
(CAVS 11.0, generated 2011-03-15; the archive's `Readme.txt` describes the
format). The files keep the archive's CRLF line endings.

| File | Vectors | Message lengths |
| --- | --- | --- |
| `SHA256ShortMsg.rsp` | 65 | 0 to 512 bits, every multiple of 8 |
| `SHA256LongMsg.rsp` | 64 | 1304 to 51200 bits |

`Len` is the message length in bits, `Msg` the message as hexadecimal (`00`
when `Len = 0`), and `MD` the expected digest. The FIPS 180-4 example
digests (`abc`, the two-block message, and the empty message) are stated in
the module itself as theorems, and the case checks them too; the one-million
`a` example runs under rustc only.

These vectors are an oracle, never an authority: a disagreement fails the
case and is filed as a defect; it is never resolved by adopting the vector's
digest.
