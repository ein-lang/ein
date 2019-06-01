use super::module::Module;
use llvm_sys::analysis::*;
use llvm_sys::prelude::*;

pub unsafe fn verify_function(function: LLVMValueRef) {
    LLVMVerifyFunction(function, LLVMVerifierFailureAction::LLVMAbortProcessAction);
}

pub unsafe fn verify_module(module: &Module) {
    LLVMVerifyModule(
        module.internal(),
        LLVMVerifierFailureAction::LLVMAbortProcessAction,
        std::ptr::null_mut(),
    );
}
