pub struct CortexConfig {
    pub context_window_size: usize,
    pub memory_top_k: usize,
    pub memory_threshold: f64,
    pub memory_importance_threshold: f32,

    pub openai_api_key: String,
}

impl CortexConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            context_window_size: dotenvy::var("CTX_WINDOW")
                .unwrap_or("12".into())
                .parse()
                .unwrap(),

            memory_top_k: dotenvy::var("MEM_TOP_K")
                .unwrap_or("5".into())
                .parse()
                .unwrap(),

            memory_threshold: dotenvy::var("MEM_THRESHOLD")
                .unwrap_or("0.3".into())
                .parse()
                .unwrap(),

            memory_importance_threshold: dotenvy::var("MEM_IMPORTANCE")
                .unwrap_or("0.6".into())
                .parse()
                .unwrap(),

            openai_api_key: dotenvy::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is required"),
        }
    }
}
