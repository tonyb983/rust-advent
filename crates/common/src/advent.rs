pub trait AdventPuzzleAssoc {
	type Input;
	type Output;

	fn default() -> Self;
	fn create(input: &Input) -> Self;

	fn run(&mut self) -> Result<Output>;
};

pub trait AdventPuzzleGen<TInput=(), TOutput=()> {
	fn default() -> Self;
	fn create(input: &Input) -> Self;

	fn run(&mut self) -> Result<TOutput>;
};
