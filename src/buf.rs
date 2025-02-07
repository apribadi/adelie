use crate::prelude::*;

pub struct Buf {
  ptr: ptr,
  cap: usize,
  len: usize,
}

#[inline(never)]
#[cold]
unsafe fn grow(_ptr: ptr, cap: usize, _len: usize, more: usize) -> (ptr, usize) {
  assert!(more <= isize::MAX as usize);

  if cap == 0 {
    let size = usize::max(more, 1024);
    let layout = core::alloc::Layout::from_size_align(size, 1).unwrap();
    let p = unsafe { alloc::alloc::alloc_zeroed(layout) };
    let p = ptr::from(p);
    assert!(! p.is_null());
    return (p, size);
  } else {
    todo!()
  }
}


impl Buf {
  #[inline(always)]
  pub fn new() -> Self {
    Self {
      ptr: ptr::invalid(1),
      cap: 0,
      len: 0,
    }
  }

  #[inline(always)]
  fn reserve(&mut self, more: usize) {
    if more <= self.cap - self.len { return; }
    let (p, c) = unsafe { grow(self.ptr, self.cap, self.len, more) };
    self.ptr = p;
    self.cap = c;
  }

  pub fn append(&mut self, size: usize) -> &mut [u8] {
    self.reserve(size);
    let p = self.ptr;
    let n = self.len;
    self.len = n + size;
    unsafe { (p + n).as_slice_mut_ref(size) }
  }

  pub fn view(&self) -> &[u8] {
    unsafe { self.ptr.as_slice_ref(self.len) }
  }
}
