use super::type_kind::*;
use super::value::Value;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::os::raw::c_uint;

#[derive(Copy, Clone, Debug)]
pub struct Type {
    internal: LLVMTypeRef,
}

impl Type {
    pub fn kind(self) -> TypeKind {
        unsafe { LLVMGetTypeKind(self.into()) }.into()
    }

    pub fn element(self) -> Type {
        assert_eq!(self.kind(), TypeKind::Pointer);

        unsafe { LLVMGetElementType(self.into()) }.into()
    }

    pub fn struct_elements(self) -> Vec<Type> {
        assert_eq!(self.kind(), TypeKind::Struct);

        let mut elements = (0..(unsafe { LLVMCountStructElementTypes(self.into()) } as usize))
            .map(|_| unsafe { std::mem::MaybeUninit::uninit().assume_init() })
            .collect::<Vec<LLVMTypeRef>>();

        unsafe { LLVMGetStructElementTypes(self.into(), elements.as_mut_ptr()) };

        elements
            .iter()
            .map(|type_| (*type_).into())
            .collect::<Vec<_>>()
    }

    pub fn size(self) -> Value {
        unsafe { LLVMSizeOf(self.into()) }.into()
    }

    pub fn struct_set_body(self, elements: &[Self]) {
        assert_eq!(self.kind(), TypeKind::Struct);

        unsafe {
            LLVMStructSetBody(
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
    }
}

impl From<LLVMTypeRef> for Type {
    fn from(internal: LLVMTypeRef) -> Self {
        Self { internal }
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
