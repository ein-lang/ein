use super::module::Module;
use super::value::*;
use llvm_sys::analysis::*;

pub fn verify_function(function: Value) {
    unsafe {
        LLVMVerifyFunction(
            function.into(),
            LLVMVerifierFailureAction::LLVMAbortProcessAction,
        )
    };
}

pub fn verify_module(module: Module) {
    unsafe {
        LLVMVerifyModule(
            module.internal(),
            LLVMVerifierFailureAction::LLVMAbortProcessAction,
            std::ptr::null_mut(),
        )
    };
}
