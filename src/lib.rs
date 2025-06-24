use std::{ffi::CString, thread};

use windows::{
    Win32::{
        Foundation::HMODULE,
        System::{LibraryLoader::GetModuleHandleA, SystemServices::DLL_PROCESS_ATTACH},
        UI::WindowsAndMessaging::{MB_OK, MessageBoxA},
    },
    core::{PCSTR, s},
};

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(_: HMODULE, fwd_reason: u32, _: *mut core::ffi::c_void) -> bool {
    if fwd_reason == DLL_PROCESS_ATTACH {
        thread::spawn(|| unsafe {
            let process_handle = GetModuleHandleA(None).unwrap();
            let phstr = format!("process_handle: {process_handle:#?}");
            let phstr = CString::new(phstr).unwrap();

            MessageBoxA(
                None,
                PCSTR(phstr.as_ptr() as *const u8),
                s!("iDebViewer has been injected!"),
                MB_OK,
            );
        });
    }

    true
}
