pub use num::complex::Complex;
mod binding;
use binding::{qulacs, wrap, Gate, CTYPE};
pub mod state;
pub use state::*;

/// Apply the Pauli X gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn x_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::X_gate));
}

/// Apply the Pauli Y gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn y_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::Y_gate));
}

/// Apply the Pauli Y gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn z_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::Z_gate));
}

/// Apply the Hadamard gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn h_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::H_gate));
}

/// Project the quantum state to the 0 state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn p0_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::P0_gate));
}

/// Project the quantum state to the 1 state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn p1_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::P1_gate));
}

/// Apply S gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn s_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::S_gate));
}

/// Apply S^dag gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn sdag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::Sdag_gate));
}

/// Apply T gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn t_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::T_gate));
}

/// Apply T^dag gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn tdag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::Tdag_gate));
}

/// Apply the square root of the X gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn sqrtx_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::sqrtX_gate));
}

/// Apply hermitian conjugate of the square root of the X gate to the quantum
/// state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn sqrtxdag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Single(target_qubit_index, qulacs::sqrtXdag_gate),
	);
}

/// Apply the square root of the Y gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn sqrty_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::sqrtY_gate));
}

/// Apply hermitian conjugate of the square root of the Y gate to the quantum
/// state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn sqrtydag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Single(target_qubit_index, qulacs::sqrtYdag_gate),
	);
}

/// Apply the CZ gate to the quantum state.
/// * `control_qubit_index` index of control qubit
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn cz_gate(control_qubit_index: u32, target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Controlled(control_qubit_index, target_qubit_index, qulacs::CZ_gate),
	);
}

/// Apply the CNOT gate to the quantum state.
/// * `control_qubit_index` index of control qubit
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn cnot_gate(control_qubit_index: u32, target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Controlled(control_qubit_index, target_qubit_index, qulacs::CNOT_gate),
	);
}

/// Apply the SWAP to the quantum state.
/// * `target_qubit_index_0` index of the first target qubit
/// * `target_qubit_index_1` index of the second target qubit
/// * `state` quantum state
pub fn swap_gate(target_qubit_index_0: u32, target_qubit_index_1: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Controlled(
			target_qubit_index_0,
			target_qubit_index_1,
			qulacs::SWAP_gate,
		),
	);
}

/// Apply a X rotation gate by angle to the quantum state. The definition is
/// given as exp(-i(θ/2)X) where θ is the angle.
/// * `target_qubit_index` index of the qubit
/// * `angle` angle of the rotation
/// * `state` quantum state
pub fn rx_gate(target_qubit_index: u32, angle: f64, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Rotation(target_qubit_index, -angle, qulacs::RX_gate),
	);
}

/// Apply a Y rotation gate by angle to the quantum state. The definition is
/// given as exp(-i(θ/2)Y) where θ is the angle.
/// * `target_qubit_index` index of the qubit
/// * `angle` angle of the rotation
/// * `state` quantum state
pub fn ry_gate(target_qubit_index: u32, angle: f64, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Rotation(target_qubit_index, -angle, qulacs::RY_gate),
	);
}

/// Apply a Z rotation gate by angle to the quantum state. The definition is
/// given as exp(-i(θ/2)Z) where θ is the angle.
/// * `target_qubit_index` index of the qubit
/// * `angle` angle of the rotation
/// * `state` quantum state
pub fn rz_gate(target_qubit_index: u32, angle: f64, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Rotation(target_qubit_index, -angle, qulacs::RZ_gate),
	);
}

///Compute the inner product of quantum states.
pub fn inner_product(
	state_bra: &[Complex<f64>],
	state_ket: &[Complex<f64>],
) -> Result<Complex<f64>, StateErr> {
	if state_bra.len() != state_ket.len() {
		return Err(StateErr::InconsistentStateLength(
			state_bra.len(),
			state_ket.len(),
		));
	}
	unsafe {
		let CTYPE { re, im } = qulacs::state_inner_product(
			state_bra.as_ptr() as *const CTYPE,
			state_ket.as_ptr() as *const CTYPE,
			state_bra.len() as u64,
		);
		Ok(Complex::new(re, im))
	}
}

///Get tensor product of states
pub fn tensor_product<L, R>(state_left: &L, state_right: &R) -> StateVec<f64>
where
	L: StateRef<f64> + AsRef<[Complex<f64>]>,
	R: StateRef<f64> + AsRef<[Complex<f64>]>,
{
	let mut result = StateVec::<f64>::new(state_left.qubit_count() + state_right.qubit_count());
	unsafe {
		qulacs::state_tensor_product(
			state_left.as_ref().as_ptr() as *const CTYPE,
			state_left.as_ref().len() as u64,
			state_right.as_ref().as_ptr() as *const CTYPE,
			state_right.as_ref().len() as u64,
			result.as_mut().as_mut_ptr() as *mut CTYPE,
		)
	}
	result
}

///Permutate qubits from state
pub fn permutate_qubit<T>(state: T, qubit_order: &[u32]) -> Result<T, StateErr>
where
	T: StateRef<f64> + AsRef<[Complex<f64>]> + AsMut<[Complex<f64>]> + Clone,
{
	if state.qubit_count() != qubit_order.len() {
		return Err(StateErr::InvalidTargetList(qubit_order.to_vec()));
	}
	let mut result = state.clone();
	unsafe {
		qulacs::state_permutate_qubit(
			qubit_order.as_ptr() as *const u32,
			state.as_ref().as_ptr() as *const CTYPE,
			result.as_mut().as_mut_ptr() as *mut CTYPE,
			state.qubit_count() as u32,
			state.as_ref().len() as u64,
		)
	}
	Ok(result)
}

///Drop qubits from state
pub fn drop_qubit<T>(
	state: &T,
	target: &[u32],
	projection: &[u32],
) -> Result<StateVec<f64>, StateErr>
where
	T: StateRef<f64> + AsRef<[Complex<f64>]>,
{
	if state.qubit_count() <= target.len() || target.len() != projection.len() {
		return Err(StateErr::InvalidTargetList(target.to_vec()));
	}
	let qubit_count = state.qubit_count() - target.len();
	let mut qs = StateVec::new(qubit_count);
	unsafe {
		qulacs::state_drop_qubits(
			target.as_ptr() as *const u32,
			projection.as_ptr() as *const u32,
			target.len() as u32,
			state.as_ref().as_ptr() as *const CTYPE,
			qs.as_mut().as_mut_ptr() as *mut CTYPE,
			state.as_ref().len() as u64,
		);
	}
	Ok(qs)
}

///Get expectation value
pub fn expectation_value_multi_qubit_pauli_operator_partial_list<T>(
	target_qubit_index_list: &[u32],
	pauli_operator_type_list: &[u32],
	state: &T,
) -> f64
where
	T: StateRef<f64> + AsRef<[Complex<f64>]>,
{
	unsafe {
		qulacs::expectation_value_multi_qubit_Pauli_operator_partial_list_single_thread(
			target_qubit_index_list.as_ptr() as *const u32,
			pauli_operator_type_list.as_ptr() as *const u32,
			target_qubit_index_list.len() as u32,
			state.as_ref().as_ptr() as *const CTYPE,
			state.as_ref().len() as u64,
		)
	}
}

///Get expectation value
pub fn multi_qubit_pauli_rotation_gate_partial_list<T>(
	target_qubit_index_list: &[u32],
	pauli_operator_type_list: &[u32],
	angle: f64,
	state: &mut T,
) where
	T: StateRef<f64> + AsMut<[Complex<f64>]>,
{
	unsafe {
		qulacs::multi_qubit_Pauli_rotation_gate_partial_list(
			target_qubit_index_list.as_ptr() as *const u32,
			pauli_operator_type_list.as_ptr() as *const u32,
			target_qubit_index_list.len() as u32,
			angle,
			state.as_mut().as_ptr() as *mut CTYPE,
			state.as_mut().len() as u64,
		)
	}
}

#[test]
fn test_lib() {
	use num::{One, Zero};
	let one = Complex::one;
	let zero = Complex::zero;

	assert_eq!([one(); 2].as_ref().len(), 2);
	let mut state = [one(), zero(), zero(), zero()];
	x_gate(1, &mut state);
	assert_eq!(state, [zero(), zero(), one(), zero()]);

	assert_eq!(inner_product(&state, &state).unwrap(), one());

	let state = [one(), zero()];
	assert_eq!(
		tensor_product(&state, &state).as_ref(),
		[one(), zero(), zero(), zero()]
	);

	let mut state = [zero(); 8];
	state.set_haar_random_state();
	let permutated_state = permutate_qubit(state, &[1, 0, 2]).unwrap();
	let corr = [0, 2, 1, 3, 4, 6, 5, 7];
	for i in 0..state.as_ref().len() {
		assert_eq!(permutated_state.as_ref()[i].re, state[corr[i]].re);
		assert_eq!(permutated_state.as_ref()[i].im, state[corr[i]].im);
	}

	let mut state = [zero(); 16];
	state.set_haar_random_state();
	let dropped_state = drop_qubit(&state, &[2, 0], &[0, 1]).unwrap();
	assert_eq!(dropped_state.as_ref().len(), 4);
	let corr = [1, 3, 9, 11];

	for i in 0..dropped_state.as_ref().len() {
		assert_eq!(dropped_state.as_ref()[i].re, state[corr[i]].re);
		assert_eq!(dropped_state.as_ref()[i].im, state[corr[i]].im);
	}

	let state = [one(); 4];
	let expected =
		expectation_value_multi_qubit_pauli_operator_partial_list(&[0, 1], &[1, 1], &state);
	assert_eq!(expected, 4.);
}
