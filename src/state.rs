use num;
use num::{Complex, One, Zero};

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
// # SAFETY
// the slice has more than 2^(len()) elements
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
		unimplemented!()
	}

	// panic-ing function
	fn get_zero_probability(&self, _qbit: usize) -> f64 {
		unimplemented!()
	}

	fn get_marginal_probability(&self, _qbits: &[usize]) -> f64 {
		unimplemented!()
	}
}

impl<T> StateMut<f64> for T
where
	T: PureStateImpl<f64> + AsMut<[Complex<f64>]>,
{
	fn set_zero_state(&mut self) {
		unimplemented!()
	}
	fn normalize(&mut self, norm_square: f64) {
		unimplemented!()
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

impl_array_state!(2, 4, 8, 16, 32);

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

struct GpuPureState();

pub trait GeneralStateRef {}

pub trait GeneralStateMut {}
