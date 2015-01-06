#![feature(unsafe_destructor)]
extern crate "wren-sys" as raw;
extern crate libc;

use std::ops::{Drop};
use std::c_str::ToCStr;

struct VM {
    _vm: *mut raw::WrenVM
}

struct Config {
    raw: *mut raw::WrenConfiguration
}

enum Error {
    CompileError(String),
    RuntimeError(String),
    UnknownError(String),
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
                heapGrowthPercent: 0i32
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
                                            source_path.to_c_str().as_ptr(),
                                            source.to_c_str().as_ptr());
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
