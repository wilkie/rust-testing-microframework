use tester::*;

mod tester;

fn foo(a: int, b: int) -> int {
  a + b
}

describe!("test", {
  test!("foo", {
    should!("add two numbers", {
      let a = 5;
      let b = 5;

      must!(foo(a, b) eq 10);
    })

    should!("add a positive and negative number", {
      let a = 5;
      let b = -5;

      must!(foo(a, b) eq 0);
    })

    should!("add a negative and positive number", {
      let a = -5;
      let b = 5;

      must!(foo(a, b) eq 0);
    })

    should!("add two negative numbers", {
      let a = -5;
      let b = -5;

      must!(foo(a, b) eq -10);
    })
  })
})
