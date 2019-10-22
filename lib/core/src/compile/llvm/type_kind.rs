use llvm_sys::LLVMTypeKind::{self, *};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeKind {
    Array,
    Double,
    FP128,
    Float,
    Function,
    Half,
    Integer,
    Label,
    Metadata,
    Pointer,
    Struct,
    Token,
    Vector,
    Void,
}

impl From<LLVMTypeKind> for TypeKind {
    fn from(kind: LLVMTypeKind) -> Self {
        match kind {
            LLVMArrayTypeKind => TypeKind::Array,
            LLVMDoubleTypeKind => TypeKind::Double,
            LLVMFP128TypeKind => TypeKind::FP128,
            LLVMFloatTypeKind => TypeKind::Float,
            LLVMFunctionTypeKind => TypeKind::Function,
            LLVMHalfTypeKind => TypeKind::Half,
            LLVMIntegerTypeKind => TypeKind::Integer,
            LLVMLabelTypeKind => TypeKind::Label,
            LLVMMetadataTypeKind => TypeKind::Metadata,
            LLVMPPC_FP128TypeKind => unimplemented!(),
            LLVMPointerTypeKind => TypeKind::Pointer,
            LLVMStructTypeKind => TypeKind::Struct,
            LLVMTokenTypeKind => TypeKind::Token,
            LLVMVectorTypeKind => TypeKind::Vector,
            LLVMVoidTypeKind => TypeKind::Void,
            LLVMX86_FP80TypeKind => unimplemented!(),
            LLVMX86_MMXTypeKind => unimplemented!(),
        }
    }
}

impl From<TypeKind> for LLVMTypeKind {
    fn from(kind: TypeKind) -> Self {
        match kind {
            TypeKind::Array => LLVMArrayTypeKind,
            TypeKind::Double => LLVMDoubleTypeKind,
            TypeKind::FP128 => LLVMFP128TypeKind,
            TypeKind::Float => LLVMFloatTypeKind,
            TypeKind::Function => LLVMFunctionTypeKind,
            TypeKind::Half => LLVMHalfTypeKind,
            TypeKind::Integer => LLVMIntegerTypeKind,
            TypeKind::Label => LLVMLabelTypeKind,
            TypeKind::Metadata => LLVMMetadataTypeKind,
            TypeKind::Pointer => LLVMPointerTypeKind,
            TypeKind::Struct => LLVMStructTypeKind,
            TypeKind::Token => LLVMTokenTypeKind,
            TypeKind::Vector => LLVMVectorTypeKind,
            TypeKind::Void => LLVMVoidTypeKind,
        }
    }
}
