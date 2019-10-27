use super::type_::*;
use super::utilities::*;
use super::value::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

#[derive(Clone, Copy, Debug)]
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

    pub fn add_function(self, name: &str, function_type: Type) -> Value {
        unsafe { LLVMAddFunction(self.internal, c_string(name).as_ptr(), function_type.into()) }
            .into()
    }

    pub fn add_global(self, name: &str, type_: Type) -> Value {
        unsafe { LLVMAddGlobal(self.internal, type_.into(), c_string(name).as_ptr()) }.into()
    }

    pub fn declare_function(self, name: &str, return_type: Type, arguments: &[Type]) {
        self.add_function(name, Type::function(return_type, arguments));
    }

    pub fn declare_intrinsics(self) {
        self.declare_function(
            "llvm.coro.id",
            Type::token(),
            &[
                Type::i32(),
                Type::generic_pointer(),
                Type::generic_pointer(),
                Type::generic_pointer(),
            ],
        );

        self.declare_function("llvm.coro.size.i32", Type::i32(), &[]);
        self.declare_function("llvm.coro.size.i64", Type::i64(), &[]);

        self.declare_function(
            "llvm.coro.begin",
            Type::generic_pointer(),
            &[Type::token(), Type::generic_pointer()],
        );
        self.declare_function(
            "llvm.coro.end",
            Type::i1(),
            &[Type::generic_pointer(), Type::i1()],
        );
        self.declare_function(
            "llvm.coro.suspend",
            Type::i8(),
            &[Type::token(), Type::i1()],
        );
        self.declare_function(
            "llvm.coro.free",
            Type::generic_pointer(),
            &[Type::token(), Type::generic_pointer()],
        );

        self.declare_function("llvm.coro.done", Type::i1(), &[Type::generic_pointer()]);
        self.declare_function(
            "llvm.coro.promise",
            Type::generic_pointer(),
            &[Type::generic_pointer(), Type::i32(), Type::i1()],
        );
        self.declare_function("llvm.coro.resume", Type::void(), &[Type::generic_pointer()]);

        self.declare_function("malloc", Type::generic_pointer(), &[Type::i64()]);
        self.declare_function("free", Type::void(), &[Type::generic_pointer()]);
    }

    #[allow(dead_code)]
    pub fn dump(self) {
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
