use crate::{
    app::tools::{Tool, ToolProvider},
    tools::echo::EchoModule,
};

pub struct InternalToolProvider;

#[async_trait::async_trait]
impl ToolProvider for InternalToolProvider {
    async fn load_tools(&self) -> Vec<Box<dyn Tool>> {
        vec![Box::new(EchoModule)]
    }
}
