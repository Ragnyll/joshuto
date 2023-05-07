use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::AppBackend;

pub trait AppExecute {
    fn execute<T: AppBackend>(
        &self,
        context: &mut AppContext,
        backend: &mut T,
        keymap_t: &AppKeyMapping,
    ) -> JoshutoResult;
}

pub trait NumberedExecute {
    fn numbered_execute<T: AppBackend>(
        &self,
        number_prefix: usize,
        context: &mut AppContext,
        backend: &mut T,
        keymap_t: &AppKeyMapping,
    ) -> JoshutoResult;
}

pub trait InteractiveExecute {
    fn interactive_execute(&self, context: &mut AppContext);
}

pub trait AppCommand: AppExecute + std::fmt::Display + std::fmt::Debug {
    fn command(&self) -> &'static str;
}

pub trait CommandComment {
    fn comment(&self) -> &'static str;
}
