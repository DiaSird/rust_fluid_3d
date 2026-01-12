use tauri::{Emitter, Listener, Window};
use utils::parameters::StopJudgeFn;

/// Create closure that reports.
pub(crate) fn sender<S>(window: Window, event: &'static str) -> impl Fn(S) + Clone
where
    S: serde::Serialize + Clone,
{
    move |payload: S| {
        if let Err(err) = window.emit(event, payload) {
            println!("{err}");
            // tracing::error!("{}", err);
        };
    }
}

pub(crate) fn new_stop_listener(window: Window) -> StopJudgeFn {
    let stop_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    {
        let stop_flag = std::sync::Arc::clone(&stop_flag);
        window.listen("terra://simulation-stop-event", move |_event| {
            stop_flag.store(true, std::sync::atomic::Ordering::Release);
        });
    }

    let stop_flag = std::sync::Arc::clone(&stop_flag);
    Box::new(move |_step| stop_flag.load(std::sync::atomic::Ordering::Acquire))
}
