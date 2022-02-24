include!(concat!(env!("OUT_DIR"), "/qulacs.rs"));
pub use num::complex::Complex;

pub enum Gate {
	SingleGate(u32, SingleGate),
	ControlledGate(u32, u32, ControlledGate),
	RotationGate(u32, f64, RotationGate),
}

type SingleGate = unsafe extern "C" fn(target_qubit_index: u32, state: *mut CTYPE, dim: u64);

type ControlledGate = unsafe extern "C" fn(
	conrol_qubit_index: u32,
	target_qubit_index: u32,
	state: *mut CTYPE,
	dim: u64,
);

type RotationGate =
	unsafe extern "C" fn(target_qubit_index: u32, angle: f64, state: *mut CTYPE, dim: u64);

pub fn wrap(state: &mut [Complex<f64>], gate: Gate) {
	let mut new_state = comp_to_ctype(state);
	let dim = new_state.len() as u64;
	let state_ptr = new_state.as_mut_ptr();
	unsafe {
		match gate {
			Gate::SingleGate(target_qubit_index, gate) => {
				gate(target_qubit_index, state_ptr, dim);
			}
			Gate::ControlledGate(control_qubit_index, target_qubit_index, gate) => {
				gate(control_qubit_index, target_qubit_index, state_ptr, dim);
			}
			Gate::RotationGate(target_qubit_index, angle, gate) => {
				gate(target_qubit_index, angle, state_ptr, dim);
			}
		}
	}
	for i in 0..dim as usize {
		state[i] = Complex::new(new_state[i].re, new_state[i].im);
	}
}

fn comp_to_ctype(state: &mut [Complex<f64>]) -> Vec<CTYPE> {
	state
		.into_iter()
		.map(|e| CTYPE { re: e.re, im: e.im })
		.collect::<Vec<CTYPE>>()
}
