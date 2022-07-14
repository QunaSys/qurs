use num::{Complex, One, Zero};
use qurs::{self, inner_product, ry_gate, x_gate, StateMut, StateRef, StateVec};
use std::f64::consts::PI;

#[test]
fn generate_state() {
	const N: usize = 5;

	let mut zero_state = vec![Complex::zero(); 2usize.pow(N as u32)];
	zero_state[0] = Complex::one();

	//Using fixed size array for quantum state.
	//The traits StateRef and StateMut are implemented for array with
	//2^N length.
	let mut state = [Complex::zero(); 2usize.pow(N as u32)];
	state.set_zero_state();

	assert_eq!(state.qubit_count(), N);
	assert_eq!(state.as_ref(), zero_state);

	//Using growable size array (vector) for quantum state.
	//The traits StateRef and StateMut are implemented for struct StateVec.
	let state = StateVec::new(N);

	assert_eq!(state.qubit_count(), N);
	//To get an underlying vector, call as_ref().
	assert_eq!(state.as_ref(), zero_state);
}
#[test]
fn init_state() {
	const N: usize = 5;

	//With array
	let mut state = [Complex::zero(); 2usize.pow(N as u32)];
	state.set_zero_state();
	// initialize to |00101>
	state.set_computational_basis(0b00101);
	// initialize with a random quantum state
	state.set_haar_random_state();
	// initialize with a random quantum state with seed
	state.set_haar_random_state_with_seed(0);

	//With StateVec
	let mut state = StateVec::new(N);
	state.set_zero_state();
	state.set_computational_basis(0b00101);
	state.set_haar_random_state();
	state.set_haar_random_state_with_seed(0);
}

#[test]
fn inner_state() {
	const N: usize = 5;
	let mut state_ket = [Complex::zero(); 2usize.pow(N as u32)];
	state_ket.set_zero_state();

	let mut state_bra = [Complex::zero(); 2usize.pow(N as u32)];
	state_bra.set_haar_random_state();

	//inner_product returns Result<Complex<f64>, StateErr>.
	if let Ok(value) = inner_product(&state_ket, &state_bra) {
		println!("{value}");
	}
}

#[test]
fn state_calc() {
	const N: usize = 5;
	let mut state = [Complex::zero(); 2usize.pow(N as u32)];
	state.set_zero_state();
	// calculate norm
	let _ = state.get_squared_norm();
	// calculate the entropy measured in the Z-basis
	let _ = state.get_entropy();

	// the probability of getting 0 by measuring the index-th qubit in the Z-basis.
	let index = 3;
	let _ = state.get_zero_probability(index);

	// calculate marginal probability (for the following case; measured 0 for
	// 0,3-th, and measured 1 for 1,2-th qubits)
	let _ = state.get_marginal_probability(&[0, 1, 2, 3], &[0, 1, 1, 0]);
}

#[test]
fn apply_gate() {
	const N: usize = 5;
	let mut state = [Complex::zero(); 2usize.pow(N as u32)];
	state.set_zero_state();
	// Apply x_gate for state
	x_gate(0, &mut state);

	// Rotate PI/2 with respect to Y
	let angle = PI / 2.0;
	ry_gate(0, angle, &mut state);
}
