# 07. Conformance Suite (Normative minimum)

Implementations MUST pass:

- placeholder scoping (unbound placeholder code/msg)
- null vs absence (Map with Null vs missing)
- BagKV duplicate behavior for ? and !
- normalizeUnique duplicate failure; normalizeFirst/Last determinism
- comprehension carrier preservation
- lookup!/lookupUnique failure codes
- PIC occurs depending-on count mismatch in encode
- PIC overlay size mismatch and unknown tag behavior
