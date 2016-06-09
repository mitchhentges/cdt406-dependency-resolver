# Dependency Resolver [![Build Status](https://travis-ci.org/mitchhentges/cdt406-dependency-resolver.svg?branch=master)](https://travis-ci.org/mitchhentges/cdt406-dependency-resolver/)

When running tests for a system, there's potential for tests to overlap. In fact, there might be "test dependencies": a
test may "require" others to pass for itself to pass. For example, a car's "engine" test may depend on the "gas lines"
test passing. If "gas lines" fails, then the "engine" test would be "known" to fail. Conversely, if "gas lines" passes,
then the "engine" test will only fail if the engine itself is not working.

This software will investigate a history of known test executions, and will realize patterns in the test results. For
each test, it will determine its dependency on other tests, and encode the dependency relationship in a JSON format.
Logically, the more test results that are provided, the more accurate the resolved dependencies will be.

## Building

1. Install [Rust](https://www.rust-lang.org/)
2. `cargo run`

## Output Specification

The test dependency information is output to a single `JSON` file.

### Tests

Each top-level `key` is a test identifier. The `value` for these keys is the minimum requirements to make the
current test pass. The `value` is either an [`operator`](#operators), or `null`.

If a test isn't dependent on anything - it will only fail if the component-under-test fails - then its value will be
`null`. For example, perhaps testing "Paint" and "Horn" aren't dependent on any other component tests.
```
{
 "Paint": null,
 "Horn": null
}
```

For the sake of consistency, if a test is dependent on 1 or more other tests, its value is always an "operator". There
are two keys: `operator` and `inputs`:

```
{
 "Test": {
   "operator": "and",
   "inputs": [...]
 }
}
```

There are two types of operators: `and`, and `or`, which is set to the value of `type`. The `inputs` list contains
both individual tests that are depended on, and other "operators".

For example, "Engine" depends on `((Electric Starter | Manual Starter) & Pistons)`. So, if "Pistons" _and_ either
"Electric Starter" or "Manual Starter" are working, then the "Engine" can be tested. If either "Pistons" or
both "Electric Starter" and "Manual Starter" fail, then the "Engine" will fail due to a dependency.
```
{
 "Engine": {
  "operator": "and",
  "inputs": [
   "Pistons",
   {
    "operator": "or",
    "inputs": [
     "Electric Starter",
     "Manual Starter"
    ]
   }
 }
}
```

So, there's two different fields: `operator`, and `inputs`.
`operator` always has either the value `and` or `or`.
`inputs` always has a non-empty array of Operators and/or test IDs.

If there's only one `input` for a dependency, then the `operator` is `or`.

## Output Example

From the following test execution results, the produced dependency information `JSON` is below:

Test ID            |Execution 1|Execution 2|Execution 3|Execution 4|Execution 5
 ----------------- | --------- | --------- | --------- | --------- | ---------
A                  |PASSED     |PASSED     |FAILED     |PASSED     |PASSED
B                  |PASSED     |FAILED     |FAILED     |FAILED     |PASSED
C                  |PASSED     |PASSED     |PASSED     |FAILED     |PASSED
D                  |FAILED     |PASSED     |FAILED     |PASSED     |PASSED

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
 "B": {
  "operator": "and",
  "inputs": [
   "A",
   "C"
  ]
 },
 "C": null,
 "D": {
  "operator": "or",
  "inputs": [
   "A"
  ]
 }
}
```
