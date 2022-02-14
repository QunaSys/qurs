include!(concat!(env!("OUT_DIR"), "/qulacs.rs"));
pub use num::complex::Complex;

#[test]
pub fn operate() {
	// innput [1,0], CTYPE is from /qulacs.rs in out dir
	let mut state = vec![CTYPE { re: 1., im: 0. }, CTYPE { re: 0., im: 0. }];
	unsafe {
		X_gate(0, state.as_mut_ptr(), 2);
		Z_gate(0, state.as_mut_ptr(), 2);
		//return [0,-1]
		dbg!(state);
	}
}
