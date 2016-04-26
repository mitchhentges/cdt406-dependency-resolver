# Dependency Resolver

## Building

1. Install [Rust](https://www.rust-lang.org/)
2. `cargo run`

## Output Specification

The test dependency information is output to a single `JSON` file.

### Tests

Each top-level `key` is a test identifier. The `value` for these keys is the minimum requirements to make the
current test fail. The `value` is either another test's ID, an [`operator`](#Operators), or `null`.

For example, if a test depends on another test, that means that if that test fails, then the current test will
fail as well. So, the `value` is the ID of the test that is depended on.

```
{
 "Brake Check": "Axle Check",
 "Coolant Pressure": "Tubing"
}
```

If a test depends on the results of another test, an [`operator`](#Operators) is used as a value:
```
{
 "Engine": {
   "operator": "and",
   "inputs": [...]
 }
}
```

Finally, if a test does not depend on another, `null` will be the value:

```
{
 "Paint": null,
 "Horn": null
}
```

### Operators

For some tests, the minimum requirement for failure is a _combination_ of other tests failing. In such a case, an
"Operator" is used to tie the test results together.
There are two types of operators: `and`, and `or`. The `type` is specified as the `operator`.
For each test to be used in the operator, the test ID is put in the `inputs` field.

For example, to represent the combination of `(Gas Tube | Fuel Pressure) & Pistons`:
```
{
 "operator": "and",
 "inputs": [
  "Pistons",
  {
   "operator": "or",
   "inputs": [
    "Gas Tube",
    "Fuel Pressure"
   ]
  }
}
```

So, there's two different fields: `operator`, and `inputs`.
`operator` always has either the value `and` or `or`.
`inputs` always has a non-empty array of more Operators and/or test IDs.

## Output Example

From the following tests and results:

Test ID            |Execution 1|Execution 2|Execution 3|Execution 4|Execution 5
 ----------------- | --------- | --------- | --------- | --------- | ---------
A                  |FAILED     |FAILED     |PASSED     |FAILED     |FAILED
B                  |FAILED     |PASSED     |PASSED     |PASSED     |FAILED
C                  |FAILED     |FAILED     |FAILED     |PASSED     |FAILED
D                  |PASSED     |FAILED     |PASSED     |FAILED     |FAILED

```
{
 "A": {
  "operator": "or",
  "inputs": [
   "D",
   {
    "operator": "and",
    "inputs": [
     "B",
     "C"
    ]
   }
  ]
 },
 "B": null,
 "C": {
  "operator": "and",
  "inputs": [
   "A",
   "B",
  ]
 },
 "D": null
}
```
