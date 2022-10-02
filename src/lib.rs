use oso::{Oso};
use pgx::*;
use lazy_static::lazy_static;
// use thiserror::Error;

pg_module_magic!();

// Is this safe for a postgres' extension? Probably.
// lazy_static! {
//     static ref OSO: Oso = {
//         let mut oso = Oso::new();
//         // TODO register classes with oso
//         oso.load_files(vec!["authorization.polar"]).expect("Failed to load oso policy file");
//         oso
//     }; 
// }

// How can I make queries inside here?

// Expect 2x jsonb and run Oso policies on them?
#[pg_extern]
fn fhir_filter(resourceJsonB: JsonB, contextJsonB: JsonB) -> bool {
    // let resourceJson = resourceJsonB.0;
    // let resourceType = resourceJson.get("resourceType")
    //     .and_then(|rt| {rt.as_str()})
    //     .expect(format!("no resource type found in resource! {:?}", resourceJson).as_str());
    // log!("got resource type {}", resourceType);
    // let resource = match resourceType {
    //     "Patient" => models::Patient::from_fhir(resourceJson),
    //     _ => Err(FhirFilterError::ResourceNotSupported(resourceType.to_owned())),
    // }.expect("failed to parse resource");

    // OSO.is_allowed("", "read", resource).expect("failed to evaluate OSO rules!")
    false
}
    
#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;
    #[pg_test]
    fn test_hello_pg_fhir_filter() {
        // let foo = serde_json::json!({
        //     "name": "foo",
        // });
        // assert_eq!("Hello, foo", crate::hello_pg_fhir_filter(JsonB(foo)));
        // Spi::connect(|spi_client| {
        //     spi_client.update("insert  into ", limit, args)
        // });

        // Create FHIR schema 
        Spi::connect(|mut spi_client| {
            let _ = spi_client.update(r#"create table patient (id text, resource jsonb);"#, None, None);
            let mut stt = spi_client.select("select count(*) from patient", None, None);
            let ct: u32 = stt.next().unwrap().by_ordinal(1).unwrap().value().unwrap();
            assert_eq!(ct, 0);
            Ok(Some(()))
        });
        // Insert some sample data, linked appropriately..

        // Run some test queries
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
