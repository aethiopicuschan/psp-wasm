use crate::types::MyI32;

// モック実装。今回は成功を示す 0 を返すようにしている。
pub fn fd_write(_i1: MyI32, _i2: MyI32, _i3: MyI32, _i4: MyI32) -> MyI32 {
    MyI32(0)
}
