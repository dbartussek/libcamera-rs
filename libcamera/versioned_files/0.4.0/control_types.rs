use std::ffi::c_uint;

pub const LIBCAMERA_CONTROL_TYPE_NONE: c_uint = 0;
pub const LIBCAMERA_CONTROL_TYPE_BOOL: c_uint = 1;
pub const LIBCAMERA_CONTROL_TYPE_BYTE: c_uint = 2;
pub const LIBCAMERA_CONTROL_TYPE_UINT16: c_uint = 3;
pub const LIBCAMERA_CONTROL_TYPE_UINT32: c_uint = 4;
pub const LIBCAMERA_CONTROL_TYPE_INT32: c_uint = 5;
pub const LIBCAMERA_CONTROL_TYPE_INT64: c_uint = 6;
pub const LIBCAMERA_CONTROL_TYPE_FLOAT: c_uint = 7;
pub const LIBCAMERA_CONTROL_TYPE_STRING: c_uint = 8;
pub const LIBCAMERA_CONTROL_TYPE_RECTANGLE: c_uint = 9;
pub const LIBCAMERA_CONTROL_TYPE_SIZE: c_uint = 10;
pub const LIBCAMERA_CONTROL_TYPE_POINT: c_uint = 11;
