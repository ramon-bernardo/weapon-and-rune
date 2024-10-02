use std::sync::Arc;

use bevy::prelude::*;
use rune::{
    runtime::RuntimeContext,
    termcolor::{ColorChoice, StandardStream},
    Diagnostics, Sources, Unit, Vm,
};

use crate::weapons;

// The RuneContext struct holds the runtime and compiled Rune unit, both wrapped in Arc for shared ownership.
#[derive(Resource)]
pub struct RuneContext {
    runtime: Arc<RuntimeContext>,
    unit: Arc<Unit>,
}

impl RuneContext {
    pub fn new(mut sources: Sources) -> anyhow::Result<Self> {
        // Construct a default Rune context and installs the weapons module.
        let mut context = rune_modules::default_context()?;
        context.install(weapons::module()?)?;

        // Diagnostic object to gather any issues during script compilation.
        let mut diagnostics = Diagnostics::new();

        // Compiles the sources into a unit, providing the context and diagnostic handler.
        let unit = rune::prepare(&mut sources)
            .with_context(&context)
            .with_diagnostics(&mut diagnostics)
            .build();

        // If there are any diagnostics (warnings or errors), they are printed to the standard error.
        if !diagnostics.is_empty() {
            let mut writer = StandardStream::stderr(ColorChoice::Always);
            diagnostics.emit(&mut writer, &sources)?; // Emit diagnostics with colored output
        }

        let unit = unit?; // Checks if the compilation was successful and retrieves the unit.
        let runtime = context.runtime()?; // Creates a runtime context from the module context.

        Ok(Self {
            runtime: Arc::new(runtime),
            unit: Arc::new(unit),
        })
    }

    // Construct a new rune virtual machine. (very cheap, it just allocates for the stack.)
    pub fn vm(&self) -> Vm {
        Vm::new(self.runtime.clone(), self.unit.clone())
    }
}
