use super::basic_block::*;
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
    pub fn new(function: Value) -> Builder {
        Builder {
            module: unsafe { LLVMGetGlobalParent(function.into()) },
            function: function.into(),
            builder: unsafe { LLVMCreateBuilder() },
        }
    }

    pub fn build_alloca(&self, type_: Type) -> Value {
        unsafe { LLVMBuildAlloca(self.builder, type_.into(), c_string("").as_ptr()) }.into()
    }

    pub fn build_load(&self, pointer: Value) -> Value {
        unsafe { LLVMBuildLoad(self.builder, pointer.into(), c_string("").as_ptr()) }.into()
    }

    pub fn build_store(&self, value: Value, pointer: Value) {
        unsafe { LLVMBuildStore(self.builder, value.into(), pointer.into()) };
    }

    pub fn build_br(&self, block: BasicBlock) {
        unsafe { LLVMBuildBr(self.builder, block.into()) };
    }

    pub fn build_cond_br(&self, condition: Value, then: BasicBlock, els: BasicBlock) {
        unsafe { LLVMBuildCondBr(self.builder, condition.into(), then.into(), els.into()) };
    }

    pub fn build_phi(
        &self,
        type_: Type,
        incoming_values: &[Value],
        incoming_blocks: &[BasicBlock],
    ) {
        unsafe {
            let phi = LLVMBuildPhi(self.builder, type_.into(), c_string("").as_ptr());

            LLVMAddIncoming(
                phi,
                incoming_values
                    .iter()
                    .map(|value| value.into())
                    .collect::<Vec<LLVMValueRef>>()
                    .as_mut_ptr(),
                incoming_blocks
                    .iter()
                    .map(|block| block.into())
                    .collect::<Vec<LLVMBasicBlockRef>>()
                    .as_mut_ptr(),
                incoming_values.len() as u32,
            )
        };
    }

    pub fn build_bit_cast(&self, value: Value, type_: Type) -> Value {
        unsafe {
            LLVMBuildBitCast(
                self.builder,
                value.into(),
                type_.into(),
                c_string("").as_ptr(),
            )
        }
        .into()
    }

    pub fn build_gep(&self, pointer: Value, indices: &[Value]) -> Value {
        unsafe {
            LLVMBuildGEP(
                self.builder,
                pointer.into(),
                indices
                    .iter()
                    .map(|value| value.into())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                indices.len() as u32,
                c_string("").as_ptr(),
            )
        }
        .into()
    }

    pub fn build_call(&self, function: Value, arguments: &[Value]) -> Value {
        unsafe {
            LLVMBuildCall(
                self.builder,
                function.into(),
                arguments
                    .iter()
                    .map(|value| value.into())
                    .collect::<Vec<LLVMValueRef>>()
                    .as_mut_ptr(),
                arguments.len() as u32,
                c_string("").as_ptr(),
            )
        }
        .into()
    }

    pub fn build_call_with_name(&self, function_name: &str, arguments: &[Value]) -> Value {
        self.build_call(
            unsafe { LLVMGetNamedFunction(self.module, c_string(function_name).as_ptr()) }.into(),
            arguments,
        )
    }

    pub fn build_ret(&self, value: Value) {
        unsafe { LLVMBuildRet(self.builder, value.into()) };
    }

    pub fn build_ret_void(&self) {
        unsafe { LLVMBuildRetVoid(self.builder) };
    }

    pub fn build_fadd(&self, lhs: Value, rhs: Value) -> Value {
        unsafe { LLVMBuildFAdd(self.builder, lhs.into(), rhs.into(), c_string("").as_ptr()) }.into()
    }

    pub fn build_fsub(&self, lhs: Value, rhs: Value) -> Value {
        unsafe { LLVMBuildFSub(self.builder, lhs.into(), rhs.into(), c_string("").as_ptr()) }.into()
    }

    pub fn build_fmul(&self, lhs: Value, rhs: Value) -> Value {
        unsafe { LLVMBuildFMul(self.builder, lhs.into(), rhs.into(), c_string("").as_ptr()) }.into()
    }

    pub fn build_fdiv(&self, lhs: Value, rhs: Value) -> Value {
        unsafe { LLVMBuildFDiv(self.builder, lhs.into(), rhs.into(), c_string("").as_ptr()) }.into()
    }

    pub fn append_basic_block(&self, name: &str) -> BasicBlock {
        unsafe { LLVMAppendBasicBlock(self.function, c_string(name).as_ptr()) }.into()
    }

    pub fn position_at_end(&self, block: BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.builder, block.into()) };
    }

    pub fn build_coro_id(&self, promise: Value) -> Value {
        self.build_call_with_name(
            "llvm.coro.id",
            &[
                const_int(Type::i32(), 0),
                self.build_bit_cast(promise, Type::generic_pointer()),
                const_null(Type::generic_pointer()),
                const_null(Type::generic_pointer()),
            ],
        )
    }

    pub fn build_coro_size_i32(&self) -> Value {
        self.build_call_with_name("llvm.coro.size.i32", &[])
    }

    pub fn build_coro_begin(&self, id: Value, frame: Value) -> Value {
        self.build_call_with_name("llvm.coro.begin", &[id, frame])
    }

    pub fn build_coro_end(&self, handle: Value) {
        self.build_call_with_name("llvm.coro.end", &[handle, const_int(Type::i1(), 0)]);
    }

    pub fn build_coro_free(&self, id: Value, handle: Value) -> Value {
        self.build_call_with_name("llvm.coro.free", &[id, handle])
    }

    pub fn build_coro_resume(&self, handle: Value) {
        self.build_call_with_name("llvm.coro.resume", &[handle]);
    }

    pub fn build_coro_done(&self, handle: Value) -> Value {
        self.build_call_with_name("llvm.coro.done", &[handle])
    }

    pub fn build_coro_promise(&self, handle: Value) -> Value {
        self.build_call_with_name(
            "llvm.coro.promise",
            &[handle, const_int(Type::i32(), 8), const_int(Type::i1(), 0)],
        )
    }

    pub fn build_malloc(&self, size: Value) -> Value {
        self.build_call_with_name("malloc", &[size])
    }

    pub fn build_free(&self, pointer: Value) {
        self.build_call_with_name("free", &[pointer]);
    }
}
