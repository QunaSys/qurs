include!(concat!(env!("OUT_DIR"), "/qulacs.rs"));
pub use num::complex::Complex;

pub enum Gate {
	SingleGate(u32, unsafe extern "C" fn(u32, *mut CTYPE, u64)),
	ControlledGate(u32, u32, unsafe extern "C" fn(u32, u32, *mut CTYPE, u64)),
	RotationGate(u32, f64, unsafe extern "C" fn(u32, f64, *mut CTYPE, u64)),
}

pub fn wrap(state: &mut [Complex<f64>], gate: Gate) {
	let dim = state.len() as u64;
	let state_ptr = state.as_mut_ptr() as *mut CTYPE;
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
}
