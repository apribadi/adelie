use crate::prelude::*;

pub enum Instruction<'a> {
  // BB entry

  Func(u32, TypeList<'a>),
  Case(),
  Join(TypeList<'a>),
  Kont(TypeList<'a>),

  // BB internal

  Op1(Op1, Value),
  Op2(Op2, Value, Value),
  Op3(Op3, Value, Value, Value),
  Select(Value, Value, Value),
  ImmI32(u32),
  ImmI64(u64),

  // BB terminator

  Cond(Value, Label, Label),
  Ret(RetPt, ValueList<'a>),
}

const TAG_OP1: u8 = 0x04;
const TAG_OP2: u8 = 0x05;

pub struct TypeList<'a>(&'a [u8]);

pub struct ValueList<'a>(&'a [u8]);

pub struct Value(pub u32);

pub struct Label(pub u32);

pub struct RetPt(pub u8);

pub type Type = crate::ssa_type::Type;

pub type Op1 = crate::ssa_op1::Op1;

pub type Op2 = crate::ssa_op2::Op2;

pub type Op3 = crate::ssa_op3::Op3;

impl<'a> TypeList<'a> {
  #[inline(always)]
  pub fn iter(&self) -> impl Iterator<Item = Type> + use<'_> {
    self.0.iter_chunks().map(|&[x]| Type::decode(x))
  }
}

impl<'a> ValueList<'a> {
  #[inline(always)]
  pub fn iter(&self) -> impl Iterator<Item = Value> + use<'_> {
    self.0.iter_chunks().map(|x| Value(u32::from_le_bytes(*x)))
  }
}

fn check_reserve<'a, 'b>(buf: &'a mut &'b [u8], size: usize) -> Option<&'b [u8]> {
  if ! (size <= buf.len()) {
    return None;
  }

  return Some(buf.pop_slice(size));
}

pub fn read<'a, 'b>(buf: &'a mut &'b [u8]) -> Option<Instruction<'b>> {
  let mut r = *buf;
  let instr;

  match check_reserve(&mut r, 1)?.pop_u8() {
    TAG_OP1 => {
      let mut r = check_reserve(&mut r, 5)?;
      let t = Op1::decode(r.pop_u8());
      let x = Value(r.pop_u32());
      instr = Instruction::Op1(t, x);
    }
    TAG_OP2 => {
      let mut r = check_reserve(&mut r, 9)?;
      let t = Op2::decode(r.pop_u8());
      let x = Value(r.pop_u32());
      let y = Value(r.pop_u32());
      instr = Instruction::Op2(t, x, y);
    }
    _ => {
      return None;
    }
  }

  *buf = r;
  return Some(instr);
}

pub struct SsaBuf(Buf);

impl SsaBuf {
  pub fn new() -> Self {
    SsaBuf(Buf::new())
  }

  pub fn view(&self) -> &[u8] {
    &* self.0
  }

  pub fn emit_op1(&mut self, t: Op1, Value(x): Value) {
    let mut w = self.0.reserve(6);
    w.put_u8(TAG_OP1);
    w.put_u8(Op1::encode(t));
    w.put_u32(x);
  }

  pub fn emit_op2(&mut self, t: Op2, Value(x): Value, Value(y): Value) {
    let mut w = self.0.reserve(10);
    w.put_u8(TAG_OP2);
    w.put_u8(Op2::encode(t));
    w.put_u32(x);
    w.put_u32(y);
  }
}

pub fn display(buf: &[u8]) {
  let mut r = buf;
  // let mut label = 0;
  let mut var = 0;

  loop {
    match read(&mut r) {
      None => { break; }
      Some(inst) => {
        match inst {
          Instruction::Op1(t, Value(x)) => {
            print!("  %{} = {:?} %{}\n", var, t, x);
            var = var + 1;
          }
          Instruction::Op2(t, Value(x), Value(y)) => {
            print!("  %{} = {:?} %{} %{}\n", var, t, x, y);
            var = var + 1;
          }
          _ => {
            print!("UNKNOWN INSTRUCTION\n")
          }

        }
      }
    }
  }
}
