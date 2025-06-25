mod hexer;

use std::{ffi::CString, mem::transmute, os::raw::c_void, thread};

use axum::{
    Json, Router,
    response::Html,
    routing::{get, post},
};
use minhook::MinHook;
use serde::{Deserialize, Serialize};
use windows::{
    Win32::{
        Foundation::HMODULE,
        System::{
            Diagnostics::Debug::OutputDebugStringA,
            LibraryLoader::GetModuleHandleA,
            SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        },
        UI::WindowsAndMessaging::{MB_OK, MessageBoxA},
    },
    core::PCSTR,
};

use crate::hexer::strstr;

type DecryptionSubroutine = unsafe extern "C" fn(
    a0: i32,
    a1: i32,
    a2: i32,
    a3: i32,
    q_string_password: *mut c_void,
    q_file_ideb_file: *mut c_void,
    q_file_zip_file: *mut c_void,
) -> i32;

static mut ORIGINAL_FUNC: Option<DecryptionSubroutine> = None;
const DECRYPTION_OFFSET: usize = 0x41210;

#[allow(unused)]
fn debug(message: &str) {
    let message = CString::new(message).unwrap();
    unsafe { OutputDebugStringA(PCSTR(message.as_ptr() as _)) };
}

#[allow(unused)]
fn dbgmsgbox(message: &str, title: Option<&str>) {
    let message = CString::new(message).unwrap();
    let title = match title {
        Some(t) => CString::new(t).unwrap(),
        None => CString::new("Debug").unwrap(),
    };
    thread::spawn(move || unsafe {
        MessageBoxA(
            None,
            PCSTR(message.as_ptr() as _),
            PCSTR(title.as_ptr() as _),
            MB_OK,
        );
    });
}

#[derive(Serialize, Deserialize)]
pub struct DecryptionTarget {
    pub password: String,
    pub ideb_path: String,
    pub zip_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Hex {
    pub password: Vec<u8>,
    pub ideb_path: Vec<u8>,
    pub zip_path: Vec<u8>,
}

unsafe extern "C" fn detour_function(
    a0: i32,
    a1: i32,
    a2: i32,
    a3: i32,
    q_string_password: *mut c_void,
    q_file_ideb_file: *mut c_void,
    q_file_zip_file: *mut c_void,
) -> i32 {
    unsafe {
        let msg = format!(
            "a0 = {a0}, a1={a1}, a2={a2}, a3={a3}, password={q_string_password:?}, ideb={q_file_ideb_file:?}, zip={q_file_zip_file:?}"
        );
        debug(&msg);

        if let Some(original_function) = ORIGINAL_FUNC {
            original_function(
                a0,
                a1,
                a2,
                a3,
                q_string_password,
                q_file_ideb_file,
                q_file_zip_file,
            )
        } else {
            0
        }
    }
}

async fn decrypt(Json(payload): Json<DecryptionTarget>) -> Json<Hex> {
    Json(Hex {
        password: strstr(4, &payload.password),
        ideb_path: strstr(3, &payload.ideb_path),
        zip_path: strstr(2, &payload.zip_path),
    })
}

async fn server() {
    let app = Router::new()
        .route("/", get(async || Html("<h1>Hello!</h1>")))
        .route("/decrypt", post(decrypt));

    match tokio::net::TcpListener::bind("127.0.0.1:8070").await {
        Ok(listener) => {
            if let Err(e) = axum::serve(listener, app).await {
                debug(&format!("creating axum service error: {e:#?}"));
            }
        }
        Err(e) => {
            debug(&format!("creating listener error: {e:#?}"));
        }
    }
}

//TODO: how to hand-craft data to feed to the subroutine?
#[unsafe(no_mangle)]
pub extern "system" fn DllMain(_: HMODULE, fwd_reason: u32, _: *mut c_void) -> bool {
    // use thread here from start
    if fwd_reason == DLL_PROCESS_ATTACH {
        thread::spawn(|| unsafe {
            debug("Bonjour! iDebViewer is being injected!");

            let process_handle = GetModuleHandleA(None).unwrap();
            let base_address = process_handle.0 as *const c_void;
            let target_address = base_address.add(DECRYPTION_OFFSET);

            // debug(&format!("target: {target_address:#?}"));
            // debug(&format!("detour: {:#?}", detour_function as *mut c_void));

            let original = match MinHook::create_hook(target_address as _, detour_function as _) {
                Ok(addr) => addr,
                Err(e) => {
                    debug(&format!("Failed creating hook...: {e:#?}"));
                    return;
                }
            };

            // debug(&format!("original after create: {original:#?}"));

            if let Err(e) = MinHook::enable_all_hooks() {
                debug(&format!("Failed enabling hook...: {e:#?}"));
                return;
            }

            ORIGINAL_FUNC = Some(transmute::<*mut c_void, DecryptionSubroutine>(original));

            // if let Some(addr) = ORIGINAL_FUNC {
            //     debug(&format!("original after transmute: {addr:#?}"));
            // } else {
            //     debug("the hell?");
            // }
        });

        // server thread
        thread::spawn(|| {
            match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(b) => b.block_on(server()),
                Err(e) => dbgmsgbox(&format!("tokio runtime error: {e:#?}"), None),
            }
        });
    }

    // do not spawn threads here...
    if fwd_reason == DLL_PROCESS_DETACH {
        MinHook::uninitialize();
        debug("Au revoir!");
    }

    true
}
