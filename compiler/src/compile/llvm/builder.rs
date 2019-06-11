use super::constants::*;
use super::type_::*;
use super::utilities::c_string;
use super::value::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub struct Builder {
    module: LLVMModuleRef,
    function: LLVMValueRef,
    builder: LLVMBuilderRef,
}

impl Builder {
    pub unsafe fn new(function: LLVMValueRef) -> Builder {
        Builder {
            module: LLVMGetGlobalParent(function),
            function,
            builder: LLVMCreateBuilder(),
        }
    }

    pub unsafe fn build_alloca(&self, type_: Type) -> Value {
        LLVMBuildAlloca(self.builder, type_.into(), c_string("").as_ptr()).into()
    }

    pub unsafe fn build_load(&self, pointer: Value) -> Value {
        LLVMBuildLoad(self.builder, pointer.into(), c_string("").as_ptr()).into()
    }

    pub unsafe fn build_store(&self, pointer: Value, value: Value) {
        LLVMBuildStore(self.builder, pointer.into(), value.into());
    }

    pub unsafe fn build_br(&self, label: LLVMBasicBlockRef) {
        LLVMBuildBr(self.builder, label);
    }

    pub unsafe fn build_cond_br(
        &self,
        condition: Value,
        then: LLVMBasicBlockRef,
        els: LLVMBasicBlockRef,
    ) {
        LLVMBuildCondBr(self.builder, condition.into(), then, els);
    }

    pub unsafe fn build_bit_cast(&self, value: Value, type_: Type) -> Value {
        LLVMBuildBitCast(
            self.builder,
            value.into(),
            type_.into(),
            c_string("").as_ptr(),
        )
        .into()
    }

    pub unsafe fn build_call(&self, function: Value, arguments: &mut [Value]) -> Value {
        LLVMBuildCall(
            self.builder,
            function.into(),
            arguments
                .iter()
                .map(|value| value.into())
                .collect::<Vec<LLVMValueRef>>()
                .as_mut_ptr(),
            arguments.len() as u32,
            std::ptr::null(),
        )
        .into()
    }

    pub unsafe fn build_call_with_name(
        &self,
        function_name: &str,
        arguments: &mut [Value],
    ) -> Value {
        self.build_call(
            LLVMGetNamedFunction(self.module, c_string(function_name).as_ptr()).into(),
            arguments,
        )
    }

    pub unsafe fn build_ret(&self, value: Value) {
        LLVMBuildRet(self.builder, value.into());
    }

    pub unsafe fn build_fadd(&self, lhs: Value, rhs: Value) -> Value {
        LLVMBuildFAdd(self.builder, lhs.into(), rhs.into(), c_string("").as_ptr()).into()
    }

    pub unsafe fn build_fsub(&self, lhs: Value, rhs: Value) -> Value {
        LLVMBuildFSub(self.builder, lhs.into(), rhs.into(), c_string("").as_ptr()).into()
    }

    pub unsafe fn build_fmul(&self, lhs: Value, rhs: Value) -> Value {
        LLVMBuildFMul(self.builder, lhs.into(), rhs.into(), c_string("").as_ptr()).into()
    }

    pub unsafe fn build_fdiv(&self, lhs: Value, rhs: Value) -> Value {
        LLVMBuildFDiv(self.builder, lhs.into(), rhs.into(), c_string("").as_ptr()).into()
    }

    pub unsafe fn append_basic_block(&self, name: &str) -> LLVMBasicBlockRef {
        LLVMAppendBasicBlock(self.function, c_string(name).as_ptr())
    }

    pub unsafe fn position_at_end(&self, block: LLVMBasicBlockRef) {
        LLVMPositionBuilderAtEnd(self.builder, block);
    }

    pub unsafe fn build_coro_id(&self, promise: Value) -> Value {
        self.build_call_with_name(
            "llvm.coro.id",
            &mut [
                const_int(Type::i32(), 0),
                self.build_bit_cast(promise, Type::generic_pointer()),
                const_null(Type::generic_pointer()),
                const_null(Type::generic_pointer()),
            ],
        )
    }

    pub unsafe fn build_coro_size_i32(&self) -> Value {
        self.build_call_with_name("llvm.coro.size.i32", &mut [])
    }

    pub unsafe fn build_coro_begin(&self, id: Value, frame: Value) -> Value {
        self.build_call_with_name("llvm.coro.begin", &mut [id, frame])
    }

    pub unsafe fn build_coro_end(&self, handle: Value) {
        self.build_call_with_name("llvm.coro.end", &mut [handle, const_int(Type::i1(), 0)]);
    }

    pub unsafe fn build_coro_free(&self, id: Value, handle: Value) -> Value {
        self.build_call_with_name("llvm.coro.free", &mut [id, handle])
    }

    pub unsafe fn build_coro_resume(&self, handle: Value) {
        self.build_call_with_name("llvm.coro.resume", &mut [handle]);
    }

    pub unsafe fn build_coro_done(&self, handle: Value) -> Value {
        self.build_call_with_name("llvm.coro.done", &mut [handle])
    }

    pub unsafe fn build_coro_promise(&self, handle: Value) -> Value {
        self.build_call_with_name(
            "llvm.coro.promise",
            &mut [handle, const_int(Type::i32(), 8), const_int(Type::i1(), 0)],
        )
    }

    pub unsafe fn build_malloc(&self, size: Value) -> Value {
        self.build_call_with_name("malloc", &mut [size])
    }

    pub unsafe fn build_free(&self, pointer: Value) {
        self.build_call_with_name("free", &mut [pointer]);
    }
}
