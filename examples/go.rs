//! ???
//!
//!

use adelie::ssa::Op1;
use adelie::ssa::Op2;
use adelie::ssa::Value;
use adelie::ssa::Instruction;

fn main() {
  let mut buf = adelie::ssa::SsaBuf::new();

  buf.emit_op1(Op1::I64Neg, Value(10));
  buf.emit_op1(Op1::I64Clz, Value(11));
  buf.emit_op2(Op2::I64Add, Value(12), Value(13));
  buf.emit_op2(Op2::I64Sub, Value(14), Value(15));

  adelie::ssa::display(buf.view());
}
