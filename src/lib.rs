extern crate wren_sys as raw;
extern crate libc;

use std::ffi::CString;

pub struct VM {
    _vm: *mut raw::WrenVM
}

pub struct Config {
    raw: *mut raw::WrenConfiguration
}

pub enum Error {
    CompileError(String),
    RuntimeError(String),
    UnknownError(String),
}

extern "C" fn default_load_module_fn(vm: *mut raw::WrenVM, name: *const libc::c_char) -> *const libc::c_char {
    CString::new("hello, wren!").unwrap().as_ptr()
}

impl Config {
    pub unsafe fn from_raw(raw: *mut raw::WrenConfiguration) -> Config {
        Config {
            raw: raw,
        }
    }
}

impl std::default::Default for Config {
    fn default() -> Config {
        unsafe {
            Config::from_raw(&mut raw::WrenConfiguration {
                reallocateFn: None,
                initialHeapSize: (1024i32 * 1024i32 * 100i32) as libc::size_t,
                minHeapSize: 0i32 as libc::size_t,
                heapGrowthPercent: 0i32,
                loadModuleFn: default_load_module_fn,
            })
        }
    }
}

impl VM {
    pub fn new(config: Config) -> VM {
        unsafe {
            let config = config.raw;
            VM {
                _vm: raw::wrenNewVM(config)
            }
        }
    }

    pub fn interpret(&self, source_path: &str, source: &str) -> Result<(), Error> {
        unsafe {
            let result = raw::wrenInterpret(self._vm,
                                            CString::new(source_path).unwrap().as_ptr(),
                                            CString::new(source).unwrap().as_ptr());
            match result {
                raw::WREN_RESULT_SUCCESS => Ok(()),
                raw::WREN_RESULT_COMPILE_ERROR => Err(Error::CompileError("".to_string())),
                raw::WREN_RESULT_RUNTIME_ERROR => Err(Error::RuntimeError("".to_string())),
                _ => Err(Error::UnknownError("should have been unreachable".to_string())),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{VM};
    use std::default::Default;
    #[test]
    fn test_interpret() {
        let vm = VM::new(Default::default());
        let source = r#"
class Unicorn {
  hasHorn {
    return true
  }
}"#;
        assert!(vm.interpret("", source).is_ok());
    }
}
