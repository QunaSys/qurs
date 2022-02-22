pub use num::complex::Complex;
mod binding;
use binding::{bindings, Gate};
#[test]
fn test() {
	let mut state = vec![
		Complex::new(1., 0.),
		Complex::new(0., 0.),
		Complex::new(0., 0.),
		Complex::new(0., 0.),
	];
	sqrtx_gate(0, &mut state);
	dbg!(state);
}

pub fn x_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(state, Gate::SingleGate(target_qubit_index, binding::X_gate));
}

pub fn y_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(state, Gate::SingleGate(target_qubit_index, binding::Y_gate));
}

pub fn z_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(state, Gate::SingleGate(target_qubit_index, binding::Z_gate));
}

pub fn h_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(state, Gate::SingleGate(target_qubit_index, binding::H_gate));
}

pub fn p0_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(
		state,
		Gate::SingleGate(target_qubit_index, binding::P0_gate),
	);
}

pub fn p1_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(
		state,
		Gate::SingleGate(target_qubit_index, binding::P1_gate),
	);
}

pub fn s_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(state, Gate::SingleGate(target_qubit_index, binding::S_gate));
}
pub fn sdag_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(
		state,
		Gate::SingleGate(target_qubit_index, binding::Sdag_gate),
	);
}
pub fn t_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(state, Gate::SingleGate(target_qubit_index, binding::T_gate));
}
pub fn tdag_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(
		state,
		Gate::SingleGate(target_qubit_index, binding::Tdag_gate),
	);
}

pub fn sqrtx_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(
		state,
		Gate::SingleGate(target_qubit_index, binding::sqrtX_gate),
	);
}
pub fn sqrtxdag_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(
		state,
		Gate::SingleGate(target_qubit_index, binding::sqrtXdag_gate),
	);
}
pub fn sqrty_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(
		state,
		Gate::SingleGate(target_qubit_index, binding::sqrtY_gate),
	);
}
pub fn sqrtydag_gate(target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(
		state,
		Gate::SingleGate(target_qubit_index, binding::sqrtYdag_gate),
	);
}
pub fn cz_gate(control_qubit_index: u32, target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(
		state,
		Gate::ControlledGate(control_qubit_index, target_qubit_index, binding::CZ_gate),
	);
}

pub fn cnot_gate(control_qubit_index: u32, target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(
		state,
		Gate::ControlledGate(control_qubit_index, target_qubit_index, binding::CNOT_gate),
	);
}

pub fn swap_gate(control_qubit_index: u32, target_qubit_index: u32, state: &mut Vec<Complex<f64>>) {
	bindings(
		state,
		Gate::ControlledGate(control_qubit_index, target_qubit_index, binding::SWAP_gate),
	);
}
