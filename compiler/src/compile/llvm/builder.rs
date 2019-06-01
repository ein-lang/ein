use super::module::Module;
use super::types::*;
use super::utilities::c_string;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub struct Builder {
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
}

impl Builder {
    pub unsafe fn new(module: &Module) -> Builder {
        Builder {
            module: module.internal(),
            builder: LLVMCreateBuilder(),
        }
    }

    pub unsafe fn build_call(
        &self,
        function: LLVMValueRef,
        arguments: &mut [LLVMValueRef],
    ) -> LLVMValueRef {
        LLVMBuildCall(
            self.builder,
            function,
            arguments.as_mut_ptr(),
            arguments.len() as u32,
            std::ptr::null(),
        )
    }

    pub unsafe fn build_call_with_name(
        &self,
        function_name: &str,
        arguments: &mut [LLVMValueRef],
    ) -> LLVMValueRef {
        self.build_call(
            LLVMGetNamedFunction(self.module, c_string(function_name).as_ptr()),
            arguments,
        )
    }

    pub unsafe fn build_ret(&self, value: LLVMValueRef) {
        LLVMBuildRet(self.builder, value);
    }

    pub unsafe fn build_fadd(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        LLVMBuildFAdd(self.builder, lhs, rhs, c_string("").as_ptr())
    }

    pub unsafe fn build_fsub(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        LLVMBuildFSub(self.builder, lhs, rhs, c_string("").as_ptr())
    }

    pub unsafe fn build_fmul(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        LLVMBuildFMul(self.builder, lhs, rhs, c_string("").as_ptr())
    }

    pub unsafe fn build_fdiv(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        LLVMBuildFDiv(self.builder, lhs, rhs, c_string("").as_ptr())
    }

    pub unsafe fn append_basic_block(
        &self,
        function: LLVMValueRef,
        name: &str,
    ) -> LLVMBasicBlockRef {
        LLVMAppendBasicBlock(function, c_string(name).as_ptr())
    }

    pub unsafe fn position_at_end(&self, block: LLVMBasicBlockRef) {
        LLVMPositionBuilderAtEnd(self.builder, block);
    }

    pub unsafe fn build_coro_id(&self) -> LLVMValueRef {
        self.build_call_with_name(
            "llvm.coro.id",
            &mut [
                LLVMConstInt(LLVMIntType(32), 0, 0),
                LLVMConstNull(generic_pointer_type()),
                LLVMConstNull(generic_pointer_type()),
                LLVMConstNull(generic_pointer_type()),
            ],
        )
    }

    pub unsafe fn build_coro_size_i32(&self) -> LLVMValueRef {
        self.build_call_with_name("llvm.coro.size.i32", &mut [])
    }

    pub unsafe fn build_coro_begin(
        &self,
        token: LLVMValueRef,
        frame: LLVMValueRef,
    ) -> LLVMValueRef {
        self.build_call_with_name("llvm.coro.begin", &mut [token, frame])
    }

    pub unsafe fn build_coro_end(&self, handle: LLVMValueRef) {
        self.build_call_with_name(
            "llvm.coro.end",
            &mut [handle, LLVMConstInt(i1_type(), 0, 0)],
        );
    }

    pub unsafe fn malloc(&self, size: LLVMValueRef) -> LLVMValueRef {
        self.build_call_with_name("malloc", &mut [size])
    }

    pub unsafe fn free(&self, pointer: LLVMValueRef) {
        self.build_call_with_name("free", &mut [pointer]);
    }
}
