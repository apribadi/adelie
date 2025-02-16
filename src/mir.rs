// (function $fib (($n i64)) ((i64))
//   (loop $continue-loop
//     (($n $n)
//      ($x #1)
//      ($y #0))
//     (if (i64.is_eq $n #0)
//       $y
//       (do
//         (let ($a) (i64.add $x $y))
//         (let ($b) (i64.sub $n #1))
//         (goto $continue-loop ($y $a $b))))))

use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Symbol<'a>(pub &'a [u8]);

#[derive(Clone, Copy)]
pub enum Type {
  I64,
}

/*
#[derive(Clone, Copy)]
pub enum ScalarType {
  Bool,
  I5,
  I6,
  I32,
  I64,
}

pub enum Type<'a> {
  Scalar(ScalarType),
  Tuple(&'a [Type<'a>]),
}

pub enum EffectType {
}
*/

pub struct Function<'a> {
  pub name: Symbol<'a>,
  pub params: &'a [(Symbol<'a>, Type)],
  // pub rets: &'a [&'a [Type]],
  pub body: Expression<'a>,
}

// let x = ...
// let x, y, z = ...
// goto ... ..., ..., ...
// return ..., ..., ...

#[derive(Clone, Copy)]
pub enum Statement<'a> {
  Let(Symbol<'a>, Expression<'a>),
  Set(Symbol<'a>, Expression<'a>),
  Var(Symbol<'a>, Expression<'a>),
  Goto(Symbol<'a>, &'a [Expression<'a>]),
  Return(),
}

// An expr can *potentially* return a single value to a single continuation.

#[derive(Clone, Copy)]
pub enum Expression<'a> {
  Call(&'a Call<'a>),
  Do(&'a [Statement<'a>]),
  If(&'a If<'a>),
  Variable(Symbol<'a>),
  ConstBool(bool),
  ConstI64(u64),
}

const _: () = assert!(size_of::<Expression<'static>>() <= 24);

#[derive(Clone, Copy)]
pub struct Call<'a> {
  pub function: Symbol<'a>,
  pub args: &'a [Expression<'a>],
}

#[derive(Clone, Copy)]
pub struct If<'a> {
  pub condition: Expression<'a>,
  pub if_true: Expression<'a>,
  pub if_false: Expression<'a>,
}

#[derive(Clone, Copy)]
pub struct Loop<'a> {
  pub name: Symbol<'a>,
  pub bindings: &'a [(Symbol<'a>, Expression<'a>)],
  pub body: Expression<'a>,
}

pub static FIB: Function<'static> = Function {
  name: Symbol(b"fib"),
  params: &[(Symbol(b"n"), Type::I64)],
  //body: Expression::ConstI64(13),
  body:
    Expression::Call(&Call {
      function: Symbol(b"add.i64"),
      args: &[
        Expression::If(&If {
          condition: Expression::ConstBool(false),
          if_true: Expression::ConstI64(1),
          if_false: Expression::ConstI64(2)
        }),
        Expression::If(&If {
          condition: Expression::ConstBool(true),
          if_true: Expression::ConstI64(3),
          if_false: Expression::ConstI64(4)
        })
      ]
    })
};
