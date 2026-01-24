# choreo_master_mobile_json

Rust port of `ChoreoMasterMobile.Json` for reading and writing `.choreo` JSON files.

## Notes / Differences

- Reference metadata is limited to `$id`/`$ref` for roles, dancers, and scene positions. Other object graphs are serialized inline.
- `Timestamp` is treated as a string (matching System.Text.Json `TimeSpan` output).
- `ExportToFile` always writes UTF-8 without an explicit encoding parameter.
- Color parsing supports `#AARRGGBB` (the format used by exported files). Other named color formats are not recognized.
