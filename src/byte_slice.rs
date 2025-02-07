//! byte slice utils

use crate::prelude::*;

pub trait ByteSlice {
  fn ptr(&self) -> ptr;

  fn get_chunk<const K: usize>(&self, offset: usize) -> &[u8; K];

  fn get_chunk_mut<const K: usize>(&mut self, offset: usize) -> &mut [u8; K];

  fn pop_chunk<'a, 'b, const K: usize>(self: &'a mut &'b Self) -> &'b [u8; K];

  fn pop_chunk_mut<'a, 'b, const K: usize>(self: &'a mut &'b mut Self) -> &'b mut [u8; K];

  fn pop_slice<'a, 'b>(self: &'a mut &'b Self, k: usize) -> &'b [u8];

  fn pop_slice_mut<'a, 'b>(self: &'a mut &'b mut Self, k: usize) -> &'b mut [u8];

  fn pop_all<'a, 'b>(self: &'a mut &'b Self) -> &'b [u8];

  fn pop_all_mut<'a, 'b>(self: &'a mut &'b mut Self) -> &'b mut [u8];

  fn put_u8(self: &mut &mut Self, value: u8);

  fn put_u16(self: &mut &mut Self, value: u16);

  fn put_u32(self: &mut &mut Self, value: u32);

  fn put_u64(self: &mut &mut Self, value: u64);

  fn put_f32(self: &mut &mut Self, value: f32);

  fn put_f64(self: &mut &mut Self, value: f64);

  fn pop_u8(self: &mut &Self) -> u8;

  fn pop_u16(self: &mut &Self) -> u16;

  fn pop_u32(self: &mut &Self) -> u32;

  fn pop_u64(self: &mut &Self) -> u64;

  fn pop_f32(self: &mut &Self) -> f32;

  fn pop_f64(self: &mut &Self) -> f64;

  fn iter_chunks<const K: usize>(&self) -> impl Iterator<Item = &[u8; K]>;
}

impl ByteSlice for [u8] {
  #[inline(always)]
  fn ptr(&self) -> ptr {
    ptr::from(self)
  }

  #[inline(always)]
  fn get_chunk<const K: usize>(&self, offset: usize) -> &[u8; K] {
    let p = self.ptr();
    let n = self.len();
    assert!(offset <= n && K <= n - offset);
    unsafe { (p + offset).as_ref() }
  }

  #[inline(always)]
  fn get_chunk_mut<const K: usize>(&mut self, offset: usize) -> &mut [u8; K] {
    let p = self.ptr();
    let n = self.len();
    assert!(offset <= n && K <= n - offset);
    unsafe { (p + offset).as_mut_ref() }
  }

  #[inline(always)]
  fn pop_chunk<'a, 'b, const K: usize>(self: &'a mut &'b [u8]) -> &'b [u8; K] {
    let p = self.ptr();
    let n = self.len();
    assert!(K <= n);
    *self = unsafe { (p + K).as_slice_ref(n - K) };
    unsafe { p.as_ref() }
  }

  #[inline(always)]
  fn pop_chunk_mut<'a, 'b, const K: usize>(self: &'a mut &'b mut [u8]) -> &'b mut [u8; K] {
    let p = self.ptr();
    let n = self.len();
    assert!(K <= n);
    *self = unsafe { (p + K).as_slice_mut_ref(n - K) };
    unsafe { p.as_mut_ref() }
  }

  #[inline(always)]
  fn pop_slice<'a, 'b>(self: &'a mut &'b Self, k: usize) -> &'b [u8] {
    let p = self.ptr();
    let n = self.len();
    assert!(k <= n);
    *self = unsafe { (p + k).as_slice_ref(n - k) };
    unsafe { p.as_slice_ref(k) }
  }

  #[inline(always)]
  fn pop_slice_mut<'a, 'b>(self: &'a mut &'b mut Self, k: usize) -> &'b mut [u8] {
    let p = self.ptr();
    let n = self.len();
    assert!(k <= n);
    *self = unsafe { (p + k).as_slice_mut_ref(n - k) };
    unsafe { p.as_slice_mut_ref(k) }
  }

  #[inline(always)]
  fn pop_all<'a, 'b>(self: &'a mut &'b Self) -> &'b [u8] {
    core::mem::replace(self, &[])
  }

  #[inline(always)]
  fn pop_all_mut<'a, 'b>(self: &'a mut &'b mut Self) -> &'b mut [u8] {
    core::mem::replace(self, &mut [])
  }

  #[inline(always)]
  fn put_u8(self: &mut &mut [u8], value: u8) {
    *self.pop_chunk_mut() = value.to_le_bytes();
  }

  #[inline(always)]
  fn put_u16(self: &mut &mut [u8], value: u16) {
    *self.pop_chunk_mut() = value.to_le_bytes();
  }

  #[inline(always)]
  fn put_u32(self: &mut &mut [u8], value: u32) {
    *self.pop_chunk_mut() = value.to_le_bytes();
  }

  #[inline(always)]
  fn put_u64(self: &mut &mut [u8], value: u64) {
    *self.pop_chunk_mut() = value.to_le_bytes();
  }

  #[inline(always)]
  fn put_f32(self: &mut &mut [u8], value: f32) {
    *self.pop_chunk_mut() = value.to_le_bytes();
  }

  #[inline(always)]
  fn put_f64(self: &mut &mut [u8], value: f64) {
    *self.pop_chunk_mut() = value.to_le_bytes();
  }

  #[inline(always)]
  fn pop_u8(self: &mut &[u8]) -> u8 {
    u8::from_le_bytes(*self.pop_chunk())
  }

  #[inline(always)]
  fn pop_u16(self: &mut &[u8]) -> u16 {
    u16::from_le_bytes(*self.pop_chunk())
  }

  #[inline(always)]
  fn pop_u32(self: &mut &[u8]) -> u32 {
    u32::from_le_bytes(*self.pop_chunk())
  }

  #[inline(always)]
  fn pop_u64(self: &mut &[u8]) -> u64 {
    u64::from_le_bytes(*self.pop_chunk())
  }

  #[inline(always)]
  fn pop_f32(self: &mut &[u8]) -> f32 {
    f32::from_le_bytes(*self.pop_chunk())
  }

  #[inline(always)]
  fn pop_f64(self: &mut &[u8]) -> f64 {
    f64::from_le_bytes(*self.pop_chunk())
  }

  #[inline(always)]
  fn iter_chunks<const K: usize>(&self) -> impl Iterator<Item = &[u8; K]> {
    IterChunks(self)
  }
}

struct IterChunks<'a, const K: usize>(&'a [u8]);

impl<'a, const K: usize> Iterator for IterChunks<'a, K> {
  type Item = &'a [u8; K];

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if K <= self.0.len() {
      Some(self.0.pop_chunk())
    } else {
      None
    }
  }
}
