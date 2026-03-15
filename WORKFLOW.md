# Axiom Workflow Example

## Goal

This document shows how the architecture applies to a Lambda-style workflow.

The example problem is:

1. AWS Lambda receives an event JSON payload
2. we extract a customer key and request context
3. we call an external customer service
4. we normalize the service response
5. we combine the original event with the fetched customer data
6. we emit the final result

## End-to-End View

```text
AWS Event
  -> SDA extract
  -> request model
  -> SDA derive HTTP request pieces
  -> Axiom HTTP GET
  -> response JSON
  -> SDA normalize response
  -> normalized customer data
  -> Enrichment join with request model
  -> business product
  -> SDA refine
  -> Shaping emit final result
  -> RESULT
```

## Stage 1: Extract from Lambda Event

Input:

```json
{
  "requestContext": {
    "requestId": "req-123",
    "accountId": "acct-9"
  },
  "pathParameters": {
    "customerId": "cust-42"
  }
}
```

`SDA` is used to normalize the event into a smaller request model:

```text
Prod{
  request_id: "req-123",
  account_id: "acct-9",
  customer_id: "cust-42"
}
```

This stage is pure. No IO happens here.

## Stage 2: Derive External Request

`SDA` can prepare the pieces needed for an HTTP request:

```text
Prod{
  method: "GET",
  path: "/customers/cust-42",
  headers: Map{
    "x-account-id" -> "acct-9"
  }
}
```

Again, this is still pure computation.

`SDA` computes the request shape.
`Axiom` performs the request.

## Stage 3: Axiom Performs HTTP CRUD

`Axiom` takes the derived request model and performs the effect:

```text
GET https://service-a.example/customers/cust-42
headers:
  x-account-id: acct-9
```

This stage belongs to `Axiom`, not `SDA`, because it involves:

- network IO
- timeouts
- retries
- auth
- caching
- failure policy

The raw response is then passed back into the pure layers as data.

## Stage 4: Normalize Service Response

Suppose the service returns:

```json
{
  "id": "cust-42",
  "name": "Ada Lovelace",
  "tier": "gold",
  "email": "ada@example.com"
}
```

`SDA` normalizes it into a canonical internal form:

```text
Prod{
  customer_id: "cust-42",
  customer_name: "Ada Lovelace",
  customer_tier: "gold",
  customer_email: "ada@example.com"
}
```

This keeps the downstream model stable even if the service response changes slightly.

## Stage 5: Enrichment

Now we combine:

- the request model from Stage 1
- the normalized customer data from Stage 4

This can be expressed as an explicit enrichment or join step:

```text
Prod{
  request_id: "req-123",
  account_id: "acct-9",
  customer_id: "cust-42",
  customer_name: "Ada Lovelace",
  customer_tier: "gold",
  customer_email: "ada@example.com"
}
```

This stage should make the matching and missing-data policy explicit.

For example:

- is customer lookup required or optional
- what happens if there are duplicate matches
- what provenance should be attached to the result

## Stage 6: Refine Business Dataset

After enrichment, `SDA` can still do more pure work:

- derive flags
- compute classifications
- drop unused fields
- prepare a final dataset for emission

Example:

```text
Prod{
  request_id: "req-123",
  customer_id: "cust-42",
  customer_name: "Ada Lovelace",
  customer_tier: "gold",
  vip: true
}
```

## Stage 7: Shape Final Output

If final output construction needs its own semantics, `Shaping` emits the final contract.

Example output:

```json
{
  "requestId": "req-123",
  "customer": {
    "id": "cust-42",
    "name": "Ada Lovelace",
    "tier": "gold",
    "vip": true
  }
}
```

If `SDA` alone is enough for this stage, `Shaping` can remain optional.

## Responsibility Split

This workflow only works cleanly if responsibilities stay separate:

- `SDA` extracts and normalizes
- `Axiom` performs HTTP and file effects
- `Enrichment` combines datasets with explicit policies
- `Shaping` emits the final output contract

The key design rule is:

- `SDA` computes requests
- `Axiom` performs requests
- `SDA` normalizes responses
- `Axiom` routes the next step

## CLI View

The same workflow could be expressed with individual tools:

```sh
sda -f extract.sda < event.json > request.json
sda -f derive-request.sda < request.json > http-request.json
axiom http --request http-request.json > service-response.json
sda -f normalize-customer.sda < service-response.json > customer.json
enr -f join-customer.enr --left request.json --right customer.json > enriched.json
shape -f result.shp < enriched.json > output.json
```

Or, later, as one Axiom workflow:

```sh
axiom run customer-lookup.ax
```

## Why This Example Matters

This example is small, but it captures the real boundary that motivated the project:

- the workflow is not pure transformation
- the workflow is not just HTTP scripting either
- the system needs both semantic rigor and effectful orchestration

That is why `Axiom` should be the glue, not the transform engine.
