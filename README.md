# Qurs

Qurs is an implementation of Qulacs written in Rust. It currently supports only a single processor and depends only on Qulacs C code (`qulacs/src/csim`).

Qulacs:https://github.com/qulacs

Qurs is licensed under the MIT license.

# Build

Qurs uses [cc](https://github.com/rust-lang/cc-rs) and [bindgen](https://github.com/rust-lang/rust-bindgen) to build Qulacs code.
The requirments for bindgen are listed in
https://rust-lang.github.io/rust-bindgen/requirements.html

```sh
cargo build
```

# Example

```rust
use num::{Complex, One, Zero};
use qurs::{self, StateMut, StateRef, StateVec};
fn main() {
	//Using array
	let mut state = [
		Complex::one(),
		Complex::zero(),
		Complex::zero(),
		Complex::zero(),
	];
	state.set_haar_random_state();
	state.get_marginal_probability(&[0, 0]);
	qurs::x_gate(1, &mut state);
	qurs::inner_product(&state, &state);

	//Using StateVec
	let mut state: StateVec<f64> = StateVec::new(2);
	state.set_haar_random_state();
	state.get_marginal_probability(&[0, 0]);
	qurs::x_gate(1, state.as_mut());
	qurs::inner_product(state.as_ref(), state.as_ref());
}
```
