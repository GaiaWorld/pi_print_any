output without the trait bounds (using specialization to
find the right impl anyway)

output value for type of impl Debug, output type name for unimplDebug.

# for example:
```
	#[derive(Debug)]
	struct A(usize);
	struct B(usize);
	fn main() {
		println!("{:?}", A(1)); // output: A(1)
		println!("{:?}", B(1)); // output: `pi_print_any::B`
	}
```