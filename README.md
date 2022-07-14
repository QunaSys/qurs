# Qurs

Qurs is an implementation of Qulacs written in Rust. Qurs currently supports only a single processor and depends only on Qulacs C code (`qulacs/src/csim`).

Qulacs: https://github.com/qulacs

Qurs is licensed under the MIT license.

# Build

Qurs uses [cc](https://github.com/rust-lang/cc-rs) and [bindgen](https://github.com/rust-lang/rust-bindgen) to build Qulacs code.
The requirements for bindgen are listed in
https://rust-lang.github.io/rust-bindgen/requirements.html

```sh
cargo build
```

# Example

```rust
use num::{Complex, One, Zero};
use qurs::{self, StateMut, StateRef, StateVec};
fn main() {
	const N: usize = 5;

	//With array
	let mut state = [Complex::zero(); 2usize.pow(N as u32)];
	assert_eq!(state.qubit_count(), N);
	// initialize to |00000>
	state.set_zero_state();
	// initialize to |00101>
	state.set_computational_basis(0b00101);
	// initialize with a random quantum state
	state.set_haar_random_state();
	// Apply x_gate for state
	x_gate(0, &mut state);
	// Rotate PI/2 with respect to Y
	let angle = PI / 2.0;
	ry_gate(0, angle, &mut state);

	//With StateVec
	let mut state = StateVec::new(N);
	assert_eq!(state.qubit_count(), N);
	state.set_zero_state();
	state.set_computational_basis(0b00101);
	state.set_haar_random_state();
	x_gate(0, state.as_mut());
	ry_gate(0, angle, state.as_mut());
}
```
