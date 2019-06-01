use super::prelude::*;
use super::types::*;
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

    pub fn internal(&self) -> LLVMModuleRef {
        self.module
    }

    pub unsafe fn add_function(&self, name: &str, function_type: Type) -> Value {
        LLVMAddFunction(self.module, c_string(name).as_ptr(), function_type)
    }

    pub unsafe fn declare_function(&self, name: &str, return_type: Type, arguments: &mut [Type]) {
        self.add_function(name, function_type(return_type, arguments));
    }

    pub unsafe fn declare_intrinsics(&self) {
        self.declare_function(
            "llvm.coro.id",
            token_type(),
            &mut [
                i32_type(),
                generic_pointer_type(),
                generic_pointer_type(),
                generic_pointer_type(),
            ],
        );

        self.declare_function("llvm.coro.size.i32", i32_type(), &mut []);
        self.declare_function("llvm.coro.size.i64", i64_type(), &mut []);

        self.declare_function(
            "llvm.coro.begin",
            generic_pointer_type(),
            &mut [token_type(), generic_pointer_type()],
        );
        self.declare_function(
            "llvm.coro.end",
            i1_type(),
            &mut [generic_pointer_type(), i1_type()],
        );
        self.declare_function(
            "llvm.coro.suspend",
            i8_type(),
            &mut [token_type(), i1_type()],
        );
        self.declare_function(
            "llvm.coro.free",
            generic_pointer_type(),
            &mut [token_type(), generic_pointer_type()],
        );

        self.declare_function("malloc", generic_pointer_type(), &mut [i32_type()]);
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
