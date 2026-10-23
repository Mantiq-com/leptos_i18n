use crate::i18n::*;
use leptos_i18n::context::{
    CookieOptions, I18nContextOptions, UseLocalesOptions, init_i18n_context_with_options,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};
use tests_common::*;

const RENDERS: usize = 2_000;

/// A server renders its app once at startup to list the routes, then disposes
/// the owner of that render. Work the context schedules on an executor thread
/// can run during that disposal and must not read the disposed locale. Such a
/// panic happens on the executor thread, so the test counts it through the
/// panic hook.
#[test]
fn context_effect_survives_the_disposal_of_its_owner() {
    let _ = any_spawner::Executor::init_futures_executor();

    let disposed_reads = Arc::new(AtomicUsize::new(0));
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook({
        let disposed_reads = Arc::clone(&disposed_reads);
        Box::new(move |info| {
            if info.to_string().contains("already been disposed") {
                disposed_reads.fetch_add(1, Ordering::SeqCst);
            }
            previous_hook(info);
        })
    });

    for _ in 0..RENDERS {
        let owner = Owner::new_root(None);
        owner.with(|| {
            // There is no request, so the headers are read as absent instead
            // of warning about the missing server integration on every render.
            let options = I18nContextOptions::default()
                .cookie_options(CookieOptions::default().ssr_cookies_header_getter(|| None))
                .ssr_lang_header_getter(
                    UseLocalesOptions::default().ssr_lang_header_getter(|| None),
                );
            let _ = init_i18n_context_with_options::<Locale>(options);
        });
        drop(owner);
    }

    // The effects have no handle to join, so give the executor threads time
    // to finish the runs that were already scheduled.
    thread::sleep(Duration::from_millis(500));

    assert_eq!(
        disposed_reads.load(Ordering::SeqCst),
        0,
        "an i18n effect read a disposed signal in {RENDERS} renders"
    );
}
