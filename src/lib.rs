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

pub fn permutate_qubit<T>(state: T, qubit_order: &[u32]) -> T
where
	T: StateRef<f64> + AsRef<[Complex<f64>]> + AsMut<[Complex<f64>]> + Clone,
{
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
	result
}

pub fn drop_qubit<T>(state: &T, target: &[u32], projection: &[u32]) -> StateVec<f64>
where
	T: StateRef<f64> + AsRef<[Complex<f64>]>,
{
	if state.qubit_count() <= target.len() || target.len() != projection.len() {
		panic!("Invalid qubit count");
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
	qs
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

	let mut state = [zero(); 8];
	state.set_haar_random_state();
	let permutated_state = permutate_qubit(state, &[1, 0, 2]);
	let corr = [0, 2, 1, 3, 4, 6, 5, 7];
	for i in 0..state.as_ref().len() {
		assert_eq!(permutated_state.as_ref()[i].re, state[corr[i]].re);
		assert_eq!(permutated_state.as_ref()[i].im, state[corr[i]].im);
	}

	let mut state = [zero(); 16];
	state.set_haar_random_state();
	let dropped_state = drop_qubit(&state, &[2, 0], &[0, 1]);
	assert_eq!(dropped_state.as_ref().len(), 4);
	let corr = [1, 3, 9, 11];

	for i in 0..dropped_state.as_ref().len() {
		assert_eq!(dropped_state.as_ref()[i].re, state[corr[i]].re);
		assert_eq!(dropped_state.as_ref()[i].im, state[corr[i]].im);
	}
}
