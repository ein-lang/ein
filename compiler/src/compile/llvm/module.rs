use super::prelude::*;
use super::utilities::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub struct Module {
    module: LLVMModuleRef,
}

impl Module {
    pub unsafe fn new(name: &str) -> Self {
        Self {
            module: LLVMModuleCreateWithName(c_string(name).as_ptr()),
        }
    }

    pub unsafe fn add_function(&self, name: &str, function_type: Type) -> Value {
        LLVMAddFunction(self.module, c_string(name).as_ptr(), function_type)
    }

    pub fn internal(&self) -> LLVMModuleRef {
        self.module
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            unsafe { std::ffi::CString::from_raw(LLVMPrintModuleToString(self.module)) }
                .to_str()
                .unwrap()
        )
    }
}
