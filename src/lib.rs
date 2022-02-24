pub use num::complex::Complex;
mod binding;
use binding::{wrap, Gate};

pub fn x_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::SingleGate(target_qubit_index, binding::X_gate));
}

pub fn y_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::SingleGate(target_qubit_index, binding::Y_gate));
}

pub fn z_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::SingleGate(target_qubit_index, binding::Z_gate));
}

pub fn h_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::SingleGate(target_qubit_index, binding::H_gate));
}

pub fn p0_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::SingleGate(target_qubit_index, binding::P0_gate),
	);
}

pub fn p1_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::SingleGate(target_qubit_index, binding::P1_gate),
	);
}

pub fn s_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::SingleGate(target_qubit_index, binding::S_gate));
}

pub fn sdag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::SingleGate(target_qubit_index, binding::Sdag_gate),
	);
}

pub fn t_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::SingleGate(target_qubit_index, binding::T_gate));
}

pub fn tdag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::SingleGate(target_qubit_index, binding::Tdag_gate),
	);
}

pub fn sqrtx_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::SingleGate(target_qubit_index, binding::sqrtX_gate),
	);
}

pub fn sqrtxdag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::SingleGate(target_qubit_index, binding::sqrtXdag_gate),
	);
}

pub fn sqrty_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::SingleGate(target_qubit_index, binding::sqrtY_gate),
	);
}

pub fn sqrtydag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::SingleGate(target_qubit_index, binding::sqrtYdag_gate),
	);
}
pub fn cz_gate(control_qubit_index: u32, target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::ControlledGate(control_qubit_index, target_qubit_index, binding::CZ_gate),
	);
}

pub fn cnot_gate(control_qubit_index: u32, target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::ControlledGate(control_qubit_index, target_qubit_index, binding::CNOT_gate),
	);
}

pub fn swap_gate(control_qubit_index: u32, target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::ControlledGate(control_qubit_index, target_qubit_index, binding::SWAP_gate),
	);
}

pub fn rx_gate(control_qubit_index: u32, angle: f64, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::RotationGate(control_qubit_index, angle, binding::RX_gate),
	);
}

pub fn ry_gate(control_qubit_index: u32, angle: f64, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::RotationGate(control_qubit_index, angle, binding::RY_gate),
	);
}

pub fn rz_gate(control_qubit_index: u32, angle: f64, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::RotationGate(control_qubit_index, angle, binding::RZ_gate),
	);
}

#[test]
fn test() {
	let c = |re: f64, im: f64| Complex::new(re, im);
	let mut state = vec![c(1., 0.), c(0., 0.), c(0., 0.), c(0., 0.)];
	x_gate(1, &mut state);
	assert_eq!(state, vec![c(0., 0.), c(0., 0.), c(1., 0.), c(0., 0.),]);
}
