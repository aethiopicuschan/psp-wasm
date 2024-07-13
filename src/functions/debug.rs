extern crate alloc;
use crate::types::MyI32;
use alloc::vec::Vec;
use wasmi::{Caller, Memory};

// デバッグ用の関数。文字列を出力する。
pub fn println(caller: Caller<'_, ()>, memory: &Memory, ptr: MyI32, len: MyI32) {
    // WASMのメモリ空間から文字列を取得する
    let ptr = ptr.0 as usize;
    let len = len.0 as usize;
    let mut buf = Vec::with_capacity(len);
    unsafe {
        buf.set_len(len);
    }
    memory.read(&caller, ptr, &mut buf).unwrap();
    let string = core::str::from_utf8(&buf).unwrap();
    psp::dprintln!("{}", string);
}
