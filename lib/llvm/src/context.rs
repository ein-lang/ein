use super::module::Module;
use super::type_::Type;
use super::utilities::*;
use super::value::Value;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::os::raw::c_uint;

pub struct Context {
    internal: LLVMContextRef,
}

impl Context {
    pub fn new() -> Self {
        Self {
            internal: unsafe { LLVMContextCreate() },
        }
    }

    pub fn create_module(&self, name: &str) -> Module {
        unsafe { LLVMModuleCreateWithNameInContext(c_string(name).as_ptr(), self.internal) }.into()
    }

    pub fn const_struct(&self, elements: &[Value]) -> Value {
        unsafe {
            LLVMConstStructInContext(
                self.internal,
                elements
                    .iter()
                    .map(|type_| type_.into())
                    .collect::<Vec<LLVMValueRef>>()
                    .as_mut_ptr(),
                elements.len() as c_uint,
                0,
            )
        }
        .into()
    }

    pub fn generic_pointer_type(&self) -> Type {
        self.pointer_type(self.i8_type())
    }

    pub fn i64_type(&self) -> Type {
        self.int_type(64)
    }

    pub fn i32_type(&self) -> Type {
        self.int_type(32)
    }

    pub fn i8_type(&self) -> Type {
        self.int_type(8)
    }

    pub fn i1_type(&self) -> Type {
        self.int_type(1)
    }

    pub fn int_type(&self, bits: c_uint) -> Type {
        unsafe { LLVMIntTypeInContext(self.internal, bits) }.into()
    }

    pub fn double_type(&self) -> Type {
        unsafe { LLVMDoubleTypeInContext(self.internal) }.into()
    }

    pub fn pointer_type(&self, content: Type) -> Type {
        unsafe { LLVMPointerType(content.into(), 0) }.into()
    }

    pub fn function_type(&self, result: Type, arguments: &[Type]) -> Type {
        unsafe {
            LLVMFunctionType(
                result.into(),
                arguments
                    .iter()
                    .map(|type_| type_.into())
                    .collect::<Vec<LLVMTypeRef>>()
                    .as_mut_ptr(),
                arguments.len() as c_uint,
                0,
            )
        }
        .into()
    }

    pub fn void_type(&self) -> Type {
        unsafe { LLVMVoidTypeInContext(self.internal) }.into()
    }

    pub fn struct_type(&self, elements: &[Type]) -> Type {
        unsafe {
            LLVMStructTypeInContext(
                self.internal,
                elements
                    .iter()
                    .map(|type_| type_.into())
                    .collect::<Vec<LLVMTypeRef>>()
                    .as_mut_ptr(),
                elements.len() as c_uint,
                0,
            )
        }
        .into()
    }
}

impl From<LLVMContextRef> for Context {
    fn from(internal: LLVMContextRef) -> Self {
        Self { internal }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
