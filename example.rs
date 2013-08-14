// An example of using the framework
// licensed: public domain or cc0

use tester::*;
mod tester;

fn foo(a: float, b: float) -> float {
  a + b
}

describe!("test", {
  test!("foo", {
    should!("add two numbers", {
      let a = 5.0;
      let b = 5.0;

      must!(foo(a, b) eq 10.0);
    })

    should!("add a positive and negative number", {
      let a = 5.0;
      let b = -5.0;

      must!(foo(a, b) eq 0.0);
    })

    should!("add a negative and positive number", {
      let a = -5.0;
      let b = 5.0;

      must!(foo(a, b) eq 0.0);
    })

    should!("add two negative numbers", {
      let a = -5.0;
      let b = -5.0;

      must!(foo(a, b) eq -10.0);
    })

    should!("add two fractions", {
      let a = 1.25;
      let b = 3.13;

      must!(foo(a, b) near 4.38);
    })
  })
})
