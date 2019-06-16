use super::module::Module;
use super::value::*;
use llvm_sys::analysis::*;

pub unsafe fn verify_function(function: Value) {
    LLVMVerifyFunction(
        function.into(),
        LLVMVerifierFailureAction::LLVMAbortProcessAction,
    );
}

pub unsafe fn verify_module(module: &Module) {
    LLVMVerifyModule(
        module.internal(),
        LLVMVerifierFailureAction::LLVMAbortProcessAction,
        std::ptr::null_mut(),
    );
}
