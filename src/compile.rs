use crate::prelude::*;
use crate::ssa;
use crate::mir;
use crate::mir::Expression;
use crate::mir::Symbol;
use crate::ssa::SsaBuf;
use crate::ssa::Label;
use crate::ssa::Value;

pub struct Env {
  value_id: u32,
  out: SsaBuf,
}

impl Env {
  pub fn new() -> Self {
    Self {
      value_id: 0,
      out: SsaBuf::new(),
    }
  }
}

pub fn compile(fun: &mir::Function<'_>) {
  let mut env = Env::new();

  env.out.emit_function(1, fun.parameters.len() as u32);

  for &(_, t) in fun.parameters.iter() {
    match t {
      mir::Type::I64 => {
        env.out.emit_parameter(ssa::Type::I64);
      }
    }
  }

  let value = compile_expression(&mut env, fun.body);
  env.out.emit_return(0, 1);
  env.out.emit_value(value);

  ssa::display(env.out.view());
}

pub fn compile_expression(env: &mut Env, exp: Expression<'_>) -> ssa::Value {
  match exp {
    Expression::ConstBool(p) => {
      env.out.emit_const_bool(p)
    }
    Expression::ConstI64(n) => {
      env.out.emit_const_i64(n)
    }
    Expression::If(&mir::If {
      condition,
      if_true,
      if_false,
    }) => {
      let p = compile_expression(env, condition);
      let (a, b) = env.out.emit_if(p, Label(0), Label(0));
      let l = env.out.emit_case();
      env.out.patch_label(b, l);
      let x = compile_expression(env, if_false);
      let c = env.out.emit_goto(Label(0), 1);
      env.out.emit_value(x);
      let l = env.out.emit_case();
      env.out.patch_label(a, l);
      let y = compile_expression(env, if_true);
      let d = env.out.emit_goto(Label(0), 1);
      env.out.emit_value(y);
      let l = env.out.emit_join(1);
      env.out.patch_label(c, l);
      env.out.patch_label(d, l);
      env.out.emit_parameter(ssa::Type::I64) // ????
    }
    Expression::Call(&mir::Call {
      function: Symbol(b"add.i64"),
      arguments: &[x, y],
    }) => {
      let x = compile_expression(env, x);
      let y = compile_expression(env, y);
      env.out.emit_op2(ssa::Op2::ADD_I64, x, y)
    }
    _ => {
      panic!()
    }
  }
}
