//! ???
//!
//!

use adelie::ssa::Op1;
use adelie::ssa::Op2;
use adelie::ssa::Value;
use adelie::ssa::Label;
use adelie::ssa::Type;
// use adelie::ssa::Instruction;

fn main() {
  let mut buf = adelie::ssa::SsaBuf::new();

  buf.emit_func(1, 1);
  buf.emit_type(Type::I64);
  buf.emit_imm_i64(1);
  buf.emit_imm_i64(0);
  buf.emit_jump(Label(1), 3);
  buf.emit_value(Value(0)); // n
  buf.emit_value(Value(1)); // #1
  buf.emit_value(Value(2)); // #0
  buf.emit_join(3);         // k, x, y
  buf.emit_type(Type::I64);
  buf.emit_type(Type::I64);
  buf.emit_type(Type::I64);
  buf.emit_op2(Op2::I64_IS_EQ, Value(3), Value(2));
  buf.emit_cond(Value(6), Label(2), Label(3));
  buf.emit_case();
  buf.emit_ret(0, 1);
  buf.emit_value(Value(5));
  buf.emit_case();
  buf.emit_op2(Op2::I64_ADD, Value(4), Value(5));
  buf.emit_op2(Op2::I64_SUB, Value(3), Value(1));
  buf.emit_jump(Label(1), 3);
  buf.emit_value(Value(8));
  buf.emit_value(Value(5));
  buf.emit_value(Value(7));

  adelie::ssa::display(buf.view());
}
