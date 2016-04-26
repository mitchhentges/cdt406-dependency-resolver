# Dependency Resolver

## Building

1. Install [Rust](https://www.rust-lang.org/)
2. `cargo run`

## Output Specification

## Output Example

From the following tests and results:
|Test ID            |Execution 1|Execution 2|Execution 3|Execution 4|Execution 5|
| ----------------- | --------- | --------- | --------- | --------- | --------- |
|A                  |FAILED     |FAILED     |PASSED     |FAILED     |FAILED     |
|B                  |FAILED     |PASSED     |PASSED     |PASSED     |FAILED     |
|C                  |FAILED     |FAILED     |FAILED     |PASSED     |FAILED     |
|D                  |PASSED     |FAILED     |PASSED     |FAILED     |FAILED     |

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