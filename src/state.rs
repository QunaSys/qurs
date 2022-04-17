use num;
use num::Complex;
use std::borrow::{Borrow, BorrowMut};

pub trait StateRef<F> {
	// TODO: Place analyzing operations performed on both PureStateRef and
	// DenseStateRef

	fn len(&self) -> usize;

	fn get_entropy() -> F;
	fn get_squared_norm() -> F;

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
// # SAFETY
// the slice has more than 2^(len()) elements
unsafe trait PureStateImpl<F>: AsRef<[F]> {
	fn len(&self) -> usize;
}

impl StateRef<Complex<f64>> for T
where
	T: PureStateImpl<Complex<f64>>,
{
	fn len(&self) -> usize {
		self.len()
	}

	fn get_entropy() -> F {
		unimplemented!()
	}

	fn get_squared_norm() -> F {
		unimplemented!()
	}

	// panic-ing function
	fn get_zero_probability(&self, qbit: usize) -> F {
		unimplemented!()
	}

	fn get_marginal_probability(&self, qbits: &[usize]) -> F {
		unimplemented!()
	}
}

impl StateMut<F> for T
where
	T: PureStateImpl<Complex<f64>> + AsMut<[F]>,
{
	fn set_zero_state(&mut self) {
		unimplemented!()
	}
	fn normalize(&mut self, norm_square: F) {
		unimplemented!()
	}
}

impl PureStateRef<Complex<f64>> for T where T: PureStateImpl<Complex<f64>> {}

impl PureStateMut<Complex<f64>> for T where T: PureStateImpl<Complex<f64>> + AsMut<[Complex<f64>]> {}

macro_rules! impl_array_state {
	(@impl, $n:expr) => {
		unsafe impl<F: num::Num> SliceImpl<F> for [F; 2usize.pow($n as u32)]
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
		impl_array_state!(@impl, 0usize, $($n),+);
	};
}

impl_array_state!(1, 2, 4, 8, 16, 32);

struct StateVec<F = num::Complex<f64>>(usize, Vec<F>);

impl<F: num::Num> StateVec<F> {
	pub fn new(n: usize) -> Self {
		assert!(n > 0);
		let mut v = vec![F::zero(); 2usize.pow(n as u32)];
		v[0] = F::one();
		Self(n, v)
	}
}

unsafe impl PureStateImpl<Complex<f64>> for StateVec<Complex<f64>> {
	fn len(&self) -> usize {
		self.0
	}
}

impl<F: num::Num> AsRef<[F]> for StateVec<F> {
	fn as_ref(&self) -> &[F] {
		&self.1
	}
}

impl<F: num::Num> AsMut<[F]> for StateVec<F> {
	fn as_mut(&mut self) -> &mut [F] {
		&mut self.1
	}
}

struct GpuPureState();

pub trait GeneralStateRef {}

pub trait GeneralStateMut {}
