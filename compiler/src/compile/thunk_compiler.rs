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
            let frame = builder.build_call_with_name("malloc", &mut [size]);
            let handle = builder.build_coro_begin(id, frame);

            builder.position_at_end(builder.append_basic_block(function, "suspend"));
            builder.build_coro_end(handle);
            builder.build_ret(handle);

            llvm::verify_function(function);

            Ok(function)
        }
    }
}
