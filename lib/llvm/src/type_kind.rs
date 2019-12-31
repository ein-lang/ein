use llvm_sys::LLVMTypeKind::{self, *};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeKind {
    Array,
    Double,
    Fp128,
    Float,
    Function,
    Half,
    Integer,
    Label,
    Metadata,
    PpcFp128,
    Pointer,
    Struct,
    Token,
    Vector,
    Void,
    X86Fp80,
    X86Mmx,
}

impl From<LLVMTypeKind> for TypeKind {
    fn from(kind: LLVMTypeKind) -> Self {
        match kind {
            LLVMArrayTypeKind => TypeKind::Array,
            LLVMDoubleTypeKind => TypeKind::Double,
            LLVMFP128TypeKind => TypeKind::Fp128,
            LLVMFloatTypeKind => TypeKind::Float,
            LLVMFunctionTypeKind => TypeKind::Function,
            LLVMHalfTypeKind => TypeKind::Half,
            LLVMIntegerTypeKind => TypeKind::Integer,
            LLVMLabelTypeKind => TypeKind::Label,
            LLVMMetadataTypeKind => TypeKind::Metadata,
            LLVMPPC_FP128TypeKind => TypeKind::PpcFp128,
            LLVMPointerTypeKind => TypeKind::Pointer,
            LLVMStructTypeKind => TypeKind::Struct,
            LLVMTokenTypeKind => TypeKind::Token,
            LLVMVectorTypeKind => TypeKind::Vector,
            LLVMVoidTypeKind => TypeKind::Void,
            LLVMX86_FP80TypeKind => TypeKind::X86Fp80,
            LLVMX86_MMXTypeKind => TypeKind::X86Mmx,
        }
    }
}

impl From<TypeKind> for LLVMTypeKind {
    fn from(kind: TypeKind) -> Self {
        match kind {
            TypeKind::Array => LLVMArrayTypeKind,
            TypeKind::Double => LLVMDoubleTypeKind,
            TypeKind::Fp128 => LLVMFP128TypeKind,
            TypeKind::Float => LLVMFloatTypeKind,
            TypeKind::Function => LLVMFunctionTypeKind,
            TypeKind::Half => LLVMHalfTypeKind,
            TypeKind::Integer => LLVMIntegerTypeKind,
            TypeKind::Label => LLVMLabelTypeKind,
            TypeKind::Metadata => LLVMMetadataTypeKind,
            TypeKind::PpcFp128 => LLVMPPC_FP128TypeKind,
            TypeKind::Pointer => LLVMPointerTypeKind,
            TypeKind::Struct => LLVMStructTypeKind,
            TypeKind::Token => LLVMTokenTypeKind,
            TypeKind::Vector => LLVMVectorTypeKind,
            TypeKind::Void => LLVMVoidTypeKind,
            TypeKind::X86Fp80 => LLVMX86_FP80TypeKind,
            TypeKind::X86Mmx => LLVMX86_MMXTypeKind,
        }
    }
}
