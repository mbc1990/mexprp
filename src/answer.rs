use crate::num::Num;
use crate::opers::Calculation;
use std::fmt;
use serde::{Serialize, Deserialize};

/// An answer of an evaluatation. Can be either a single answer or multiple. This struct contains some
/// helper methods for performing operations on single or multiple answers. The `op` method takes another
/// `Num`, and a function with two `Num` arguments, itself and the other (as references). It performs
/// that function on all combinations and returns an answer with all of the results in one. The `unop`
/// function is similar but it performs an operation on only itself, without another value (*un*ary
/// *op*eration).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Answer<N: Num> {
	/// A single answer
	Single(N),
	/// Multiple answers. Will always be at least two (probably)
	Multiple(Vec<N>),
}

impl<N: Num> Answer<N> {
	/// Perform an operation on all the values of an answer with all the values of another answer
	pub fn op<F: Fn(&N, &N) -> Calculation<N>>(&self, other: &Self, oper: F) -> Calculation<N> {
		fn push_answers<N: Num>(answer: Answer<N>, list: &mut Vec<N>) {
			match answer {
				Answer::Single(n) => list.push(n),
				Answer::Multiple(ns) => for n in ns {
					list.push(n)
				},
			}
		}

		match *self {
			Answer::Single(ref n) => match *other {
				Answer::Single(ref n2) => oper(n, n2),
				Answer::Multiple(ref n2s) => {
					let mut answers = Vec::new();
					for n2 in n2s {
						push_answers(oper(n, n2)?, &mut answers);
					}
					Ok(Answer::Multiple(answers))
				}
			},
			Answer::Multiple(ref ns) => match *other {
				Answer::Single(ref n2) => {
					let mut answers = Vec::new();
					for n in ns {
						push_answers(oper(n, n2)?, &mut answers);
					}
					Ok(Answer::Multiple(answers))
				}
				Answer::Multiple(ref n2s) => {
					let mut answers = Vec::new();
					for n in ns {
						for n2 in n2s {
							push_answers(oper(n, n2)?, &mut answers);
						}
					}
					Ok(Answer::Multiple(answers))
				}
			},
		}
	}

	/// Perform an operation on all the values of an answer
	pub fn unop<F: Fn(&N) -> Calculation<N>>(&self, oper: F) -> Calculation<N> {
		fn push_answers<N: Num>(answer: Answer<N>, list: &mut Vec<N>) {
			match answer {
				Answer::Single(n) => list.push(n),
				Answer::Multiple(ns) => for n in ns {
					list.push(n)
				},
			}
		}

		match *self {
			Answer::Single(ref n) => oper(n),
			Answer::Multiple(ref ns) => {
				let mut answers = Vec::new();
				for n in ns {
					push_answers(oper(n)?, &mut answers);
				}
				Ok(Answer::Multiple(answers))
			}
		}
	}

	/// Unwrap the single variant of an answer
	pub fn unwrap_single(self) -> N {
		match self {
			Answer::Single(n) => n,
			Answer::Multiple(_) => panic!("Attempted to unwrap multiple answers as one"),
		}
	}

	/// Convert this answer into a vector
	pub fn to_vec(self) -> Vec<N> {
		match self {
			Answer::Single(n) => vec![n],
			Answer::Multiple(ns) => ns,
		}
	}

	/// Adds all the answers of another answer to the asnwers of this answer, returning a new answer
	pub fn join(self, other: Self) -> Self {
		let mut new = Vec::new();
		match self {
			Answer::Single(n) => {
				new.push(n);
			}
			Answer::Multiple(mut ns) => {
				new.append(&mut ns);
			}
		}

		match other {
			Answer::Single(n) => {
				new.push(n);
			}
			Answer::Multiple(mut ns) => {
				new.append(&mut ns);
			}
		}
		Answer::Multiple(new)
	}
}

impl<N: Num> fmt::Display for Answer<N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Answer::Single(ref n) => write!(f, "{}", n),
			Answer::Multiple(ref ns) => {
				let mut buf = String::from("{");
				for (i, n) in ns.iter().enumerate() {
					buf.push_str(&format!("{}", n));
					if i + 1 < ns.len() {
						buf.push_str(", ");
					}
				}
				buf.push_str("}");
				write!(f, "{}", &buf)
			}
		}
	}
}
