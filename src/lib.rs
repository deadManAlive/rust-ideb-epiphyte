use windows::{
    Win32::{
        Foundation::HMODULE,
        System::SystemServices::DLL_PROCESS_ATTACH,
        UI::WindowsAndMessaging::{MB_OK, MessageBoxA},
    },
    core::s,
};

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(_: HMODULE, fwd_reason: u32, _: *mut core::ffi::c_void) -> bool {
    if fwd_reason == DLL_PROCESS_ATTACH {
        unsafe {
            MessageBoxA(None, s!("Hello, world!"), s!("as"), MB_OK);
        }
    }

    true
}
