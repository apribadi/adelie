#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Op2 {
  I64Add,
  I64Asr,
  I64BitAnd,
  I64BitOr,
  I64BitXor,
  I64Lsr,
  I64Mul,
  I64Rot,
  I64Shl,
  I64Sub,
  Unknown0x0A,
  Unknown0x0B,
  Unknown0x0C,
  Unknown0x0D,
  Unknown0x0E,
  Unknown0x0F,
  Unknown0x10,
  Unknown0x11,
  Unknown0x12,
  Unknown0x13,
  Unknown0x14,
  Unknown0x15,
  Unknown0x16,
  Unknown0x17,
  Unknown0x18,
  Unknown0x19,
  Unknown0x1A,
  Unknown0x1B,
  Unknown0x1C,
  Unknown0x1D,
  Unknown0x1E,
  Unknown0x1F,
  Unknown0x20,
  Unknown0x21,
  Unknown0x22,
  Unknown0x23,
  Unknown0x24,
  Unknown0x25,
  Unknown0x26,
  Unknown0x27,
  Unknown0x28,
  Unknown0x29,
  Unknown0x2A,
  Unknown0x2B,
  Unknown0x2C,
  Unknown0x2D,
  Unknown0x2E,
  Unknown0x2F,
  Unknown0x30,
  Unknown0x31,
  Unknown0x32,
  Unknown0x33,
  Unknown0x34,
  Unknown0x35,
  Unknown0x36,
  Unknown0x37,
  Unknown0x38,
  Unknown0x39,
  Unknown0x3A,
  Unknown0x3B,
  Unknown0x3C,
  Unknown0x3D,
  Unknown0x3E,
  Unknown0x3F,
  Unknown0x40,
  Unknown0x41,
  Unknown0x42,
  Unknown0x43,
  Unknown0x44,
  Unknown0x45,
  Unknown0x46,
  Unknown0x47,
  Unknown0x48,
  Unknown0x49,
  Unknown0x4A,
  Unknown0x4B,
  Unknown0x4C,
  Unknown0x4D,
  Unknown0x4E,
  Unknown0x4F,
  Unknown0x50,
  Unknown0x51,
  Unknown0x52,
  Unknown0x53,
  Unknown0x54,
  Unknown0x55,
  Unknown0x56,
  Unknown0x57,
  Unknown0x58,
  Unknown0x59,
  Unknown0x5A,
  Unknown0x5B,
  Unknown0x5C,
  Unknown0x5D,
  Unknown0x5E,
  Unknown0x5F,
  Unknown0x60,
  Unknown0x61,
  Unknown0x62,
  Unknown0x63,
  Unknown0x64,
  Unknown0x65,
  Unknown0x66,
  Unknown0x67,
  Unknown0x68,
  Unknown0x69,
  Unknown0x6A,
  Unknown0x6B,
  Unknown0x6C,
  Unknown0x6D,
  Unknown0x6E,
  Unknown0x6F,
  Unknown0x70,
  Unknown0x71,
  Unknown0x72,
  Unknown0x73,
  Unknown0x74,
  Unknown0x75,
  Unknown0x76,
  Unknown0x77,
  Unknown0x78,
  Unknown0x79,
  Unknown0x7A,
  Unknown0x7B,
  Unknown0x7C,
  Unknown0x7D,
  Unknown0x7E,
  Unknown0x7F,
  Unknown0x80,
  Unknown0x81,
  Unknown0x82,
  Unknown0x83,
  Unknown0x84,
  Unknown0x85,
  Unknown0x86,
  Unknown0x87,
  Unknown0x88,
  Unknown0x89,
  Unknown0x8A,
  Unknown0x8B,
  Unknown0x8C,
  Unknown0x8D,
  Unknown0x8E,
  Unknown0x8F,
  Unknown0x90,
  Unknown0x91,
  Unknown0x92,
  Unknown0x93,
  Unknown0x94,
  Unknown0x95,
  Unknown0x96,
  Unknown0x97,
  Unknown0x98,
  Unknown0x99,
  Unknown0x9A,
  Unknown0x9B,
  Unknown0x9C,
  Unknown0x9D,
  Unknown0x9E,
  Unknown0x9F,
  Unknown0xA0,
  Unknown0xA1,
  Unknown0xA2,
  Unknown0xA3,
  Unknown0xA4,
  Unknown0xA5,
  Unknown0xA6,
  Unknown0xA7,
  Unknown0xA8,
  Unknown0xA9,
  Unknown0xAA,
  Unknown0xAB,
  Unknown0xAC,
  Unknown0xAD,
  Unknown0xAE,
  Unknown0xAF,
  Unknown0xB0,
  Unknown0xB1,
  Unknown0xB2,
  Unknown0xB3,
  Unknown0xB4,
  Unknown0xB5,
  Unknown0xB6,
  Unknown0xB7,
  Unknown0xB8,
  Unknown0xB9,
  Unknown0xBA,
  Unknown0xBB,
  Unknown0xBC,
  Unknown0xBD,
  Unknown0xBE,
  Unknown0xBF,
  Unknown0xC0,
  Unknown0xC1,
  Unknown0xC2,
  Unknown0xC3,
  Unknown0xC4,
  Unknown0xC5,
  Unknown0xC6,
  Unknown0xC7,
  Unknown0xC8,
  Unknown0xC9,
  Unknown0xCA,
  Unknown0xCB,
  Unknown0xCC,
  Unknown0xCD,
  Unknown0xCE,
  Unknown0xCF,
  Unknown0xD0,
  Unknown0xD1,
  Unknown0xD2,
  Unknown0xD3,
  Unknown0xD4,
  Unknown0xD5,
  Unknown0xD6,
  Unknown0xD7,
  Unknown0xD8,
  Unknown0xD9,
  Unknown0xDA,
  Unknown0xDB,
  Unknown0xDC,
  Unknown0xDD,
  Unknown0xDE,
  Unknown0xDF,
  Unknown0xE0,
  Unknown0xE1,
  Unknown0xE2,
  Unknown0xE3,
  Unknown0xE4,
  Unknown0xE5,
  Unknown0xE6,
  Unknown0xE7,
  Unknown0xE8,
  Unknown0xE9,
  Unknown0xEA,
  Unknown0xEB,
  Unknown0xEC,
  Unknown0xED,
  Unknown0xEE,
  Unknown0xEF,
  Unknown0xF0,
  Unknown0xF1,
  Unknown0xF2,
  Unknown0xF3,
  Unknown0xF4,
  Unknown0xF5,
  Unknown0xF6,
  Unknown0xF7,
  Unknown0xF8,
  Unknown0xF9,
  Unknown0xFA,
  Unknown0xFB,
  Unknown0xFC,
  Unknown0xFD,
  Unknown0xFE,
  Unknown0xFF,
}

const _: () = assert!(Op2::Unknown0xFF as u8 == 0xff);

impl Op2 {
  #[inline(always)]
  pub fn encode(self) -> u8 {
    self as u8
  }

  #[inline(always)]
  pub fn decode(x: u8) -> Self {
    unsafe { std::mem::transmute(x) }
  }
}
