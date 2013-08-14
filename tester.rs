https://gist.github.com/wilkie/6227782#[macro_escape];

pub use std::uint;
pub use std::io;

pub fn report_failure(expected: ~str, actual: ~str) {
  io::print(fmt!("\n\x1b[31;1mFail\x1b[39;0m - %s vs %s", actual, expected));
}

trait TestInput {
  fn compare(&self, b: &Self) -> bool;
  fn compare_near(&self, b: &Self, tolerance: Self) -> bool;
  fn fail(&self, b: Self);
}

macro_rules! impl_test_input(
  ($t:ty) => {
    impl TestInput for $t {
      pub fn compare(&self, b: &$t) -> bool {
        *self == *b
      }

      pub fn compare_near(&self, b: &$t, _tolerance: $t) -> bool {
        *self == *b
      }

      pub fn fail(&self, b: $t) {
        report_failure(self.to_str(), b.to_str());
      }
    }
  }
)

macro_rules! impl_test_float(
  ($t:ty) => {
    impl TestInput for $t {
      pub fn compare(&self, b: &$t) -> bool {
        *self == *b
      }

      pub fn compare_near(&self, b: &$t, tolerance: $t) -> bool {
        (*self > b - tolerance) && (*self < b + tolerance)
      }

      pub fn fail(&self, b: $t) {
        report_failure(self.to_str(), b.to_str());
      }
    }
  }
)

macro_rules! impl_test_char(
  ($t:ty) => {
    impl TestInput for $t {
      pub fn compare(&self, b: &$t) -> bool {
        *self == *b
      }

      pub fn compare_near(&self, b: &$t, _tolerance: $t) -> bool {
        *self == *b
      }

      pub fn fail(&self, b: $t) {
        report_failure(fmt!("%c", *self), fmt!("%c", b))
      }
    }
  }
)

impl_test_input!(~str)
impl_test_input!(u8)
impl_test_input!(u16)
impl_test_input!(u32)
impl_test_input!(u64)
impl_test_input!(uint)
impl_test_input!(i8)
impl_test_input!(i16)
impl_test_input!(i32)
impl_test_input!(i64)
impl_test_input!(int)
impl_test_input!(bool)

impl_test_char!(char)

impl_test_float!(float)
impl_test_float!(f32)
impl_test_float!(f64)

pub fn perform_test<T:TestInput>(a:T, b:T, negate: bool) -> bool {
  if (a.compare(&b) ^ negate) {
    true
  }
  else {
    a.fail(b);
    false
  }
}

pub fn perform_near<T:TestInput>(a:T, b:T, tolerance:T, negate:bool) -> bool {
  if (a.compare_near(&b, tolerance) ^ negate) {
    true
  }
  else {
    a.fail(b);
    false
  }
}

macro_rules! describe(
  ($prompt:expr, $func:expr) => (
    fn main() {
      let module_name = $prompt;

      let mut _current_test = "<invalid>";

      let mut _indent = 0;

      let mut _tests:uint = 0;
      let mut _fails:uint = 0;
      let mut _successes:uint = 0;

      io::println(module_name);

      $func;

      let assertions = _successes + _fails;
      io::println(fmt!("\n%u tests %u assertions %u failures", _tests, assertions, _fails));
    }
  )
)

macro_rules! test(
  ($prompt:expr, $func:expr) => ({
    _current_test = $prompt;
    _indent += 1;

    do uint::range_step(0, _indent, 1) |_| {
      io::print(" ");

      true
    };

    io::println(_current_test);

    $func;

    _indent -= 1;
  })
)

macro_rules! must(
  ($a:expr eq $b:expr) => (
    if (perform_test($a, $b, false)) { _successes += 1; } else { _failure = true; _fails += 1; }
  );
  ($a:expr near $b:expr) => (
    if (perform_near($a, $b, 0.00001, false)) { _successes += 1; } else { _failure = true; _fails += 1; }
  );
  ($a:expr near $b:expr within $t:expr) => (
    if (perform_near($a, $b, $t, false)) { _successes += 1; } else { _failure = true; _fails += 1; }
  );
)

macro_rules! wont(
  ($a:expr eq $b:expr) => (
    if (perform_test($a, $b, true)) { _successes += 1; } else { _failure = true; _fails += 1; }
  );
  ($a:expr near $b:expr) => (
    if (perform_near($a, $b, 0.00001, true)) { _successes += 1; } else { _failure = true; _fails += 1; }
  );
  ($a:expr near $b:expr within $t:expr) => (
    if (perform_near($a, $b, $t, true)) { _successes += 1; } else { _failure = true; _fails += 1; }
  );
)

macro_rules! should(
  ($prompt:expr, $func:expr) => ({
    let mut _failure = false;

    _tests += 1;
    _indent += 1;

    do uint::range_step(0, _indent, 1) |_| {
      io::print(" ");

      true
    };

    io::print("should ");
    io::print($prompt);

    $func;

    if (!_failure) {
      io::print(" - ");
      io::println("\x1b[32;1mPass\x1b[39;0m");
    }
    else {
      io::println("");
    }

    _indent -= 1;
  })
)