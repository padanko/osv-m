pub mod runtime;

use rhai;
use once_cell::sync::Lazy;

pub struct RhaiExtRuntime {
    pub rhai_: rhai::Engine,
}

const RHAI_INSTANCE: Lazy<RhaiExtRuntime> = Lazy::new(|| {
    
    let mut rhai_engine = rhai::Engine::new();

    rhai_engine.register_fn("epoch_time", crate::utils::random_id::epoch_time);
    rhai_engine.register_fn("random_integer", |x: i64| {
        crate::utils::random_id::random_integer(x.min(1) as usize)
    });

    rhai_engine.register_fn("random_topicid", crate::utils::random_id::generate_topic_id);
    
    RhaiExtRuntime {
        rhai_: rhai_engine
    }
});