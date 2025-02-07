use crate::prelude::*;

pub enum Instruction<'a> {
  // BB entry

  Func(u32, TypeList<'a>),
  Case(),
  Join(TypeList<'a>),
  Kont(TypeList<'a>),

  // BB internal

  ImmI32(u32),
  ImmI64(u64),
  Op1(Op1, Value),
  Op2(Op2, Value, Value),
  Select(Value, Value, Value),

  // BB terminator

  Cond(Value, Label, Label),
  Ret(u32, ValueList<'a>),
  Jump(Label, ValueList<'a>),
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Tag(pub u8);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Type(pub u8);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Op1(pub u8);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Op2(pub u8);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Value(pub u32);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Label(pub u32);

pub struct TypeList<'a>(&'a [u8]);

pub struct ValueList<'a>(&'a [u8]);

impl Tag {
  pub const FUNC: Self = Self(0x01);
  pub const CASE: Self = Self(0x02);
  pub const JOIN: Self = Self(0x03);
  pub const KONT: Self = Self(0x04);

  pub const IMM_I32: Self = Self(0x08);
  pub const IMM_I64: Self = Self(0x09);
  pub const OP1: Self = Self(0x05);
  pub const OP2: Self = Self(0x06);
  pub const SELECT: Self = Self(0x07);

  pub const COND: Self = Self(0x0a);
  pub const JUMP: Self = Self(0x0b);
  pub const RET: Self = Self(0x0c);
  pub const CALL: Self = Self(0x0d);
  pub const TAILCALL: Self = Self(0x0e);
}

impl Type {
  pub const BOOL: Self = Self(0x00);
  pub const I5: Self = Self(0x03);
  pub const I6: Self = Self(0x04);
  pub const I32: Self = Self(0x07);
  pub const I64: Self = Self(0x08);

  pub fn info(self)
    -> (
      &'static str,
    )
  {
    match self {
      Self::I64 => (
        "i64",
      ),
      _ => (
        "unknown",
      )
    }
  }

  pub fn name(self) -> &'static str {
    self.info().0
  }
}

impl core::fmt::Display for Type {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.name())
  }
}

impl Op1 {
  pub const I64_BIT_NOT: Self = Self(0x05);
  pub const I64_CLZ: Self = Self(0x06);
  pub const I64_CTZ: Self = Self(0x07);
  pub const I64_NEG: Self = Self(0x08);

  pub fn name(self) -> &'static str {
    self.info().0
  }

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
    write!(f, "{}", self.name())
  }
}

impl Op2 {
  pub const I64_BIT_XOR: Self = Self(0x05);
  pub const I64_ADD: Self = Self(0x06);
  pub const I64_SUB: Self = Self(0x07);
  pub const I64_IS_EQ: Self = Self(0x08);

  pub fn info(self)
    -> &'static (
      &'static str,
    )
  {
    match self {
      Self::I64_BIT_XOR => &(
        "i64.bit_xor",
      ),
      Self::I64_ADD => &(
        "i64.add",
      ),
      Self::I64_SUB => &(
        "i64.sub",
      ),
      Self::I64_IS_EQ => &(
        "i64.is_eq",
      ),
      _ => &(
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

pub struct SsaBuf(Buf);

impl SsaBuf {
  pub fn new() -> Self {
    SsaBuf(Buf::new())
  }

  pub fn view(&self) -> &[u8] {
    self.0.view()
  }

  pub fn emit_type(&mut self, Type(t): Type) {
    let mut w = self.0.append(1);
    w.put_u8(t);
  }

  pub fn emit_value(&mut self, Value(x): Value) {
    let mut w = self.0.append(4);
    w.put_u32(x);
  }

  pub fn emit_func(&mut self, nrets: u32, nargs: u32) {
    let mut w = self.0.append(9);
    w.put_u8(Tag::FUNC.0);
    w.put_u32(nrets);
    w.put_u32(nargs);
  }

  pub fn emit_case(&mut self) {
    let mut w = self.0.append(1);
    w.put_u8(Tag::CASE.0);
  }

  pub fn emit_join(&mut self, nargs: u32) {
    let mut w = self.0.append(5);
    w.put_u8(Tag::JOIN.0);
    w.put_u32(nargs);
  }

  pub fn emit_imm_i64(&mut self, c: u64) {
    let mut w = self.0.append(9);
    w.put_u8(Tag::IMM_I64.0);
    w.put_u64(c);
  }

  pub fn emit_op1(&mut self, Op1(t): Op1, Value(x): Value) {
    let mut w = self.0.append(6);
    w.put_u8(Tag::OP1.0);
    w.put_u8(t);
    w.put_u32(x);
  }

  pub fn emit_op2(&mut self, Op2(t): Op2, Value(x): Value, Value(y): Value) {
    let mut w = self.0.append(10);
    w.put_u8(Tag::OP2.0);
    w.put_u8(t);
    w.put_u32(x);
    w.put_u32(y);
  }

  pub fn emit_select(&mut self, Value(p): Value, Value(x): Value, Value(y): Value) {
    let mut w = self.0.append(13);
    w.put_u8(Tag::SELECT.0);
    w.put_u32(p);
    w.put_u32(x);
    w.put_u32(y);
  }

  pub fn emit_cond(&mut self, Value(p): Value, Label(a): Label, Label(b): Label) {
    let mut w = self.0.append(13);
    w.put_u8(Tag::COND.0);
    w.put_u32(p);
    w.put_u32(a);
    w.put_u32(b);
    assert!(w.is_empty());
  }

  pub fn emit_jump(&mut self, Label(a): Label, nargs: u32) {
    let mut w = self.0.append(9);
    w.put_u8(Tag::JUMP.0);
    w.put_u32(a);
    w.put_u32(nargs);
  }

  pub fn emit_ret(&mut self, index: u32, nargs: u32) {
    let mut w = self.0.append(9);
    w.put_u8(Tag::RET.0);
    w.put_u32(index);
    w.put_u32(nargs);
  }
}

fn window<'a, 'b>(buf: &'a mut &'b [u8], size: usize) -> Option<&'b [u8]> {
  if size <= buf.len() {
    Some(buf.pop_slice(size))
  } else {
    None
  }
}

pub fn read<'a, 'b>(buf: &'a mut &'b [u8]) -> Option<Instruction<'b>> {
  let mut cursor = *buf;

  let instr =
    match Tag(window(&mut cursor, 1)?.pop_u8()) {
      Tag::FUNC => {
        let mut r = window(&mut cursor, 8)?;
        let nrets = r.pop_u32();
        let nargs = r.pop_u32();
        let mut r = window(&mut cursor, nargs as usize)?;
        Instruction::Func(nrets, TypeList(r.pop_all()))
      }
      Tag::CASE => {
        Instruction::Case()
      }
      Tag::JOIN => {
        let mut r = window(&mut cursor, 4)?;
        let nargs = r.pop_u32();
        let mut r = window(&mut cursor, nargs as usize)?;
        Instruction::Join(TypeList(r.pop_all()))
      }
      Tag::IMM_I64 => {
        let mut r = window(&mut cursor, 8)?;
        Instruction::ImmI64(r.pop_u64())
      }
      Tag::OP1 => {
        let mut r = window(&mut cursor, 5)?;
        let t = Op1(r.pop_u8());
        let x = Value(r.pop_u32());
        Instruction::Op1(t, x)
      }
      Tag::OP2 => {
        let mut r = window(&mut cursor, 9)?;
        let t = Op2(r.pop_u8());
        let x = Value(r.pop_u32());
        let y = Value(r.pop_u32());
        Instruction::Op2(t, x, y)
      }
      Tag::SELECT => {
        let mut r = window(&mut cursor, 12)?;
        let p = Value(r.pop_u32());
        let x = Value(r.pop_u32());
        let y = Value(r.pop_u32());
        Instruction::Select(p, x, y)
      }
      Tag::COND => {
        let mut r = window(&mut cursor, 12)?;
        let p = Value(r.pop_u32());
        let a = Label(r.pop_u32());
        let b = Label(r.pop_u32());
        Instruction::Cond(p, a, b)
      }
      Tag::JUMP => {
        let mut r = window(&mut cursor, 8)?;
        let a = r.pop_u32();
        let nargs = r.pop_u32();
        let mut r = window(&mut cursor, nargs as usize * 4)?;
        Instruction::Jump(Label(a), ValueList(r.pop_all()))
      }
      Tag::RET => {
        let mut r = window(&mut cursor, 8)?;
        let index = r.pop_u32();
        let nargs = r.pop_u32();
        let mut r = window(&mut cursor, nargs as usize * 4)?;
        Instruction::Ret(index, ValueList(r.pop_all()))
      }
      _ => {
        return None;
      }
    };

  *buf = cursor;
  return Some(instr);
}

pub fn display(buf: &[u8]) {
  let mut r = buf;
  let mut label_id = 0;
  let mut value_id = 0;

  loop {
    match read(&mut r) {
      None => { break; }
      Some(inst) => {
        match inst {
          Instruction::Func(nrets, args) => {
            print!("{}: func {} (", label_id, nrets);
            label_id = label_id + 1;
            for (i, ty) in args.iter().enumerate() {
              if i != 0 {
                print!(", ");
              }
              print!("%{} {}", value_id, ty);
              value_id = value_id + 1;
            }
            print!(")\n");
          }
          Instruction::Case() => {
            print!("{}: case\n", label_id);
            label_id = label_id + 1;
          }
          Instruction::Join(args) => {
            print!("{}: join (", label_id);
            label_id = label_id + 1;
            for (i, ty) in args.iter().enumerate() {
              if i != 0 {
                print!(", ");
              }
              print!("%{} {}", value_id, ty);
              value_id = value_id + 1;
            }
            print!(")\n");
          }
          Instruction::ImmI64(c) => {
            print!("\t%{} = i64.imm #{}\n", value_id, c);
            value_id = value_id + 1;
          }
          Instruction::Op1(t, Value(x)) => {
            print!("\t%{} = {} %{}\n", value_id, t, x);
            value_id = value_id + 1;
          }
          Instruction::Op2(t, Value(x), Value(y)) => {
            print!("\t%{} = {} %{} %{}\n", value_id, t, x, y);
            value_id = value_id + 1;
          }
          Instruction::Select(Value(p), Value(x), Value(y)) => {
            print!("\t%{} = select %{} %{} %{}\n", value_id, p, x, y);
            value_id = value_id + 1;
          }
          Instruction::Cond(Value(p), Label(a), Label(b)) => {
            print!("\tcond %{} =>{} =>{}\n", p, a, b);
          }
          Instruction::Jump(Label(a), args) => {
            print!("\tjump =>{} (", a);
            for (i, Value(x)) in args.iter().enumerate() {
              if i != 0 {
                print!(", ");
              }
              print!("%{}", x);
            }
            print!(")\n");
          }
          Instruction::Ret(index, args) => {
            print!("\tret {} (", index);
            for (i, Value(x)) in args.iter().enumerate() {
              if i != 0 {
                print!(", ");
              }
              print!("%{}", x);
            }
            print!(")\n");
          }
          _ => {
            print!("UNKNOWN INSTRUCTION\n")
          }

        }
      }
    }
  }
}
