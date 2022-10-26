pub use num::complex::Complex;

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(improper_ctypes)]
#[allow(unused)]
pub mod qulacs {
	include!(concat!(env!("OUT_DIR"), "/qulacs.rs"));
}
pub use qulacs::{CTYPE, UINT};

/// Value indicate to apply the gate in controlled gate.
/// Note: Rust requires that we implement `Copy` trait to cast `ControlValue`
/// into `u32`.
#[derive(Clone, Copy)]
pub enum ControlValue {
	/// For now we don't use `Zero`.
	#[allow(dead_code)]
	Zero,
	One,
}

pub enum Gate<'a> {
	Single {
		target_qubit_index: u32,
		gate: unsafe extern "C" fn(u32, *mut CTYPE, u64),
	},
	Controlled {
		control_qubit_index: u32,
		target_qubit_index: u32,
		gate: unsafe extern "C" fn(u32, u32, *mut CTYPE, u64),
	},
	MultiControlledSingleTarget {
		// (control_index, control_value)
		controls: &'a [(u32, ControlValue)],
		target_index: u32,
		matrix: &'a [Complex<f64>; 4],
		gate: unsafe extern "C" fn(*const u32, *const u32, u32, u32, *const CTYPE, *mut CTYPE, u64),
	},
	Rotation {
		target_qubit_index: u32,
		/// `angle` of the rotation (radian)
		angle: f64,
		gate: unsafe extern "C" fn(u32, f64, *mut CTYPE, u64),
	},
}

/// Custom constructors.
impl Gate<'_> {
	pub fn single_of<'a>(
		target_qubit_index: u32,
		gate: unsafe extern "C" fn(u32, *mut CTYPE, u64),
	) -> Gate<'a> {
		Gate::Single {
			target_qubit_index,
			gate,
		}
	}

	pub fn rotation_of<'a>(
		target_qubit_index: u32,
		angle: f64,
		gate: unsafe extern "C" fn(u32, f64, *mut CTYPE, u64),
	) -> Gate<'a> {
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
			Gate::MultiControlledSingleTarget {
				controls,
				target_index,
				matrix,
				gate,
			} => {
				let control_qubit_count = controls.len() as u32;

				let mut control_qubit_index_list: Vec<u32> = Vec::new();
				let mut control_value_list: Vec<u32> = Vec::new();
				for c in controls.iter() {
					control_qubit_index_list.push(c.0);
					control_value_list.push(c.1 as u32);
				}
				let control_qubit_index_list_ptr = control_qubit_index_list.as_ptr() as *const UINT;
				let control_value_list_ptr = control_value_list.as_ptr() as *const UINT;
				let matrix_ptr = matrix.as_ptr() as *const CTYPE;

				gate(
					control_qubit_index_list_ptr,
					control_value_list_ptr,
					control_qubit_count,
					target_index,
					matrix_ptr,
					state_ptr,
					dim,
				)
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
