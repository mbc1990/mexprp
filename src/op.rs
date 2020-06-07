use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Op {
	In(In),
	Pre(Pre),
	Post(Post),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum In {
	Pow,
	Mul,
	Div,
	Add,
	Sub,
	PlusMinus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Pre {
	Neg,
	Pos,
	PosNeg,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Post {
	Fact,
	Percent,
}

impl Op {
	pub fn precedence(&self) -> i32 {
		use self::In::*;
		use self::Pre::*;
		use self::Post::*;
		match *self {
			Op::In(ref op) => match *op {
				Pow => 4,
				Mul | Div => 3,
				Add | Sub | PlusMinus => 2,
			},
			Op::Pre(ref op) => match *op {
				Neg | Pos | PosNeg => 4,
			},
			Op::Post(ref op) => match *op {
				Fact => 4,
				Percent => 4,
			},
		}
	}

	pub fn is_left_associative(&self) -> bool {
		use self::In::*;
		use self::Pre::*;
		use self::Post::*;
		match *self {
			Op::In(ref op) => match *op {
				Pow => false,
				Mul | Div | Add | Sub | PlusMinus => true,
			},
			Op::Pre(ref op) => match *op {
				Neg | Pos | PosNeg => false,
			},
			Op::Post(ref op) => match *op {
				Fact => true,
				Percent => true,
			},
		}
	}

	pub fn to_string(&self) -> String {
		use self::In::*;
		use self::Pre::*;
		use self::Post::*;
		String::from(match *self {
			Op::In(ref op) => match *op {
				Pow => "^",
				Mul => "*",
				Div => "/",
				Add => "+",
				Sub => "-",
				PlusMinus => "±",
			},
			Op::Pre(ref op) => match *op {
				Neg => "-",
				Pos => "+",
				PosNeg => "±",
			},
			Op::Post(ref op) => match *op {
				Fact => "!",
				Percent => "%",
			},
		})
	}

	/// True if the operator should be evaluated before this one
	pub fn should_shunt(&self, other: &Op) -> bool {
		if (other.precedence() > self.precedence()) || (other.precedence() == self.precedence() && other.is_left_associative()) {
			true
		} else {
			false
		}
	}
}

use std::fmt;
impl fmt::Display for Op {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Paren {
	Open,
	Close,
}

impl Paren {
	pub fn to_str(&self) -> &str {
		match *self {
			Paren::Open => "(",
			Paren::Close => ")",
		}
	}
}

impl fmt::Display for Paren {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(self.to_str())
	}
}
