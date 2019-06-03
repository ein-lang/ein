use super::llvm;

pub unsafe fn evaluate_thunk(builder: &llvm::Builder, handle: llvm::Value) -> llvm::Value {
    let loop_block = builder.append_basic_block("loop");
    let resume_block = builder.append_basic_block("resume");
    let load_block = builder.append_basic_block("load");

    builder.build_br(loop_block);

    builder.position_at_end(loop_block);
    builder.build_cond_br(builder.build_coro_done(handle), load_block, resume_block);

    builder.position_at_end(resume_block);
    builder.build_coro_resume(handle);
    builder.build_br(loop_block);

    builder.position_at_end(load_block);
    let promise = builder.build_coro_promise(handle);
    builder.build_load(builder.build_bit_cast(promise, llvm::double_type()))
}
