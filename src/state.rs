use super::binding::qulacs::{self, CTYPE};
use num::{Complex, One, Zero};
use rand;
#[allow(clippy::len_without_is_empty)]
pub trait StateRef<F> {
	// TODO: Place analyzing operations performed on both PureStateRef and
	// DenseStateRef

	fn len(&self) -> usize;

	fn get_entropy(&self) -> F;
	fn get_squared_norm(&self) -> F;

	// panic-ing function
	fn get_zero_probability(&self, qbit: usize) -> F;
	fn get_marginal_probability(&self, qbits: &[usize]) -> F;
}

pub trait StateMut<F>: StateRef<F> {
	// TODO: Place updating operations performed on both PureStateRef and
	// DenseStateRef

	fn set_zero_state(&mut self);
	fn set_computational_basis(&mut self, comp_basis: usize);
	fn set_haar_random_state(&mut self);
	fn set_haar_random_state_with_seed(&mut self, seed: u32);
	fn normalize(&mut self, norm_square: F);
	fn add_state(&mut self, state: &[Complex<F>]);
	fn multiply_coef(&mut self, coef: Complex<F>);
}

pub trait PureStateRef<F>: StateRef<F> {}

pub trait PureStateMut<F>: PureStateRef<F> + StateMut<F> {}

// Internal trait to implement `PureStateRef` and `PureStateImpl` for slice-like
// types
//
/// # Safety
/// the slice has more than 2^(len()) elements
unsafe trait PureStateImpl<F>: AsRef<[Complex<F>]> {
	fn len(&self) -> usize;
}

impl<T> StateRef<f64> for T
where
	T: PureStateImpl<f64>,
{
	fn len(&self) -> usize {
		self.len()
	}

	fn get_entropy(&self) -> f64 {
		unsafe {
			qulacs::measurement_distribution_entropy(
				self.as_ref().as_ptr() as *const CTYPE,
				self.as_ref().len() as u64,
			)
		}
	}

	fn get_squared_norm(&self) -> f64 {
		unsafe {
			qulacs::state_norm_squared(
				self.as_ref().as_ptr() as *const CTYPE,
				self.as_ref().len() as u64,
			)
		}
	}

	// panic-ing function
	fn get_zero_probability(&self, qbit: usize) -> f64 {
		unsafe {
			qulacs::M0_prob(
				qbit as u32,
				self.as_ref().as_ptr() as *const CTYPE,
				self.as_ref().len() as u64,
			)
		}
	}

	// panic-ing function
	fn get_marginal_probability(&self, qbits: &[usize]) -> f64 {
		let mut target_index = vec![];
		let mut target_value = vec![];

		for (i, val) in qbits.iter().enumerate() {
			if *val == 0 || *val == 1 {
				target_index.push(i as u32);
				target_value.push(*val as u32);
			}
		}

		unsafe {
			qulacs::marginal_prob(
				target_index.as_ptr() as *const u32,
				target_value.as_ptr() as *const u32,
				target_index.len() as u32,
				self.as_ref().as_ptr() as *const CTYPE,
				self.as_ref().len() as u64,
			)
		}
	}
}

impl<T> StateMut<f64> for T
where
	T: PureStateImpl<f64> + AsMut<[Complex<f64>]>,
{
	fn set_zero_state(&mut self) {
		unsafe {
			qulacs::initialize_quantum_state(
				self.as_mut().as_mut_ptr() as *mut CTYPE,
				self.as_ref().len() as u64,
			);
		}
	}
	fn set_computational_basis(&mut self, comp_basis: usize) {
		unsafe {
			qulacs::initialize_quantum_state(
				self.as_mut().as_mut_ptr() as *mut CTYPE,
				self.as_ref().len() as u64,
			);
		};
		self.as_mut()[0] = Complex::zero();
		self.as_mut()[comp_basis] = Complex::one();
	}

	fn set_haar_random_state(&mut self) {
		unsafe {
			qulacs::initialize_Haar_random_state_with_seed(
				self.as_mut().as_mut_ptr() as *mut CTYPE,
				self.as_ref().len() as u64,
				rand::random(),
			);
		};
	}

	fn set_haar_random_state_with_seed(&mut self, seed: u32) {
		unsafe {
			qulacs::initialize_Haar_random_state_with_seed(
				self.as_mut().as_mut_ptr() as *mut CTYPE,
				self.as_ref().len() as u64,
				seed,
			);
		};
	}

	fn normalize(&mut self, squared_norm: f64) {
		unsafe {
			qulacs::normalize(
				squared_norm,
				self.as_mut().as_mut_ptr() as *mut CTYPE,
				self.as_ref().len() as u64,
			)
		};
	}

	fn add_state(&mut self, state: &[Complex<f64>]) {
		unsafe {
			qulacs::state_add(
				state.as_ptr() as *const CTYPE,
				self.as_mut().as_mut_ptr() as *mut CTYPE,
				state.as_ref().len() as u64,
			);
		}
	}

	fn multiply_coef(&mut self, coef: Complex<f64>) {
		unsafe {
			qulacs::state_multiply(
				CTYPE {
					re: coef.re,
					im: coef.im,
				},
				self.as_mut().as_mut_ptr() as *mut CTYPE,
				self.as_ref().len() as u64,
			)
		};
	}
}

impl<T> PureStateRef<f64> for T where T: PureStateImpl<f64> {}

impl<T> PureStateMut<f64> for T where T: PureStateImpl<f64> + AsMut<[Complex<f64>]> {}

macro_rules! impl_array_state {
	(@impl, $n:expr) => {
		unsafe impl<F: num::Num> PureStateImpl<F> for [Complex<F>; 2usize.pow($n as u32)]
		{
			fn len(&self) -> usize {
				$n
			}
		}
	};
	(@impl, $sum:expr, $n0:expr) => {
		impl_array_state!(@impl, $sum);
		impl_array_state!(@impl, $sum + $n0);
	};
	(@impl, $sum:expr, $n0:expr, $($n:expr),+) => {
		impl_array_state!(@impl, $sum, $($n),+);
		impl_array_state!(@impl, $sum + $n0, $($n),+);
	};
	($($n:expr),+) => {
		impl_array_state!(@impl, 1usize, $($n),+);
	};
}

impl_array_state!(1, 2, 4, 8, 16);

pub struct StateVec<F = f64>(usize, Vec<Complex<F>>);

impl<F: num::Num + Clone> StateVec<F> {
	pub fn new(n: usize) -> Self {
		assert!(n > 0);
		let mut v = vec![<Complex<F>>::zero(); 2usize.pow(n as u32)];
		v[0] = <Complex<F>>::one();
		Self(n, v)
	}
}

unsafe impl PureStateImpl<f64> for StateVec<f64> {
	fn len(&self) -> usize {
		self.0
	}
}

impl<F: num::Num> AsRef<[Complex<F>]> for StateVec<F> {
	fn as_ref(&self) -> &[Complex<F>] {
		&self.1
	}
}

impl<F: num::Num> AsMut<[Complex<F>]> for StateVec<F> {
	fn as_mut(&mut self) -> &mut [Complex<F>] {
		&mut self.1
	}
}

// struct GpuPureState();

pub trait GeneralStateRef {}

pub trait GeneralStateMut {}

#[cfg(test)]
mod state_tests {
	use super::{StateMut, StateRef};
	use num::{complex::Complex64, Complex, One, Zero};
	const EPS: f64 = 1e-10;

	macro_rules! assert_near {
		($x:expr, $y:expr, $d:expr) => {
			if ($x - $y).abs() >= $d {
				panic!("assert_near fail; left: {} right:{}", $x, $y);
			}
		};
	}

	#[test]
	fn test_len() {
		assert_eq!([seed_comp(); 2].len(), 1);
		assert_eq!([seed_comp(); 16].len(), 4);
		assert_eq!([seed_comp(); 128].len(), 7);
	}

	#[test]
	fn test_normalize() {
		let mut state = [seed_comp(), seed_comp(), seed_comp(), seed_comp()];
		state.set_haar_random_state();
		state.set_haar_random_state_with_seed(1);
		state.add_state(&state.clone());
		state.multiply_coef(seed_comp());
		state.normalize(state.get_squared_norm());
		let norm: f64 = state.iter().map(|e| e.norm_sqr()).sum();
		assert_near!(norm, 1., EPS);
		state.set_zero_state();
		assert_eq!(
			state,
			[
				Complex::one(),
				Complex::zero(),
				Complex::zero(),
				Complex::zero()
			]
		);
		assert_near!(state.get_entropy(), 0., EPS);
	}
	#[test]
	fn test_get_probability() {
		let state = [seed_comp(), seed_comp(), seed_comp(), seed_comp()];
		let probs: Vec<f64> = state.iter().map(|e| e.norm_sqr()).collect();
		assert_near!(state.get_zero_probability(0), probs[0] + probs[2], EPS);
		let tests = [
			([0, 0], probs[0]),
			([1, 0], probs[1]),
			([0, 1], probs[2]),
			([1, 1], probs[3]),
			([0, 2], probs[0] + probs[2]),
			([1, 2], probs[1] + probs[3]),
			([2, 0], probs[0] + probs[1]),
			([2, 1], probs[2] + probs[3]),
			([2, 2], probs[0] + probs[1] + probs[2] + probs[3]),
		];

		for (i, prob) in tests {
			assert_near!(state.get_marginal_probability(&i), prob, EPS);
		}
	}

	fn seed_comp() -> Complex64 {
		Complex::new(rand::random(), rand::random())
	}
}
