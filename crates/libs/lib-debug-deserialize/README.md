# serde_debug_deserialize

A debug wrapper for Serde deserialization that provides detailed error information.

## Features

- Wraps Serde deserialization to provide more detailed error messages
- Identifies the problematic field in JSON deserialization errors
- Shows context around the error in the JSON string
- Easy to use with existing Serde-compatible structs

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde_debug_deserialize = "0.1.0"
```

Then, in your code:

```rust
use serde_debug_deserialize::JsonDebugDeserialize;
use serde::Deserialize;

#[derive(Deserialize)]
struct MyStruct {
    // your fields here
}

// Wrap your type with DebugDeserialize for debugging
let result: Result<JsonDebugDeserialize<MyStruct>, _> = serde_json::from_str(&json_string);

// When you're done debugging, remove the DebugDeserialize wrapper
let result: Result<MyStruct, _> = serde_json::from_str(&json_string);
```

## Example output:
```
Raw response:
{"count":532,"next":"https://sandbox.elationemr.com/api/2.0/medications/?limit=1&offset=529","previous":"https://sandbox.elationemr.com/api/2.0/medications/?limit=1&offset=527","results":[{"id":142435380428826,"auth_refills":null,"directions":"1 tablet orally daily","chart_date":"2024-07-06T00:33:44Z","show_in_chart_feed":true,"created_date":"2024-07-06T00:33:44Z","deleted_date":null,"document_date":"2024-07-06T00:33:44Z","fulfillment":null,"is_doc_med":true,"last_modified":"2024-07-06T00:33:44Z","medication":{"id":8369471535,"rxnorm_cuis":["979485","311380"],"ndcs":["13668-0113-90","13668-0113-10","68382-0135-10","68382-0135-06","68382-0135-16","65862-0201-90","65862-0201-99","00054-0123-22","00781-5700-10","00781-5805-10","68180-0210-03","00781-5700-92","68084-0346-01","68084-0346-11","60429-0316-10","60429-0316-30","60429-0316-90","42571-0110-90","42571-0110-10","68071-0857-30","35356-0949-30","59746-0333-10","59746-0333-90","50268-0516-15","00904-6391-61","71610-0260-45","43547-0360-09","43547-0360-50","71610-0260-30","68788-7570-09","61919-0461-30","68788-7470-03","68645-0524-70","68788-7470-01","68788-7470-09","62332-0027-90","68788-7470-05","72789-0029-90","68788-7470-06","62332-0027-91","71610-0260-60","62332-0027-30","33261-0996-90","71205-0069-30","33261-0996-30","33261-0996-60","23155-0644-09","23155-0644-10","71205-0069-90","72789-0029-14","23155-0644-03","71610-0417-45","68788-7736-01","42291-0422-10","72865-0141-90","50268-0504-15","72865-0141-30","68180-0376-03","13107-0195-90","13107-0195-99","72865-0141-10","57237-0204-90","71205-0226-30","68180-0376-09","57237-0204-99","13107-0195-30","50090-2622-01","50090-2622-00","68788-7570-03","71610-0439-30","68645-0577-54","68788-7570-01","68788-7570-06","68788-7570-05","50268-0504-11","68788-7737-09","71610-0065-60","68788-6788-01","71335-0249-02","68788-6788-03","71335-0249-01","68788-6788-05","71610-0065-45","68788-6788-06","68788-6788-09","71610-0370-30","64380-0933-04","64380-0933-08","64380-0933-05","71335-0249-04","71610-0224-45","71335-0249-03","68788-7736-03","68788-7736-05","68788-7736-06","68788-7736-09","50268-0516-11","71610-0155-60","68788-7737-01","71610-0155-45","71610-0155-30","63739-0673-10","68788-7737-03","68788-7737-05","68788-7737-06","43063-0477-30","71610-0370-45","71335-0700-01","68788-7213-01","68645-0407-54","68788-7213-03","33342-0044-07","33342-0044-10","33342-0044-44","31722-0700-90","31722-0700-10","31722-0700-05","60429-0863-90","60429-0863-30","72189-0100-90","61919-0461-90","60429-0863-10","68788-7213-05","43063-0477-14","68788-7213-06","71610-0224-30","68788-7213-09","00904-7047-61","71610-0224-60","72189-0082-30","71610-0224-80","29300-0144-19","70934-0616-30","71610-0439-45","82009-0042-10","50090-5626-01","50090-5626-00","71610-0439-15","63187-0357-90","70934-0616-90","72789-0163-90","50090-4645-01","72189-0303-90","63629-2426-01","43547-0360-11","50090-5406-01","50090-5626-02","72789-0282-30","71610-0439-60","50090-4645-00"],"name":"Losartan Potassium Tab 25 mg","brand_name":"Losartan Potassium","generic_name":"Losartan Potassium Tab 25 mg","is_controlled":false,"type":"prescription","route":"Oral","strength":"25 mg","form":"Tab","practice":null,"created_date":null,"creation_type":"medispan","market_end_date":null,"obsolete_date":null},"medication_type":"prescription","notes":"","order_type":"New","patient":142424942706689,"pharmacy_instructions":"","practice":142350464319492,"prescribing_physician":null,"documenting_personnel":142435379970051,"qty":null,"qty_units":null,"num_samples":"","signed_by":142435379970051,"signed_date":"2024-07-06T00:33:44Z","start_date":"2024-07-05","icd10_codes":[],"thread":{"id":142435380363287,"dc_date":null,"is_permanent":true}}]}

WARNING: You're using the JsonDebugDeserialize on your struct! Remove this in production!

Deserialization error: invalid type: null, expected i64 at line 1 column 3511

Problematic field: prescribing_physician

Context:
...ctice":142350464319492,"prescribing_physician":null,"qty":null,"qty_units":null,"show_in_chart_feed"...
                                                     ^

```


## Warning

This crate is intended for debugging purposes only. Make sure to remove `DebugDeserialize` wrappers before deploying to production, as they may impact performance and print sensitive information.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
