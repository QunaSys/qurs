use num;

pub trait StateRef<F: num::Float> {
	// TODO: Place analyzing operations performed on both PureStateRef and
	// DenseStateRef

	fn len(&self) -> usize;

	fn get_entropy() -> F;
	fn get_squared_norm() -> F;

	// panic-ing function
	fn get_zero_probability(&self, qbit: usize) -> F;
	fn get_marginal_probability(&self, qbits: &[usize]) -> F;
}

pub trait StateMut<F: num::Float>: StateRef<F> {
	// TODO: Place updating operations performed on both PureStateRef and
	// DenseStateRef

	fn set_zero_state(&mut self);
	fn normalize(&mut self, norm_square: F);
}

pub trait PureStateRef {}

impl<T: PureStateRef> StateRef for T {}

pub trait PureStateMut: PureStateRef {}

impl<T: PureStateMut> StateMut for T {}

// TODO: N=2以外(4,8,16,32,...)の場合も実装。マクロを再帰的に使う
impl<T: Borrow<[Complex<f64>; 2]>> PureStateRef for T {}

impl<T: BorrowMut<[Complex<f64>; 2]>> PureStateMut for T {}

struct GpuPureState();

pub trait DenseStateRef {}

pub trait DenseStateMut {}
