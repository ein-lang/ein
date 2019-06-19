use super::type_kind::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::os::raw::c_uint;

#[derive(Copy, Clone, Debug)]
pub struct Type {
    internal: LLVMTypeRef,
}

impl Type {
    pub(super) fn new(internal: LLVMTypeRef) -> Self {
        Self { internal }
    }

    pub unsafe fn kind(&self) -> TypeKind {
        LLVMGetTypeKind(self.into()).into()
    }

    pub unsafe fn element(&self) -> Type {
        LLVMGetElementType(self.into()).into()
    }

    pub unsafe fn struct_elements(&self) -> Vec<Type> {
        let mut elements = (0..(LLVMCountStructElementTypes(self.into()) as usize))
            .map(|_| std::mem::uninitialized())
            .collect::<Vec<LLVMTypeRef>>();

        LLVMGetStructElementTypes(self.into(), elements.as_mut_ptr());

        elements
            .iter()
            .map(|type_| (*type_).into())
            .collect::<Vec<_>>()
    }

    #[allow(dead_code)]
    pub unsafe fn dump(&self) {
        LLVMDumpType(self.into())
    }

    pub unsafe fn token() -> Self {
        LLVMTokenTypeInContext(LLVMGetGlobalContext()).into()
    }

    pub unsafe fn generic_pointer() -> Self {
        Self::pointer(Self::i8())
    }

    pub unsafe fn i64() -> Self {
        Self::int(64)
    }

    pub unsafe fn i32() -> Self {
        Self::int(32)
    }

    pub unsafe fn i8() -> Self {
        Self::int(8)
    }

    pub unsafe fn i1() -> Self {
        Self::int(1)
    }

    pub unsafe fn int(bits: c_uint) -> Self {
        LLVMIntType(bits).into()
    }

    pub unsafe fn double() -> Self {
        LLVMDoubleType().into()
    }

    pub unsafe fn pointer(content: Self) -> Self {
        LLVMPointerType(content.into(), 0).into()
    }

    pub unsafe fn function(result: Self, arguments: &[Self]) -> Self {
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
        .into()
    }

    pub unsafe fn void() -> Type {
        LLVMVoidType().into()
    }

    pub unsafe fn struct_(elements: &[Self]) -> Type {
        LLVMStructType(
            elements
                .iter()
                .map(|type_| type_.into())
                .collect::<Vec<LLVMTypeRef>>()
                .as_mut_ptr(),
            elements.len() as c_uint,
            0,
        )
        .into()
    }
}

impl From<LLVMTypeRef> for Type {
    fn from(type_ref: LLVMTypeRef) -> Self {
        Self::new(type_ref)
    }
}

impl From<Type> for LLVMTypeRef {
    fn from(type_: Type) -> Self {
        type_.internal
    }
}

impl From<&Type> for LLVMTypeRef {
    fn from(type_: &Type) -> Self {
        type_.internal
    }
}
