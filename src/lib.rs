use oso::{Oso, PolarClass};
use pgx::*;
use lazy_static::lazy_static;
use fhir;

pg_module_magic!();

// Is this safe for a postgres' extension? Probably.
lazy_static! {
    static ref OSO: Oso = {
        let mut oso = Oso::new();
        // TODO register classes with oso
        oso.load_file("authorization.polar").expect("Failed to load oso policy file");
        oso
    }; 
}

// TODO define classes 
// struct 
#[derive(PolarClass, Debug, Clone)]
pub struct Patient {
    pub isAccessFrozen: bool,
    pub isSharingDisabled: bool,
    //pub consents: []
}

// Expect 2x jsonb and run Oso policies on them?
#[pg_extern]
fn fhir_filter(resourceJsonB: JsonB, context: JsonB) -> bool {
    let resourceJson = resourceJsonB.0;
    let resourceType = resourceJson.get("resourceType")
        .and_then(|rt| {rt.as_str()})
        .expect(format!("no resource type found in resource! {:?}", resourceJson).as_str());
    log!("got resource type {}", resourceType);

    // let js = serde_json::to_string(&cond);
    // let res = serde_json::from_value::<Condition>(resourceJson)
    //     .expect(format!("Failed to parse json {:?}", resourceJson).as_str());
    // let res = match resourceType {
    //     "Condition" => ConditionWrapper()
    //     // Other types handled here
    // };
    //return OSO.is_allowed("martin", "read",  ConditionWrapper(res)).expect("Failed to evaluate policy!");
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
