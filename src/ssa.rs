use crate::prelude::*;

pub enum Instruction<'a> {
  // BB entry

  Function(u32, TypeList<'a>),
  Case(),
  Join(TypeList<'a>),
  Kont(TypeList<'a>),

  // BB internal

  ConstBool(bool),
  ConstI32(u32),
  ConstI64(u64),
  Op1(Op1, Value),
  Op2(Op2, Value, Value),
  Select(Value, Value, Value),

  // BB terminator

  If(Value, Label, Label),
  Return(u32, ValueList<'a>),
  Goto(Label, ValueList<'a>),
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
  pub const FUNCTION: Self = Self(0x01);
  pub const CASE: Self = Self(0x02);
  pub const JOIN: Self = Self(0x03);
  pub const KONT: Self = Self(0x04);

  pub const CONST_BOOL: Self = Self(0x05);
  pub const CONST_I32: Self = Self(0x08);
  pub const CONST_I64: Self = Self(0x09);
  pub const OP1: Self = Self(0x05);
  pub const OP2: Self = Self(0x06);
  pub const SELECT: Self = Self(0x07);

  pub const IF: Self = Self(0x0a);
  pub const GOTO: Self = Self(0x0b);
  pub const RETURN: Self = Self(0x0c);
  pub const CALL: Self = Self(0x0d);
  pub const TAILCALL: Self = Self(0x0e);
}

impl Type {
  pub const BOOL: Self = Self(0x00);
  pub const I5: Self = Self(0x03);
  pub const I6: Self = Self(0x04);
  pub const I32: Self = Self(0x07);
  pub const I64: Self = Self(0x08);

  pub fn name(self) -> &'static str {
    self.info().0
  }

  fn info(self)
    -> &'static (
      &'static str,
    )
  {
    match self {
      Self::I64 => &(
        "i64",
      ),
      _ => &(
        "unknown",
      )
    }
  }
}

impl core::fmt::Display for Type {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.name())
  }
}

impl Op1 {
  pub const CAST_I32_I64_SX: Self = Self(0x01);
  pub const CAST_I32_I64_ZX: Self = Self(0x02);
  pub const CAST_I64_I32: Self = Self(0x03);
  pub const CTZ_I64: Self = Self(0x07);
  pub const NEG_I64: Self = Self(0x08);

  pub fn name(self) -> &'static str {
    self.info().0
  }

  fn info(self)
    -> &'static (
      &'static str,
    )
  {
    match self {
      Self::CTZ_I64 => &(
        "ctz.i64",
      ),
      Self::NEG_I64 => &(
        "neg.i64",
      ),
      _ => &(
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
  pub const ADD_I64: Self = Self(0x06);
  pub const SUB_I64: Self = Self(0x07);
  pub const IS_EQ_I64: Self = Self(0x08);

  pub fn name(self) -> &'static str {
    self.info().0
  }

  fn info(self)
    -> &'static (
      &'static str,
    )
  {
    match self {
      Self::ADD_I64 => &(
        "add.i64",
      ),
      Self::SUB_I64 => &(
        "sub.i64",
      ),
      Self::IS_EQ_I64 => &(
        "is_eq.i64",
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

pub struct SsaBuf {
  buf: Buf,
  value_id: u32,
  label_id: u32,
}

#[derive(Clone, Copy)]
pub struct PatchPoint(pub usize);

impl SsaBuf {
  pub fn new() -> Self {
    Self {
      buf: Buf::new(),
      value_id: 0,
      label_id: 0,
    }
  }

  pub fn next_value(&mut self) -> Value {
    let n = self.value_id;
    self.value_id = n + 1;
    Value(n)
  }

  pub fn next_label(&mut self) -> Label {
    let n = self.label_id;
    self.label_id = n + 1;
    Label(n)
  }

  pub fn view(&self) -> &[u8] {
    self.buf.view()
  }

  pub fn patch_point(&self) -> PatchPoint {
    PatchPoint(self.buf.len())
  }

  pub fn patch_label(&mut self, PatchPoint(u): PatchPoint, Label(a): Label) {
    let mut w = self.buf.get_slice_mut(u, 4);
    w.put_u32(a)
  }

  pub fn emit_parameter(&mut self, Type(t): Type) -> Value {
    let mut w = self.buf.append(1);
    w.put_u8(t);
    self.next_value()
  }

  pub fn emit_value(&mut self, Value(x): Value) {
    let mut w = self.buf.append(4);
    w.put_u32(x);
  }

  pub fn emit_function(&mut self, nkonts: u32, nargs: u32) {
    let mut w = self.buf.append(9);
    w.put_u8(Tag::FUNCTION.0);
    w.put_u32(nkonts);
    w.put_u32(nargs);
    self.value_id = 0;
    self.label_id = 1;
  }

  pub fn emit_case(&mut self) -> Label {
    let mut w = self.buf.append(1);
    w.put_u8(Tag::CASE.0);
    self.next_label()
  }

  pub fn emit_join(&mut self, nargs: u32) -> Label {
    let mut w = self.buf.append(5);
    w.put_u8(Tag::JOIN.0);
    w.put_u32(nargs);
    self.next_label()
  }

  pub fn emit_const_bool(&mut self, p: bool) -> Value {
    let mut w = self.buf.append(2);
    w.put_u8(Tag::CONST_BOOL.0);
    w.put_u8(p as u8);
    self.next_value()
  }

  pub fn emit_const_i64(&mut self, c: u64) -> Value {
    let mut w = self.buf.append(9);
    w.put_u8(Tag::CONST_I64.0);
    w.put_u64(c);
    self.next_value()
  }

  pub fn emit_op1(&mut self, Op1(t): Op1, Value(x): Value) -> Value {
    let mut w = self.buf.append(6);
    w.put_u8(Tag::OP1.0);
    w.put_u8(t);
    w.put_u32(x);
    self.next_value()
  }

  pub fn emit_op2(&mut self, Op2(t): Op2, Value(x): Value, Value(y): Value) -> Value {
    let mut w = self.buf.append(10);
    w.put_u8(Tag::OP2.0);
    w.put_u8(t);
    w.put_u32(x);
    w.put_u32(y);
    self.next_value()
  }

  pub fn emit_select(&mut self, Value(p): Value, Value(x): Value, Value(y): Value) -> Value {
    let mut w = self.buf.append(13);
    w.put_u8(Tag::SELECT.0);
    w.put_u32(p);
    w.put_u32(x);
    w.put_u32(y);
    self.next_value()
  }

  pub fn emit_if(&mut self, Value(p): Value, Label(a): Label, Label(b): Label) -> (PatchPoint, PatchPoint) {
    let mut w = self.buf.append(13);
    w.put_u8(Tag::IF.0);
    w.put_u32(p);
    w.put_u32(a);
    w.put_u32(b);
    let a = PatchPoint(self.buf.len() - 8);
    let b = PatchPoint(self.buf.len() - 4);
    (a, b)
  }

  pub fn emit_goto(&mut self, Label(a): Label, nargs: u32) -> PatchPoint {
    let mut w = self.buf.append(9);
    w.put_u8(Tag::GOTO.0);
    w.put_u32(a);
    w.put_u32(nargs);
    PatchPoint(self.buf.len() - 8)
  }

  pub fn emit_return(&mut self, index: u32, nargs: u32) {
    let mut w = self.buf.append(9);
    w.put_u8(Tag::RETURN.0);
    w.put_u32(index);
    w.put_u32(nargs);
  }
}

fn chomp<'a, 'b>(buf: &'a mut &'b [u8], size: usize) -> Option<&'b [u8]> {
  if size <= buf.len() {
    Some(buf.pop_slice(size))
  } else {
    None
  }
}

pub fn read<'a, 'b>(buf: &'a mut &'b [u8]) -> Option<Instruction<'b>> {
  let mut cursor = *buf;

  let instr =
    match Tag(chomp(&mut cursor, 1)?.pop_u8()) {
      Tag::FUNCTION => {
        let mut r = chomp(&mut cursor, 8)?;
        let nkonts = r.pop_u32();
        let nargs = r.pop_u32();
        let mut r = chomp(&mut cursor, nargs as usize)?;
        Instruction::Function(nkonts, TypeList(r.pop_all()))
      }
      Tag::CASE => {
        Instruction::Case()
      }
      Tag::JOIN => {
        let mut r = chomp(&mut cursor, 4)?;
        let nargs = r.pop_u32();
        let mut r = chomp(&mut cursor, nargs as usize)?;
        Instruction::Join(TypeList(r.pop_all()))
      }
      Tag::CONST_BOOL => {
        let mut r = chomp(&mut cursor, 1)?;
        Instruction::ConstBool(r.pop_u8() != 0)
      }
      Tag::CONST_I64 => {
        let mut r = chomp(&mut cursor, 8)?;
        Instruction::ConstI64(r.pop_u64())
      }
      Tag::OP1 => {
        let mut r = chomp(&mut cursor, 5)?;
        let t = Op1(r.pop_u8());
        let x = Value(r.pop_u32());
        Instruction::Op1(t, x)
      }
      Tag::OP2 => {
        let mut r = chomp(&mut cursor, 9)?;
        let t = Op2(r.pop_u8());
        let x = Value(r.pop_u32());
        let y = Value(r.pop_u32());
        Instruction::Op2(t, x, y)
      }
      Tag::SELECT => {
        let mut r = chomp(&mut cursor, 12)?;
        let p = Value(r.pop_u32());
        let x = Value(r.pop_u32());
        let y = Value(r.pop_u32());
        Instruction::Select(p, x, y)
      }
      Tag::IF => {
        let mut r = chomp(&mut cursor, 12)?;
        let p = Value(r.pop_u32());
        let a = Label(r.pop_u32());
        let b = Label(r.pop_u32());
        Instruction::If(p, a, b)
      }
      Tag::GOTO => {
        let mut r = chomp(&mut cursor, 8)?;
        let a = r.pop_u32();
        let nargs = r.pop_u32();
        let mut r = chomp(&mut cursor, nargs as usize * 4)?;
        Instruction::Goto(Label(a), ValueList(r.pop_all()))
      }
      Tag::RETURN => {
        let mut r = chomp(&mut cursor, 8)?;
        let index = r.pop_u32();
        let nargs = r.pop_u32();
        let mut r = chomp(&mut cursor, nargs as usize * 4)?;
        Instruction::Return(index, ValueList(r.pop_all()))
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
  let mut function_id = 0;
  let mut label_id = 0;
  let mut value_id = 0;
  let mut nkonts = 0;

  loop {
    match read(&mut r) {
      None => { break; }
      Some(inst) => {
        match inst {
          Instruction::Function(n, args) => {
            label_id = 0;
            value_id = 0;
            nkonts = n;
            print!("{}: function ${} (", function_id, label_id);
            function_id = function_id + 1;
            label_id = label_id + 1;
            for (i, ty) in args.iter().enumerate() {
              if i != 0 {
                print!(", ");
              }
              print!("%{} {}", value_id, ty);
              value_id = value_id + 1;
            }
            print!(") -> ");
            if nkonts == 0 {
              print!("!");
            } else {
              print!("(");
              for i in 0 .. nkonts {
                if i != 0 {
                  print!("|");
                }
                print!("...");
              }
              print!(")");
            }
            print!("\n");
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
          Instruction::ConstBool(p) => {
            print!("\t%{} = const.bool #{}\n", value_id, p);
            value_id = value_id + 1;
          }
          Instruction::ConstI64(c) => {
            print!("\t%{} = const.i64 #{}\n", value_id, c);
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
          Instruction::If(Value(p), Label(a), Label(b)) => {
            print!("\tif %{} =>{} =>{}\n", p, a, b);
          }
          Instruction::Goto(Label(a), args) => {
            print!("\tgoto =>{} (", a);
            for (i, Value(x)) in args.iter().enumerate() {
              if i != 0 {
                print!(", ");
              }
              print!("%{}", x);
            }
            print!(")\n");
          }
          Instruction::Return(index, args) => {
            print!("\treturn (", );
            for _ in 0 .. index {
              print!("|")
            }
            for (i, Value(x)) in args.iter().enumerate() {
              if i != 0 {
                print!(", ");
              }
              print!("%{}", x);
            }
            for _ in 0 .. nkonts.saturating_sub(index).saturating_sub(1) {
              print!("|")
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
