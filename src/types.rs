use wasmi::{
    core::{UntypedValue, ValueType},
    WasmType,
};

pub struct MyF64(pub f64);

impl WasmType for MyF64 {
    fn ty() -> ValueType {
        ValueType::F64
    }
}

impl From<UntypedValue> for MyF64 {
    fn from(value: UntypedValue) -> Self {
        MyF64(f64::from(value))
    }
}

impl Into<UntypedValue> for MyF64 {
    fn into(self) -> UntypedValue {
        UntypedValue::from(self.0)
    }
}

pub struct MyI32(pub i32);

impl WasmType for MyI32 {
    fn ty() -> ValueType {
        ValueType::I32
    }
}

impl From<UntypedValue> for MyI32 {
    fn from(value: UntypedValue) -> Self {
        MyI32(i32::from(value))
    }
}

impl Into<UntypedValue> for MyI32 {
    fn into(self) -> UntypedValue {
        UntypedValue::from(self.0)
    }
}
