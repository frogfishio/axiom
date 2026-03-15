# 06. Stable Error Codes (Normative)

Fail(code,msg) must use stable codes and stable short English messages.

SDA-core:
- t_sda_wrong_shape "wrong shape"
- t_sda_missing_key "missing key"
- t_sda_duplicate_key "duplicate key"
- t_sda_selector_not_static "selector not static"
- t_sda_duplicate_label_in_selector "duplicate label"
- t_sda_unknown_field "unknown field"
- t_sda_unbound_placeholder "unbound placeholder"

Enrichment:
- t_src_missing "missing key"
- t_src_duplicate "duplicate key"
- t_src_schema_mismatch "schema mismatch"

PIC:
- t_pic_short_input "short input"
- t_pic_overflow "overflow"
- t_pic_invalid_digit "invalid digit"
- t_pic_invalid_sign "invalid sign"
- t_pic_invalid_encoding "invalid encoding"
- t_pic_wrong_shape "wrong shape"
- t_pic_occurs_count_mismatch "occurs count mismatch"
- t_pic_varsize_requires_framing "varsize requires framing"
- t_pic_overlay_size_mismatch "overlay size mismatch"
- t_pic_overlay_unknown_choice "overlay unknown choice"
