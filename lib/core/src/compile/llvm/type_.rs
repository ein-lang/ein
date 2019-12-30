use super::type_kind::*;
use super::utilities::*;
use super::value::Value;
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

    #[allow(dead_code)]
    pub fn dump(self) {
        unsafe { LLVMDumpType(self.into()) }
    }

    pub fn token() -> Self {
        unsafe { LLVMTokenTypeInContext(LLVMGetGlobalContext()) }.into()
    }

    pub fn generic_pointer() -> Self {
        Self::pointer(Self::i8())
    }

    pub fn i64() -> Self {
        Self::int(64)
    }

    pub fn i32() -> Self {
        Self::int(32)
    }

    pub fn i8() -> Self {
        Self::int(8)
    }

    pub fn i1() -> Self {
        Self::int(1)
    }

    pub fn int(bits: c_uint) -> Self {
        unsafe { LLVMIntType(bits) }.into()
    }

    pub fn double() -> Self {
        unsafe { LLVMDoubleType() }.into()
    }

    pub fn pointer(content: Self) -> Self {
        unsafe { LLVMPointerType(content.into(), 0) }.into()
    }

    pub fn function(result: Self, arguments: &[Self]) -> Self {
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

    pub fn void() -> Type {
        unsafe { LLVMVoidType() }.into()
    }

    pub fn struct_(elements: &[Self]) -> Type {
        unsafe {
            LLVMStructType(
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

    pub fn struct_create_named(name: &str) -> Type {
        unsafe { LLVMStructCreateNamed(LLVMGetGlobalContext(), c_string(name).as_ptr()) }.into()
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
