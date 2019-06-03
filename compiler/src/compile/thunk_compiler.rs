use super::error::CompileError;
use super::llvm;
use super::utilities::evaluate_thunk;

pub struct ThunkCompiler<'a> {
    module: &'a llvm::Module,
}

impl<'a> ThunkCompiler<'a> {
    pub fn new(module: &'a llvm::Module) -> Self {
        Self { module }
    }

    pub fn compile(&mut self, name: &str) -> Result<llvm::Value, CompileError> {
        unsafe {
            let function = self.module.add_function(
                name,
                llvm::function_type(
                    llvm::generic_pointer_type(),
                    &mut [llvm::generic_pointer_type(), llvm::generic_pointer_type()],
                ),
            );

            let builder = llvm::Builder::new(function);
            builder.append_basic_block("entry");
            builder.position_at_end(builder.append_basic_block("entry"));

            let promise = builder.build_alloca(llvm::double_type());
            let id = builder.build_coro_id(promise);
            let size = builder.build_coro_size_i32();
            let frame = builder.build_malloc(size);
            let handle = builder.build_coro_begin(id, frame);
            let main_block = builder.append_basic_block("eval");
            builder.build_br(main_block);

            builder.position_at_end(main_block);
            builder.build_store(
                promise,
                builder.build_fadd(
                    evaluate_thunk(&builder, llvm::get_param(function, 0)),
                    evaluate_thunk(&builder, llvm::get_param(function, 1)),
                ),
            );

            let suspend_block = builder.append_basic_block("suspend");
            builder.position_at_end(suspend_block);
            builder.build_coro_end(handle);
            builder.build_ret(handle);

            builder.position_at_end(builder.append_basic_block("cleanup"));
            builder.build_free(builder.build_coro_free(id, handle));
            builder.build_br(suspend_block);

            llvm::verify_function(function);

            Ok(function)
        }
    }
}
