// hey, look! compact imports!
use std::{ffi::c_void, mem::transmute, ptr::null};

type HModule = *const c_void;
type FarProc = *const c_void;
extern "stdcall" {
    fn LoadLibraryA(name: *const u8) -> HModule;
    fn GetProcAddress(module: HModule, name: *const u8) -> FarProc;
}

// This one is just for readability
type MessageBoxA = extern "stdcall" fn(*const c_void, *const u8, *const u8, u32);

fn main() {
    unsafe {
        let h = LoadLibraryA("USER32.dll\0".as_ptr());

        let MessageBoxA: MessageBoxA = transmute(GetProcAddress(h, "MessageBoxA\0".as_ptr()));
        // no, you're not having a stroke - yes, you can have a local variable and a
        // type with the same name in scope at the same time!

        MessageBoxA(null(), "Hello from Rust\0".as_ptr(), null(), 0);
    }
}
