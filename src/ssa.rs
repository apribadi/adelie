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

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Tag(pub u8);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Type(pub u8);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Op1(pub u8);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Op2(pub u8);

pub struct Value(pub u32);

pub struct Label(pub u32);

pub struct RetPt(pub u8);

pub struct TypeList<'a>(&'a [u8]);

pub struct ValueList<'a>(&'a [u8]);

pub type Op3 = crate::ssa_op3::Op3;

impl Tag {
  pub const OP1: Self = Self(0x04);
  pub const OP2: Self = Self(0x05);
}

impl Type {
  pub const BOOL: Self = Self(0x00);
  pub const I5: Self = Self(0x03);
  pub const I6: Self = Self(0x04);
  pub const I32: Self = Self(0x07);
  pub const I64: Self = Self(0x08);
  pub const VOID: Self = Self(0x0d);
}

impl Op1 {
  pub const I64_BIT_NOT: Self = Self(0x05);
  pub const I64_CLZ: Self = Self(0x06);
  pub const I64_CTZ: Self = Self(0x07);
  pub const I64_NEG: Self = Self(0x08);

  pub fn info(self)
    -> (
      &'static str,
    )
  {
    match self {
      Self::I64_CTZ => (
        "i64.ctz",
      ),
      Self::I64_NEG => (
        "i64.neg",
      ),
      _ => (
        "unknown",
      )
    }
  }
}

impl core::fmt::Display for Op1 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.info().0)
  }
}

impl Op2 {
  pub const I64_BIT_XOR: Self = Self(0x05);
  pub const I64_ADD: Self = Self(0x06);
  pub const I64_SUB: Self = Self(0x07);

  pub fn info(self)
    -> (
      &'static str,
    )
  {
    match self {
      Self::I64_BIT_XOR => (
        "i64.bit_xor",
      ),
      Self::I64_ADD => (
        "i64.add",
      ),
      Self::I64_SUB => (
        "i64.sub",
      ),
      _ => (
        "unknown",
      )
    }
  }
}

impl core::fmt::Display for Op2 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.info().0)
  }
}

impl<'a> TypeList<'a> {
  #[inline(always)]
  pub fn iter(&self) -> impl Iterator<Item = Type> + use<'_> {
    self.0.iter_chunks().map(|&[x]| Type(x))
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

  match Tag(check_reserve(&mut r, 1)?.pop_u8()) {
    Tag::OP1 => {
      let mut r = check_reserve(&mut r, 5)?;
      let t = Op1(r.pop_u8());
      let x = Value(r.pop_u32());
      instr = Instruction::Op1(t, x);
    }
    Tag::OP2 => {
      let mut r = check_reserve(&mut r, 9)?;
      let t = Op2(r.pop_u8());
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

  pub fn emit_op1(&mut self, Op1(t): Op1, Value(x): Value) {
    let mut w = self.0.reserve(6);
    w.put_u8(Tag::OP1.0);
    w.put_u8(t);
    w.put_u32(x);
  }

  pub fn emit_op2(&mut self, Op2(t): Op2, Value(x): Value, Value(y): Value) {
    let mut w = self.0.reserve(10);
    w.put_u8(Tag::OP2.0);
    w.put_u8(t);
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
            print!("  %{} = {} %{}\n", var, t, x);
            var = var + 1;
          }
          Instruction::Op2(t, Value(x), Value(y)) => {
            print!("  %{} = {} %{} %{}\n", var, t, x, y);
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
