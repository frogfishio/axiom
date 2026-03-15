# 05. PIC Codecs (Normative)

## 05.1 Codec interface
Codec[T]:
- decode(Bytes) -> Res[T]
- encode(T) -> Res[Bytes]
- size : Opt[Num]

## 05.2 Required primitives
U8/I8, U16BE/U16LE/I16BE/I16LE, U32BE/U32LE/I32BE/I32LE
Str(n, encoding, pad, trim)
Bits(n)
PackedDecimal/ZonedDecimal recommended.

## 05.3 pic{...}
Decodes left-to-right into Prod{...}.

## 05.4 OCCURS
occurs(n,C) and occurs(fieldName,C)
Variable-size occurs requires framing: lenPrefix / until / stride.

## 05.5 Overlays (REDEFINES)
overlay NAME size N choose by FIELD { TAG => ARM, ... }
- FIELD is earlier discriminator
- each ARM fixed-size exactly N
- decode yields raw bytes + chosen tagged view
- encode chooses by FIELD; unknown tag fails t_pic_overlay_unknown_choice
