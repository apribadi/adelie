use crate::prelude::*;
use crate::ssa;
use crate::mir;
use crate::mir::Expression;
use crate::mir::Symbol;
use crate::ssa::SsaBuf;
use crate::ssa::Label;
use crate::ssa::Value;

pub struct Env {
  out: SsaBuf,
}

impl Env {
  pub fn new() -> Self {
    Self {
      out: SsaBuf::new(),
    }
  }
}

pub fn compile(fun: &mir::Function<'_>) {
  let mut env = Env::new();

  env.out.emit_function(1, fun.params.len() as u32);

  for &(_, t) in fun.params.iter() {
    match t {
      mir::Type::I64 => {
        let _ = env.out.emit_param(ssa::Type::I64);
      }
    }
  }

  match compile_expression(&mut env, fun.body) {
    None => {}
    Some((value, _)) => {
      env.out.emit_return(0, 1);
      env.out.emit_value(value);
    }
  }

  ssa::display(env.out.view());
}


// For now, an expression either evaluates to a single typed ssa value, or
// doesn't return to its continuation at all.
//
// Later we will want to extend this to
// - aggregate types
// - zero or multiple return values
// - two or more continuations

pub fn compile_expression(env: &mut Env, exp: Expression<'_>) -> Option<(ssa::Value, ssa::Type)> {
  match exp {
    Expression::ConstBool(p) => {
      Some((env.out.emit_const_bool(p), ssa::Type::BOOL))
    }
    Expression::ConstI64(n) => {
      Some((env.out.emit_const_i64(n), ssa::Type::I64))
    }
    Expression::Call(&mir::Call { function: Symbol(b"add.i64"), args: &[x, y] }) => {
      let (x, t) = compile_expression(env, x)?;
      assert!(t == ssa::Type::I64);
      let (y, t) = compile_expression(env, y)?;
      assert!(t == ssa::Type::I64);
      Some((env.out.emit_op2(ssa::Op2::ADD_I64, x, y), ssa::Type::I64))
    }
    Expression::If(&mir::If { condition, if_true, if_false }) => {
      let (p, t) = compile_expression(env, condition)?;
      assert!(t == ssa::Type::BOOL);
      let (a, b) = env.out.emit_if(p, Label(0), Label(0));

      let case0 = 'arm: {
        let label = env.out.emit_case();
        env.out.patch_label(b, label);
        let Some((x, t)) = compile_expression(env, if_false) else { break 'arm None; };
        let point = env.out.emit_goto(Label(0), 1);
        env.out.emit_value(x);
        Some((t, point))
      };

      let case1 = 'arm: {
        let label = env.out.emit_case();
        env.out.patch_label(a, label);
        let Some((x, t)) = compile_expression(env, if_true) else { break 'arm None; };
        let point = env.out.emit_goto(Label(0), 1);
        env.out.emit_value(x);
        Some((t, point))
      };

      match [case0, case1] {
        [None, None] => {
          None
        }
        [Some((t, point)), None] | [None, Some((t, point))] => {
          let label = env.out.emit_join(1);
          env.out.patch_label(point, label);
          Some((env.out.emit_param(t), t))
        }
        [Some((t0, point0)), Some((t1, point1))] => {
          assert!(t0 == t1);
          let label = env.out.emit_join(1);
          env.out.patch_label(point0, label);
          env.out.patch_label(point1, label);
          Some((env.out.emit_param(t0), t0))
        }
      }
    }
    _ => {
      panic!()
    }
  }
}
