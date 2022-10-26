use num::{Complex, Zero};
use qurs::gate::{ccnot_gate, h_gate};
use qurs::prelude::*;
use qurs::{self};

#[test]
fn test_ccnot_gate() {
	const N: usize = 3;
	let mut state = [Complex::zero(); 2usize.pow(N as u32)];
	state.set_computational_basis(0b010);

	h_gate(0, &mut state);
	h_gate(1, &mut state);

	ccnot_gate(0, 1, 2, &mut state);

	let delta = 0.00001;
	let expected = [0.5, 0.5, 0.75];

	(0..N).for_each(|i| {
		assert!(
			state.get_zero_probability(i).is_ok(),
			"{}th zero_probability is null!",
			i
		);
		let actual = state.get_zero_probability(i).unwrap();
		assert!(
			actual <= expected[i] + delta && actual >= expected[i] - delta,
			"actual = {}, expected = {} ± {}",
			actual,
			expected[i],
			delta
		);
	});
}
