extern crate alloc;
use alloc::vec::Vec;
use wasmi::{Caller, Memory};

pub fn println(caller: Caller<'_, ()>, memory: &Memory, ptr: i32, len: i32) {
    let ptr = ptr as usize;
    let len = len as usize;
    let mut buf = Vec::with_capacity(len);
    unsafe {
        buf.set_len(len);
    }
    memory.read(&caller, ptr, &mut buf).unwrap();
    let string = core::str::from_utf8(&buf).unwrap();
    psp::dprintln!("{}", string);
}
