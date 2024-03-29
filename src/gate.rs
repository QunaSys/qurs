use crate::binding::{qulacs, wrap, ControlValue, Gate};
use num::{Complex, One, Zero};

/// Apply the Pauli X gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn x_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::single_of(target_qubit_index, qulacs::X_gate));
}

/// Apply the Pauli Y gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn y_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::single_of(target_qubit_index, qulacs::Y_gate));
}

/// Apply the Pauli Y gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn z_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::single_of(target_qubit_index, qulacs::Z_gate));
}

/// Apply the Hadamard gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn h_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::single_of(target_qubit_index, qulacs::H_gate));
}

/// Project the quantum state to the 0 state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn p0_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::single_of(target_qubit_index, qulacs::P0_gate));
}

/// Project the quantum state to the 1 state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn p1_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::single_of(target_qubit_index, qulacs::P1_gate));
}

/// Apply S gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn s_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::single_of(target_qubit_index, qulacs::S_gate));
}

/// Apply S^dag gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn sdag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::single_of(target_qubit_index, qulacs::Sdag_gate),
	);
}

/// Apply T gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn t_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::single_of(target_qubit_index, qulacs::T_gate));
}

/// Apply T^dag gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn tdag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::single_of(target_qubit_index, qulacs::Tdag_gate),
	);
}

/// Apply the square root of the X gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn sqrtx_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::single_of(target_qubit_index, qulacs::sqrtX_gate),
	);
}

/// Apply hermitian conjugate of the square root of the X gate to the quantum
/// state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn sqrtxdag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::single_of(target_qubit_index, qulacs::sqrtXdag_gate),
	);
}

/// Apply the square root of the Y gate to the quantum state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn sqrty_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::single_of(target_qubit_index, qulacs::sqrtY_gate),
	);
}

/// Apply hermitian conjugate of the square root of the Y gate to the quantum
/// state.
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn sqrtydag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::single_of(target_qubit_index, qulacs::sqrtYdag_gate),
	);
}

/// Apply the CZ gate to the quantum state.
/// * `control_qubit_index` index of control qubit
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn cz_gate(control_qubit_index: u32, target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Controlled {
			control_qubit_index,
			target_qubit_index,
			gate: qulacs::CZ_gate,
		},
	);
}

/// Apply the CNOT gate to the quantum state.
/// * `control_qubit_index` index of control qubit
/// * `target_qubit_index` index of the qubit
/// * `state` quantum state
pub fn cnot_gate(control_qubit_index: u32, target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Controlled {
			control_qubit_index,
			target_qubit_index,
			gate: qulacs::CNOT_gate,
		},
	);
}

/// Apply the CCNOT(a.k.a Toffoli) gate to the quantum state.
pub fn ccnot_gate(
	control_qubit_index1: u32,
	control_qubit_index2: u32,
	target_qubit_index: u32,
	state: &mut [Complex<f64>],
) {
	multi_control_u_gate(
		&[control_qubit_index1, control_qubit_index2],
		target_qubit_index,
		&[
			Complex::zero(),
			Complex::one(),
			Complex::one(),
			Complex::zero(),
		],
		state,
	);
}

/// Apply the CCZ gate to the quantum state.
pub fn ccz_gate(
	control_qubit_index1: u32,
	control_qubit_index2: u32,
	target_qubit_index: u32,
	state: &mut [Complex<f64>],
) {
	multi_control_u_gate(
		&[control_qubit_index1, control_qubit_index2],
		target_qubit_index,
		&[
			Complex::one(),
			Complex::zero(),
			Complex::zero(),
			-Complex::one(),
		],
		state,
	);
}

/// Apply arbitrary multi-controlled gate represented as `matrix`.
/// * `control_qubit_indexes` if these qubits are `ControlValue::One` then apply
///   `matrix`
/// * `target_qubit_index` the target applied `matrix`
/// * `state` quantum state
fn multi_control_u_gate(
	control_qubit_indexes: &[u32],
	target_qubit_index: u32,
	matrix: &[Complex<f64>; 4],
	state: &mut [Complex<f64>],
) {
	let controls = control_qubit_indexes
		.iter()
		.map(|x| -> (u32, ControlValue) { (*x, ControlValue::One) })
		.collect::<Vec<_>>();
	wrap(
		state,
		Gate::MultiControlledSingleTarget {
			controls: controls.as_slice(),
			target_index: target_qubit_index,
			matrix,
			gate: qulacs::multi_qubit_control_single_qubit_dense_matrix_gate,
		},
	);
}

/// Apply the SWAP to the quantum state.
/// * `target_qubit_index_0` index of the first target qubit
/// * `target_qubit_index_1` index of the second target qubit
/// * `state` quantum state
pub fn swap_gate(target_qubit_index_0: u32, target_qubit_index_1: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Controlled {
			control_qubit_index: target_qubit_index_0,
			target_qubit_index: target_qubit_index_1,
			gate: qulacs::SWAP_gate,
		},
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
		Gate::rotation_of(target_qubit_index, -angle, qulacs::RX_gate),
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
		Gate::rotation_of(target_qubit_index, -angle, qulacs::RY_gate),
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
		Gate::rotation_of(target_qubit_index, -angle, qulacs::RZ_gate),
	);
}
