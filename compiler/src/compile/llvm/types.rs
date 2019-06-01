use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub unsafe fn token_type() -> LLVMTypeRef {
    LLVMTokenTypeInContext(LLVMGetGlobalContext())
}

pub unsafe fn generic_pointer_type() -> LLVMTypeRef {
    pointer_type(i8_type())
}

pub unsafe fn coroutine_handle_type() -> LLVMTypeRef {
    pointer_type(i8_type())
}

pub unsafe fn i64_type() -> LLVMTypeRef {
    int_type(64)
}

pub unsafe fn i32_type() -> LLVMTypeRef {
    int_type(32)
}

pub unsafe fn i8_type() -> LLVMTypeRef {
    int_type(8)
}

pub unsafe fn i1_type() -> LLVMTypeRef {
    int_type(1)
}

pub unsafe fn int_type(bits: u32) -> LLVMTypeRef {
    LLVMIntType(bits)
}

pub unsafe fn double_type() -> LLVMTypeRef {
    LLVMDoubleType()
}

pub unsafe fn pointer_type(internal: LLVMTypeRef) -> LLVMTypeRef {
    LLVMPointerType(internal, 0)
}

pub unsafe fn function_type(result: LLVMTypeRef, arguments: &mut [LLVMTypeRef]) -> LLVMTypeRef {
    LLVMFunctionType(result, arguments.as_mut_ptr(), arguments.len() as u32, 0)
}
