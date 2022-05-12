pub use num::complex::Complex;
mod binding;
use binding::{qulacs, wrap, Gate, CTYPE};
pub mod state;
pub use state::*;

pub fn x_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::X_gate));
}

pub fn y_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::Y_gate));
}

pub fn z_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::Z_gate));
}

pub fn h_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::H_gate));
}

pub fn p0_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::P0_gate));
}

pub fn p1_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::P1_gate));
}

pub fn s_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::S_gate));
}

pub fn sdag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::Sdag_gate));
}

pub fn t_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::T_gate));
}

pub fn tdag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::Tdag_gate));
}

pub fn sqrtx_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::sqrtX_gate));
}

pub fn sqrtxdag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Single(target_qubit_index, qulacs::sqrtXdag_gate),
	);
}

pub fn sqrty_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(state, Gate::Single(target_qubit_index, qulacs::sqrtY_gate));
}

pub fn sqrtydag_gate(target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Single(target_qubit_index, qulacs::sqrtYdag_gate),
	);
}
pub fn cz_gate(control_qubit_index: u32, target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Controlled(control_qubit_index, target_qubit_index, qulacs::CZ_gate),
	);
}

pub fn cnot_gate(control_qubit_index: u32, target_qubit_index: u32, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Controlled(control_qubit_index, target_qubit_index, qulacs::CNOT_gate),
	);
}

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

pub fn rx_gate(target_qubit_index: u32, angle: f64, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Rotation(target_qubit_index, angle, qulacs::RX_gate),
	);
}

pub fn ry_gate(target_qubit_index: u32, angle: f64, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Rotation(target_qubit_index, angle, qulacs::RY_gate),
	);
}

pub fn rz_gate(target_qubit_index: u32, angle: f64, state: &mut [Complex<f64>]) {
	wrap(
		state,
		Gate::Rotation(target_qubit_index, angle, qulacs::RZ_gate),
	);
}

pub fn inner_product(state_bra: &[Complex<f64>], state_ket: &[Complex<f64>]) -> Complex<f64> {
	unsafe {
		let CTYPE { re, im } = qulacs::state_inner_product(
			state_bra.as_ptr() as *const CTYPE,
			state_ket.as_ptr() as *const CTYPE,
			state_bra.len() as u64,
		);
		Complex::new(re, im)
	}
}

pub fn tensor_product<T>(state_left: &T, state_right: &T) -> StateVec<f64>
where
	T: StateRef<f64> + AsRef<[Complex<f64>]>,
{
	let mut result = StateVec::<f64>::new(state_left.len() + state_right.len());
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

pub fn permutate_qubit<T>(state: &T, qubit_order: &[u64]) -> StateVec<f64>
where
	T: StateRef<f64> + AsRef<[Complex<f64>]>,
{
	let mut result = StateVec::<f64>::new(state.len());
	unsafe {
		qulacs::state_permutate_qubit(
			qubit_order.as_ptr() as *const u32,
			state.as_ref().as_ptr() as *const CTYPE,
			result.as_mut().as_mut_ptr() as *mut CTYPE,
			state.len() as u32,
			state.as_ref().len() as u64,
		)
	}
	result
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
	assert_eq!(inner_product(&state, &state), one());
	let state = [one(), zero()];
	assert_eq!(
		tensor_product(&state, &state).as_ref(),
		[one(), zero(), zero(), zero()]
	);
}
