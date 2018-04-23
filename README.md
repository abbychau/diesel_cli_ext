# Diesel CLI Extension

Diesel CLI Extension is a tool-belt that aids Diesel CLI after it built schema.rs .

## How to use

First of all, `diesel print-schema > src/schema.rs` 

To generate model structs:
`diesel_ext > src/models.rs`, `diesel_ext model > src/models.rs` (it is the default option)

To generate proto structs:
`diesel_ext proto > myproto.proto`

To generate proto conversions:
`diesel_ext into_proto > myproto.proto`, `diesel_ext from_proto > myproto.proto`
