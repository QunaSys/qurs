include!(concat!(env!("OUT_DIR"), "/operator.rs"));
pub use num::complex::Complex;

// Deconstruct the complex vector and pass pointer to C++ code and back
pub fn operate(state: Vec<Complex<f64>>) {
	let dim = state.len();
	let mut vec = vec![];
	for elem in state {
		vec.push(elem.re);
		vec.push(elem.im);
	}
	let mut operator = unsafe { Operator::new() };
	let ptr = unsafe { operator.X(vec.as_mut_ptr()) };
	let slice = unsafe { std::slice::from_raw_parts(ptr, dim * 2 as usize) };
	dbg!(slice);
}

// Create a complex vector and pass it to operate
#[test]
fn test() {
	let state = vec![Complex::new(10.1, 20.1), Complex::new(10.1, 20.1)];
	operate(state);
}
