mod data_type_wrapper;

use types::c_void;
use class::gc::GC;

pub use self::data_type_wrapper::{DataTypeWrapper, WrappableData};

pub extern "C" fn free<T: Sized + WrappableData>(data: *mut c_void) {
    // Memory is freed when the box goes out of the scope
    let object = unsafe { Box::from_raw(data as *mut T) };
    GC::adjust_memory_usage(-(object.data_size() as isize));
}
