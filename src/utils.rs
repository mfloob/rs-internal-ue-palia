use std::ffi::CString;
use once_cell::sync::OnceCell;
use windows::{
    Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState,
    Win32::System::{
            Console::{AllocConsole, FreeConsole}, 
            LibraryLoader::{GetModuleHandleW, FreeLibraryAndExitThread, GetModuleHandleA}
        }, core::{PCWSTR, PCSTR},    
};

use crate::ue;

// old: 0x7989C10
// 7FF7DDC60090 - 0x7A10090
// 48 89 5C 24 ? 57 48 83 EC 20 4C 8B 89 ? ? ? ? 33 DB
// .text:00007FF7D87ED5D2                 sar     ecx, 10h
// .text:00007FF7D87ED5D5                 movsxd  rcx, ecx
// .text:00007FF7D87ED5D8                 lea     rdx, [rax+rax*2]
// .text:00007FF7D87ED5DC                 mov     rax, cs:qword_7FF7DDC60090 <---------------------
// .text:00007FF7D87ED5E3                 mov     rcx, [rax+rcx*8]
// .text:00007FF7D87ED5E7                 lea     r8, [rcx+rdx*8]
// .text:00007FF7D87ED5EB                 jmp     short loc_7FF7D87ED5F0
pub unsafe fn get_g_objects() -> *const ue::TUObjectArray {
    let base = get_base_address();
    std::mem::transmute(base + 0x81b5060)
}

// old: 0x78EA280
// 7FF7DDBC0700 - 0x7970700
// "ERROR_NAME_SIZE_EXCEEDED" 
//   2nd xref then scroll down
// .text:00007FF7D6BEEE1F                 cmp     cs:byte_7FF7DDB9CB39, bl
// .text:00007FF7D6BEEE25                 jz      loc_7FF7D6BEED7D
// .text:00007FF7D6BEEE2B                 lea     rax, FNAMEPOOLDATA <------------------------------
// .text:00007FF7D6BEEE32                 jmp     loc_7FF7D6BEED90
pub unsafe fn get_g_names() -> *const ue::FNamePool {
    let base = get_base_address();
    std::mem::transmute(base + 0x8115540)
}

// old: 0x7AF7D50
// 7FF7D6D98355 - 0x7B7E1D0
// jump to name aScreenshotname
// xref "{}"
// .text:00007FF7D7A6BF8B                 lea     rdx, asc_7FF7DC0AB5C0 ; "{}"
// .text:00007FF7D7A6BF92                 call    sub_7FF7D8627DB0
// .text:00007FF7D7A6BF97                 lea     r12, [r13+20h]
// .text:00007FF7D7A6BF9B                 mov     rdx, rbx
// .text:00007FF7D7A6BF9E                 mov     rcx, r12
// .text:00007FF7D7A6BFA1                 call    sub_7FF7D8627DB0
// .text:00007FF7D7A6BFA6                 mov     r14, [rbp+5E0h+arg_20]
// .text:00007FF7D7A6BFAD                 movsxd  rdi, dword ptr [r14+8]
// .text:00007FF7D7A6BFB1                 cmp     edi, 1
// .text:00007FF7D7A6BFB4                 jg      short loc_7FF7D7A6BFEA
// .text:00007FF7D7A6BFB6                 mov     rax, cs:MY_GWORLD <-----------------
pub unsafe fn get_g_world() -> *const *const ue::UWorld {
    let base = get_base_address();
    std::mem::transmute(base + 0x830fd50)
}

pub unsafe fn get_font() -> *const ue::UFont {
    let g_objects = ue::G_OBJECTS.unwrap();
    if g_objects as u64 == 0 {
        return std::ptr::null();
    }

    static FONT: OnceCell<usize> = OnceCell::new();
    FONT.get_or_init(|| {
        for i in 0..(*g_objects).num_elements {
            let object = (*g_objects).get_object_by_index(i);
            if object.is_null() {
                continue;
            }
    
            let obj_name = (*object).get_full_name();
            if obj_name == "Font Roboto.Roboto" {
                return object as usize;
            }
        }

        return 0;
    });

    return *FONT.get().unwrap() as *const ue::UFont;
}

pub unsafe fn key_released(key: u16) -> bool {
    static mut PRESSED_KEYS: [bool; 255] = [false; 255];

    let result = GetAsyncKeyState(key as i32);
    let is_pressed = !((result >> 15) & 1 == 0);
    let was_pressed = PRESSED_KEYS[key as usize];

    let index = key as usize;
    if is_pressed && was_pressed {
        return false;
    }

    if is_pressed && !was_pressed {
        PRESSED_KEYS[index] = true;
        return false;
    }

    if !is_pressed && !was_pressed {
        return false;
    }

    PRESSED_KEYS[index] = false;
    return true;
}

pub fn get_base_address() -> u64 {
    unsafe {
        GetModuleHandleW(PCWSTR(std::ptr::null())).unwrap().0 as u64
    }
}

pub fn alloc_console() {
    unsafe {
        AllocConsole();
    }
}

pub fn free_console() {
    unsafe {
        FreeConsole();
    }
}

pub fn unload() { 
    unsafe {
        let module_name = CString::new("rs_internal_ue_palia.dll").unwrap();
        let module = GetModuleHandleA(PCSTR::from_raw(module_name.as_ptr() as *const u8)).unwrap();
        FreeLibraryAndExitThread(module, 0);
    }
}