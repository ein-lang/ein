use super::basic_block::*;
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

    #[allow(dead_code)]
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

    pub fn build_malloc(&self, size: Value) -> Value {
        self.build_call_with_name("malloc", &[size])
    }
}
