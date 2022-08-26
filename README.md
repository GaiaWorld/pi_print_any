output without the trait bounds (using specialization to
find the right impl anyway)

output value for type of impl Debug, output type name for unimplDebug.

# for example:
```
	#[derive(Debug)]
	struct A(usize);
	struct B(usize);
	fn main() {
		println_any!("{:?}", A(1)); // output: A(1)
		println_any!("{:?}", B(1)); // output: `pi_print_any::B`

		print_any!("{:?}", A(1)); // output: A(1)
		print_any!("{:?}", B(1)); // output: `pi_print_any::B`
	}
```

In addition to using print and println output, you can also use other macros to output, `out_any` allows you to pass in the output macro you want to use

# for example:
```
	#[derive(Debug)]
	struct A(usize);
	struct B(usize);
	fn main() {
		out_any!(log::info, "{:?}", A(1)); // output: A(1)
		out_any!(log::info, "{:?}", B(1)); // output: `pi_print_any::B`
	}
```