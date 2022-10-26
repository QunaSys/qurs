use super::binding::qulacs::{self, CTYPE};
use num::{Complex, One, Zero};
use ordered_float::OrderedFloat;
use std::fmt;
use superslice::Ext;

pub trait StateRef<F> {
	///Get qubit count
	fn qubit_count(&self) -> usize;

	///Get entropy
	fn get_entropy(&self) -> F;

	///Get squared norm
	fn get_squared_norm(&self) -> F;

	///Sampling measurement results
	fn sampling(&self, sampling_count: u32) -> Vec<u64>;

	/// Get probability with which we obtain 0 when we measure a qubit
	fn get_zero_probability(&self, qbit: usize) -> Result<F, StateErr>;

	/// Get merginal probability for measured values
	fn get_marginal_probability(
		&self,
		sorted_target_qubit_index_list: &[u32],
		measured_value_list: &[u32],
	) -> Result<F, StateErr>;
}

pub trait StateMut<F>: StateRef<F> {
	///Set state to |0>
	fn set_zero_state(&mut self);

	///Set state to computational basis
	fn set_computational_basis(&mut self, comp_basis: usize);

	///Set Haar random state
	fn set_haar_random_state(&mut self);

	///Set Haar random state with seed
	fn set_haar_random_state_with_seed(&mut self, seed: u32);

	///Normalize quantum state
	fn normalize(&mut self, norm_square: F);

	///Add state vector to this state
	fn add_state(&mut self, state: &[Complex<F>]);

	///Multiply coefficient to this state
	fn multiply_coef(&mut self, coef: Complex<F>);
}

pub trait PureStateRef<F>: StateRef<F> {}

pub trait PureStateMut<F>: PureStateRef<F> + StateMut<F> {}

/// Internal trait to implement `PureStateRef` and `PureStateImpl` for
/// slice-like types
///
/// # Safety
/// the slice may have more than 2^(qubit_count()) elements
unsafe trait PureStateImpl<F>: AsRef<[Complex<F>]> {
	fn qubit_count(&self) -> usize;
}

impl<T: PureStateImpl<f64>> StateRef<f64> for T {
	fn qubit_count(&self) -> usize {
		self.qubit_count()
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

	fn get_zero_probability(&self, qbit: usize) -> Result<f64, StateErr> {
		if qbit >= self.qubit_count() {
			return Err(StateErr::InvalidTargetQubitIndex(qbit));
		}
		unsafe {
			Ok(qulacs::M0_prob(
				qbit as u32,
				self.as_ref().as_ptr() as *const CTYPE,
				self.as_ref().len() as u64,
			))
		}
	}

	fn get_marginal_probability(
		&self,
		sorted_target_qubit_index_list: &[u32],
		measured_value_list: &[u32],
	) -> Result<f64, StateErr> {
		if sorted_target_qubit_index_list.len() != measured_value_list.len() {
			return Err(StateErr::InvalidTargetList(
				sorted_target_qubit_index_list.to_vec(),
			));
		}
		unsafe {
			Ok(qulacs::marginal_prob(
				sorted_target_qubit_index_list.as_ptr() as *const u32,
				measured_value_list.as_ptr() as *const u32,
				measured_value_list.len() as u32,
				self.as_ref().as_ptr() as *const CTYPE,
				self.as_ref().len() as u64,
			))
		}
	}

	fn sampling(&self, sampling_count: u32) -> Vec<u64> {
		let mut stacked_prob: Vec<OrderedFloat<f64>> = vec![];
		let mut result: Vec<u64> = vec![];
		let mut sum = 0.;
		stacked_prob.push(OrderedFloat(0.));
		for i in 0..self.as_ref().len() {
			sum += Complex::norm_sqr(&self.as_ref()[i]);
			stacked_prob.push(OrderedFloat(sum));
		}
		for _ in 0..sampling_count {
			let r = rand::random::<f64>();
			let index = stacked_prob.lower_bound(&OrderedFloat(r));
			result.push(index as u64 - 1);
		}
		result
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
			fn qubit_count(&self) -> usize {
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
		impl_array_state!(@impl, 0usize, $($n),+);
	};
}

impl_array_state!(1, 2, 4, 8, 16);

#[derive(Debug)]
pub struct StateVec<F = f64>(usize, Vec<Complex<F>>);

impl<F: num::Num + Clone> StateVec<F> {
	pub fn new(n: usize) -> Self {
		let mut v = vec![<Complex<F>>::zero(); 2usize.pow(n as u32)];
		v[0] = <Complex<F>>::one();
		Self(n, v)
	}

	/// Append an qubit initialized to |0> at end of this `StateVec`.
	///
	/// Returns the index of added qubit.
	///
	/// ```
	/// # use qurs::prelude::*;
	/// # use qurs::StateVec;
	/// let mut v = StateVec::new(0);
	/// assert_eq!(v.qubit_count(), 0);
	/// assert_eq!(v.push(), 0);
	/// assert_eq!(v.qubit_count(), 1);
	/// ```
	pub fn push(&mut self) -> usize {
		self.1
			.extend(core::iter::repeat(<Complex<F>>::zero()).take(self.1.len()));
		self.0 += 1;
		self.0 - 1
	}
}

unsafe impl PureStateImpl<f64> for StateVec<f64> {
	fn qubit_count(&self) -> usize {
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

pub trait GeneralStateRef {}

pub trait GeneralStateMut {}

#[non_exhaustive]
pub enum StateErr {
	InconsistentStateLength(usize, usize),
	InvalidTargetQubitIndex(usize),
	InvalidTargetList(Vec<u32>),
}

impl fmt::Display for StateErr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			StateErr::InconsistentStateLength(l, r) => {
				write!(f, "Inconsistent state length: left state has length of {l}, but right state has {r}")
			}
			StateErr::InvalidTargetQubitIndex(i) => {
				write!(f, "Invalid target qubit index: target qubit index must be smaller than qubit count, but {i} was supplied")
			}
			StateErr::InvalidTargetList(list) => {
				write!(f, "Invalid target list: {:?}", list)
			}
		}
	}
}

impl fmt::Debug for StateErr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		<Self as fmt::Display>::fmt(self, f)
	}
}

#[cfg(test)]
pub mod state_tests {
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
	fn test_impl() {
		assert_eq!([seed_comp(); 1].qubit_count(), 0);
		assert_eq!([seed_comp(); 2].qubit_count(), 1);
		assert_eq!([seed_comp(); 16].qubit_count(), 4);
		assert_eq!([seed_comp(); 128].qubit_count(), 7);

		let mut state = [Complex::one(), Complex::zero()];
		state.add_state(&state.clone());
		assert_eq!(state, [Complex::new(2., 0.), Complex::zero()]);

		state.normalize(state.get_squared_norm());
		assert_eq!(state, [Complex::one(), Complex::zero()]);

		let mut state = [Complex::one(), Complex::zero()];
		let random = seed_comp();
		state.multiply_coef(random);
		assert_eq!(state, [random, Complex::zero()]);

		state.set_computational_basis(1);
		assert_eq!(state, [Complex::zero(), Complex::one()]);

		assert_eq!(state.sampling(5), [1; 5]);

		state.set_haar_random_state();
		assert_near!(state.get_squared_norm(), 1., EPS);

		state.set_haar_random_state_with_seed(1);
		assert_near!(state.get_squared_norm(), 1., EPS);

		state.set_zero_state();
		assert_eq!(state, [Complex::one(), Complex::zero()]);

		assert_near!(state.get_entropy(), 0., EPS);

		let state = [seed_comp(), seed_comp(), seed_comp(), seed_comp()];
		let probs: Vec<f64> = state.iter().map(|e| e.norm_sqr()).collect();
		assert_near!(
			state.get_zero_probability(0).unwrap(),
			probs[0] + probs[2],
			EPS
		);

		let tests = [
			(vec![0, 1], vec![0, 0], probs[0]),
			(vec![0, 1], vec![1, 0], probs[1]),
			(vec![0, 1], vec![0, 1], probs[2]),
			(vec![0, 1], vec![1, 1], probs[3]),
			(vec![0], vec![0], probs[0] + probs[2]),
			(vec![0], vec![1], probs[1] + probs[3]),
			(vec![1], vec![0], probs[0] + probs[1]),
			(vec![1], vec![1], probs[2] + probs[3]),
			(vec![], vec![], probs[0] + probs[1] + probs[2] + probs[3]),
		];

		for (i, j, prob) in tests {
			assert_near!(state.get_marginal_probability(&i, &j).unwrap(), prob, EPS);
		}
	}

	fn seed_comp() -> Complex64 {
		Complex::new(rand::random(), rand::random())
	}
}
