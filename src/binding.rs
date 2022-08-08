pub use num::complex::Complex;

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(improper_ctypes)]
#[allow(unused)]
pub mod qulacs {
	include!(concat!(env!("OUT_DIR"), "/qulacs.rs"));
}
pub use qulacs::CTYPE;

pub enum Gate {
	Single {
		target_qubit_index: u32,
		gate: unsafe extern "C" fn(u32, *mut CTYPE, u64),
	},
	Controlled {
		control_qubit_index: u32,
		target_qubit_index: u32,
		gate: unsafe extern "C" fn(u32, u32, *mut CTYPE, u64),
	},
	Rotation {
		target_qubit_index: u32,
		/// `angle` of the rotation (radian)
		angle: f64,
		gate: unsafe extern "C" fn(u32, f64, *mut CTYPE, u64),
	},
}

/// Custom constructors.
impl Gate {
	pub fn single_of(
		target_qubit_index: u32,
		gate: unsafe extern "C" fn(u32, *mut CTYPE, u64),
	) -> Gate {
		Gate::Single {
			target_qubit_index,
			gate,
		}
	}

	pub fn rotation_of(
		target_qubit_index: u32,
		angle: f64,
		gate: unsafe extern "C" fn(u32, f64, *mut CTYPE, u64),
	) -> Gate {
		Gate::Rotation {
			target_qubit_index,
			angle,
			gate,
		}
	}
}

pub fn wrap(state: &mut [Complex<f64>], gate: Gate) {
	let dim = state.len() as u64;
	let state_ptr = state.as_mut_ptr() as *mut CTYPE;
	unsafe {
		match gate {
			Gate::Single {
				target_qubit_index,
				gate,
			} => {
				gate(target_qubit_index, state_ptr, dim);
			}
			Gate::Controlled {
				control_qubit_index,
				target_qubit_index,
				gate,
			} => {
				gate(control_qubit_index, target_qubit_index, state_ptr, dim);
			}
			Gate::Rotation {
				target_qubit_index,
				angle,
				gate,
			} => {
				gate(target_qubit_index, angle, state_ptr, dim);
			}
		}
	}
}
