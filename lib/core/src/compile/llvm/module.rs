use super::type_::*;
use super::utilities::*;
use super::value::*;
use llvm_sys::core::*;
use llvm_sys::linker::*;
use llvm_sys::prelude::*;

pub struct Module {
    internal: LLVMModuleRef,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            internal: unsafe { LLVMModuleCreateWithName(c_string(name).as_ptr()) },
        }
    }

    pub(super) fn internal(&self) -> LLVMModuleRef {
        self.internal
    }

    pub fn add_function(&mut self, name: &str, function_type: Type) -> Value {
        unsafe { LLVMAddFunction(self.internal, c_string(name).as_ptr(), function_type.into()) }
            .into()
    }

    pub fn add_global(&mut self, name: &str, type_: Type) -> Value {
        unsafe { LLVMAddGlobal(self.internal, type_.into(), c_string(name).as_ptr()) }.into()
    }

    pub fn declare_function(&mut self, name: &str, return_type: Type, arguments: &[Type]) {
        self.add_function(name, Type::function(return_type, arguments));
    }

    pub fn declare_intrinsics(&mut self) {
        self.declare_function("malloc", Type::generic_pointer(), &[Type::i64()]);
    }

    pub fn link(&mut self, other: Self) {
        unsafe {
            LLVMLinkModules2(self.internal(), other.internal());
        }
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.internal) }
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            unsafe { std::ffi::CString::from_raw(LLVMPrintModuleToString(self.internal)) }
                .to_str()
                .unwrap()
        )
    }
}

impl From<LLVMModuleRef> for Module {
    fn from(internal: LLVMModuleRef) -> Self {
        Self { internal }
    }
}
