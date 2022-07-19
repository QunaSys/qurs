use crate::binding::{qulacs, wrap, Gate};
use num::Complex;

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
