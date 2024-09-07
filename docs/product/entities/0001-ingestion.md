# Ingestion

> "Ingestion" is a single log which have purpose to represent interaction with given "Substance", ingestions can be managed by "Mixtures" and "Stacks" where mixture is a monolith builtup from ingestion templates that happen when mixture is ingested and stacks are defining cyclic ingestions.

## Structure

```rs
struct Ingestion {
	id: ULID,
	substance_iupac: String,
	route_of_administration: RouteOfAdministration,
	dosage: Dosage,
	ingested_at: DateTime<Local>,
	created_at: DateTime<Local>,
	updated_at: DateTime<Local>
}
```
