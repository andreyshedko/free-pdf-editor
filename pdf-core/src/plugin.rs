use crate::event::EventBus;
use std::collections::HashMap;

pub struct PluginContext<'a> {
    pub event_bus: &'a mut EventBus,
    pub registered_tools: &'a mut HashMap<String, ToolDescriptor>,
}

#[derive(Debug, Clone)]
pub struct ToolDescriptor {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
}

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str { "" }
    fn on_load(&self, ctx: &mut PluginContext<'_>);
    fn on_unload(&self) {}
}

#[derive(Default)]
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
    tools: HashMap<String, ToolDescriptor>,
}

impl PluginRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn load(&mut self, plugin: Box<dyn Plugin>, bus: &mut EventBus) {
        let mut ctx = PluginContext {
            event_bus: bus,
            registered_tools: &mut self.tools,
        };
        plugin.on_load(&mut ctx);
        self.plugins.push(plugin);
    }

    pub fn unload_all(&mut self) {
        for p in &self.plugins {
            p.on_unload();
        }
        self.plugins.clear();
    }

    pub fn plugin_count(&self) -> usize { self.plugins.len() }
    pub fn registered_tools(&self) -> &HashMap<String, ToolDescriptor> { &self.tools }
}
