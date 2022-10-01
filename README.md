# pg_fhir_filter

**This is a proof of concept only. Do not use it**

This is a proof-of-concept authorization system for FHIR data. It leverages postgresql Row Level Security (RLS) and extension functions to efficiently authorize arbitrary queries over FHIR data.

## Rationale

Previous proof-of-concept work introduced authorization as a _gateway_ in front of a standard FHIR server: 
```
User ---> Gateway ---> FHIR server ---> Database
```

See [https://github.com/patientsknowbest/fhirgate](fhirgate).

Some limitations of that approach:
- FHIR searches are somewhat limited; it's not possible to perform searches with OR criteria over disparate properties; which means that security related properties must be duplicated into the same field; for example the meta.security field of a data point must contain both privacy flags and source information.
- Features of FHIR APIs like `_includes` and `_revincludes` search parameters are difficult to authorize as they effectively 'join' to other FHIR resources which may have different access policies.
- Features like graphQL or arbitrary SQL are near impossible to authorize.

In this repository, an alternative approach is considered:
```
User ---> FHIR server ---> Database+authorization
```
Here we attempt to push the authorization concern right down into the database.
The practical implementation of this looks something like: 

- Extend the FHIR server to add the authenticated user to postgresql queries as a context parameter by using e.g. `SET LOCAL`
- Implement uniform RLS policies for every resource, which defer to a custom function
- Implement a custom function which accepts the context information + FHIR resource JSON and approves or denies access.

To note here: it might be possible to avoid custom functions entirely and implement access control rules in pgplsql. _However_ this might be detrimental to testing. In this PoC I am exploring the possibility of re-using oso authorization library and access control rules, since those are logically the same regardless of the enforcement point. This should also facilitate testing since unit tests might be written directly against the policy evaluation function rather than requiring a postgresql installation to test on.

Doing this would permit: 
- higher level components can remain unconcerned and uncomplicated by authorization considerations.
- arbitrary SQL queries (including user submitted queries) might be executed safely over FHIR tables.
- this also permits all FHIR API features like `_includes` as these are implemented by the FHIR server as SQL queries.

And potential limitations of this approach are:
- lock-in to database implementation (wheras the fhirgate approach is agnostic to FHIR server implementation)
- custom functions prevent the use of managed postgresql instances
- debugging & testing policies might become problematic depending on the tooling & approach taken.
- **performance** 

This PoC explores the feasibility and performance of this approach.









