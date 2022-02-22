include!(concat!(env!("OUT_DIR"), "/qulacs.rs"));
pub use num::complex::Complex;

type SingleGate = unsafe extern "C" fn(target_qubit_index: u32, state: *mut CTYPE, dim: ITYPE);
type ControlledGate = unsafe extern "C" fn(
	conrol_qubit_index: u32,
	target_qubit_index: u32,
	state: *mut CTYPE,
	dim: u64,
);
pub enum Gate {
	SingleGate(u32, SingleGate),
	ControlledGate(u32, u32, ControlledGate),
}

///bindings between C++
pub fn bindings(state: &mut Vec<Complex<f64>>, gate: Gate) {
	let mut new_state = comp_to_ctype(state);
	unsafe {
		match gate {
			Gate::SingleGate(target_qubit_index, gate) => {
				gate(
					target_qubit_index,
					new_state.as_mut_ptr(),
					new_state.len() as u64,
				);
			}
			Gate::ControlledGate(control_qubit_index, target_qubit_index, gate) => {
				gate(
					control_qubit_index,
					target_qubit_index,
					new_state.as_mut_ptr(),
					new_state.len() as u64,
				);
			}
		}
	}
	*state = ctype_to_comp(new_state);
}

fn comp_to_ctype(state: &Vec<Complex<f64>>) -> Vec<CTYPE> {
	state
		.into_iter()
		.map(|e| CTYPE { re: e.re, im: e.im })
		.collect::<Vec<CTYPE>>()
}

fn ctype_to_comp(state: Vec<CTYPE>) -> Vec<Complex<f64>> {
	state
		.into_iter()
		.map(|e| Complex::new(e.re, e.im))
		.collect::<Vec<Complex<f64>>>()
}
