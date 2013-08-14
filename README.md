## Usage

Just use the module and have it be accessible to the load path:

```
use tester::*;
mod tester;
```

## Syntax

```
describe!("module", {
  test!("function", {
    should!("do something", {
      must!(foo(1, 2) eq 5);
    })
  })
})
```

The `describe` macro establishes a context for a module with `test` macros defining each function. Tests are defined by `should` macros that should describe what behavior is being reflected by the function.

The `must` macros are assertions. The order should be: `must!(computed eq expected);`. For example, to test the function foo: `must!(foo() eq 5);` There are special considerations for floating point: `must!(pi() near 3.1415);` which tests that the given value is within 0.00001 to account for natural floating point error. You can specify the amount of tolerance: `must!(pi() near 3.1415 within 0.001);`

## License

Public domain or CC0.

## Output

Successful output will look like this:

![output screenshot](http://wilkie.io/images/rust-testing-micro-framework/rust_testing.png)

A failure will look like this:

![output screenshot](http://wilkie.io/images/rust-testing-micro-framework/rust_testing_failure.png)

It will report the expected result and the given result. No frills.

## TODO

* Randomize order of tests
* Better error reporting (line number for instance)
