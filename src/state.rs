use num;
use num::{Complex, One, Zero};
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
	fn normalize(&mut self, norm_square: F);
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
		self.as_ref()
			.iter()
			.map(|val| {
				let p = val.norm_sqr().max(1e-15);
				-1.0 * p * p.ln()
			})
			.sum()
	}

	fn get_squared_norm(&self) -> f64 {
		self.as_ref().iter().map(|val| val.norm_sqr()).sum()
	}

	// panic-ing function
	fn get_zero_probability(&self, qbit: usize) -> f64 {
		if self.len() < qbit {
			panic!("Error: get_zero_probability: index of target qubit must be smaller than qubit counts of the state");
		}
		let mask = 1 << qbit;
		let dim = self.as_ref().len();
		(0..dim / 2)
			.map(|state_index| {
				let basis_0 = insert_zero_to_basis_index(state_index, mask, qbit);
				self.as_ref()[basis_0].norm_sqr()
			})
			.sum()
	}

	// panic-ing function
	fn get_marginal_probability(&self, qbits: &[usize]) -> f64 {
		if self.len() != qbits.len() {
			panic!("Error: get_marginal_probability(&[usize]): the length of qbits must be equal to len()");
		}
		let target: Vec<(usize, &usize)> = qbits
			.iter()
			.enumerate()
			.filter(|e| *e.1 == 0 || *e.1 == 1)
			.collect();
		let dim = self.as_ref().len();
		let loop_dim = dim >> target.len();
		(0..loop_dim)
			.map(|mut basis| {
				for (insert_index, insert_val) in &target {
					let mask = 1 << insert_index;
					basis = insert_zero_to_basis_index(basis, mask, *insert_index);
					basis ^= mask * *insert_val;
				}
				self.as_ref()[basis].norm_sqr()
			})
			.sum()
	}
}

impl<T> StateMut<f64> for T
where
	T: PureStateImpl<f64> + AsMut<[Complex<f64>]>,
{
	fn set_zero_state(&mut self) {
		self.as_mut()[0] = Complex::one();
		for i in 1..self.as_ref().len() {
			self.as_mut()[i] = Complex::zero();
		}
	}
	fn normalize(&mut self, norm_square: f64) {
		for e in self.as_mut().iter_mut() {
			*e *= (1. / norm_square).sqrt();
		}
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

fn insert_zero_to_basis_index(basis_index: usize, basis_mask: usize, qubit_index: usize) -> usize {
	((basis_index >> qubit_index) << (qubit_index + 1)) + basis_index % basis_mask
}

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
