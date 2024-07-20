#![no_std]
#![no_main]

mod functions;
mod types;

use wasmi::{Caller, Config, Engine, Func, Linker, Memory, MemoryType, Module, StackLimits, Store};

psp::module!("sample_module", 1, 1);

fn inner_main() {
    // ここからWASM関連
    let mut config = Config::default();
    let stack_limits = match StackLimits::new(256, 512, 128) {
        Ok(stack_limits) => stack_limits,
        Err(e) => {
            psp::dprintln!("Error: {:?}", e);
            return;
        }
    };
    config.set_stack_limits(stack_limits);

    let engine = Engine::new(&config);

    let wasm_binary: &'static [u8] = include_bytes!("../go/hello.wasm");

    let module = match Module::new(&engine, &wasm_binary[..]) {
        Ok(module) => module,
        Err(e) => {
            psp::dprintln!("Error: {:?}", e);
            return;
        }
    };

    let mut store = Store::new(&engine, ());

    let memory_type = match MemoryType::new(1, Some(10)) {
        Ok(memory_type) => memory_type,
        Err(e) => {
            psp::dprintln!("Error: {:?}", e);
            return;
        }
    };
    let memory = match Memory::new(&mut store, memory_type) {
        Ok(memory) => memory,
        Err(e) => {
            psp::dprintln!("Error: {:?}", e);
            return;
        }
    };

    // 関数のラップ
    let host_println = Func::wrap(
        &mut store,
        move |caller: Caller<'_, ()>, ptr: i32, len: i32| {
            let memory = match caller.get_export("memory") {
                Some(export) => match export.into_memory() {
                    Some(memory) => memory,
                    None => {
                        psp::dprintln!("Error: Memory export is not a memory");
                        return;
                    }
                },
                None => {
                    psp::dprintln!("Error: Memory export not found");
                    return;
                }
            };
            functions::debug::println(caller, &memory, ptr, len);
        },
    );
    let host_fd_write = Func::wrap(&mut store, functions::wasi::fd_write);

    psp::dprintln!("到達点1");

    // リンカーを初期化し、関数を登録します。
    let mut linker = <Linker<()>>::new(&engine);
    psp::dprintln!("到達点2");
    match linker.define("env", "memory", memory) {
        Ok(_) => {}
        Err(e) => {
            psp::dprintln!("Error: {:?}", e);
            return;
        }
    }
    match linker.define("debug", "println", host_println) {
        Ok(_) => {}
        Err(e) => {
            psp::dprintln!("Error: {:?}", e);
            return;
        }
    }
    match linker.define("wasi_snapshot_preview1", "fd_write", host_fd_write) {
        Ok(_) => {}
        Err(e) => {
            psp::dprintln!("Error: {:?}", e);
            return;
        }
    }
    psp::dprintln!("到達点3");
    let pre_instance = match linker.instantiate(&mut store, &module) {
        Ok(pre_instance) => pre_instance,
        Err(e) => {
            psp::dprintln!("Error: {:?}", e);
            return;
        }
    };
    psp::dprintln!("到達点4");
    let instance = match pre_instance.start(&mut store) {
        Ok(instance) => instance,
        Err(e) => {
            psp::dprintln!("Error: {:?}", e);
            return;
        }
    };
    psp::dprintln!("到達点5");
    let start = match instance.get_typed_func::<(), ()>(&store, "_start") {
        Ok(start) => start,
        Err(e) => {
            psp::dprintln!("Error: {:?}", e);
            return;
        }
    };
    psp::dprintln!("到達点6");
    match start.call(&mut store, ()) {
        Ok(_) => {}
        Err(e) => {
            psp::dprintln!("Error: {:?}", e);
            return;
        }
    }

    psp::dprintln!("到達点7");
}

fn psp_main() {
    // PSP特有の機能を有効にします。
    psp::enable_home_button();
    if let Err(e) = psp::catch_unwind(|| {
        inner_main();
    }) {
        psp::dprintln!("Error: {:?}", e);
    }
}
