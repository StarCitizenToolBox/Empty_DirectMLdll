#![allow(non_snake_case)]
// Empty DirectML.dll stub
// This mimics the real DirectML.dll exports but returns safe default values
// Used to satisfy build dependencies when DirectML is not actually used

use std::os::raw::c_void;

// Common HRESULT values
const E_NOTIMPL: i32 = 0x80004001u32 as i32;
const E_NOINTERFACE: i32 = 0x80004002u32 as i32;
const S_OK: i32 = 0;

// DirectML main entry point (ordinal 1)
#[no_mangle]
#[inline(never)]
pub extern "system" fn DMLCreateDevice(
    _d3d12_device: *mut c_void,
    _flags: u32,
    _riid: *const c_void,
    
    ppv: *mut *mut c_void,
) -> i32 {
    if !ppv.is_null() {
        unsafe { *ppv = std::ptr::null_mut(); }
    }
    E_NOTIMPL
}

// DMLCreateDevice1 (ordinal 2)
#[no_mangle]
#[inline(never)]
pub extern "system" fn DMLCreateDevice1(
    _d3d12_device: *mut c_void,
    _flags: u32,
    _minimum_feature_level: u32,
    _riid: *const c_void,
    ppv: *mut *mut c_void,
) -> i32 {
    if !ppv.is_null() {
        unsafe { *ppv = std::ptr::null_mut(); }
    }
    E_NOTIMPL
}

// Additional common DirectML exports
#[no_mangle]
#[inline(never)]
pub extern "system" fn DMLGetDebugInterface(
    _riid: *const c_void,
    ppv: *mut *mut c_void,
) -> i32 {
    if !ppv.is_null() {
        unsafe { *ppv = std::ptr::null_mut(); }
    }
    E_NOINTERFACE
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn DllGetClassObject(
    _rclsid: *const c_void,
    _riid: *const c_void,
    ppv: *mut *mut c_void,
) -> i32 {
    if !ppv.is_null() {
        unsafe { *ppv = std::ptr::null_mut(); }
    }
    E_NOINTERFACE
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn DllCanUnloadNow() -> i32 {
    S_OK
}
