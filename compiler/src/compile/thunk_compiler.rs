use super::error::CompileError;
use super::llvm;

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
                    llvm::coroutine_handle_type(),
                    &mut [llvm::coroutine_handle_type(), llvm::coroutine_handle_type()],
                ),
            );

            let builder = llvm::Builder::new(&self.module);
            builder.append_basic_block(function, "entry");
            builder.position_at_end(builder.append_basic_block(function, "entry"));

            let id = builder.build_coro_id();
            let size = builder.build_coro_size_i32();
            let frame = builder.build_malloc(size);
            let handle = builder.build_coro_begin(id, frame);

            let suspend_block = builder.append_basic_block(function, "suspend");
            builder.position_at_end(suspend_block);
            builder.build_coro_end(handle);
            builder.build_ret(handle);

            builder.position_at_end(builder.append_basic_block(function, "cleanup"));
            builder.build_free(builder.build_coro_free(id, handle));
            builder.build_br(suspend_block);

            llvm::verify_function(function);

            Ok(function)
        }
    }
}
