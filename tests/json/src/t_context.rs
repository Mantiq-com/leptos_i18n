use crate::i18n::*;
use leptos_i18n::{I18nContext, context::init_i18n_subcontext};
use tests_common::*;

/// Runs `test` with a context started on `locale`. A subcontext resolves its
/// locale without a request or a browser.
fn with_context(locale: Locale, test: impl FnOnce(I18nContext<Locale>)) {
    Owner::new().with(|| {
        let i18n = init_i18n_subcontext::<Locale>(Some(Signal::stored(locale)));
        test(i18n);
    });
}

#[test]
fn plain_key_follows_the_locale() {
    with_context(Locale::en, |i18n| {
        assert_eq_rendered!(t!(i18n, click_to_change_lang), "Click to change language");
        i18n.set_locale(Locale::fr);
        assert_eq_rendered!(
            t!(i18n, click_to_change_lang),
            "Cliquez pour changez de langue"
        );
    });
}

#[test]
fn plain_key_is_copy() {
    with_context(Locale::en, |i18n| {
        let text = t!(i18n, click_to_change_lang);
        let copy = text;
        assert_eq_rendered!(text, "Click to change language");
        assert_eq_rendered!(copy, "Click to change language");
    });
}

#[test]
fn plain_key_inside_a_view() {
    with_context(Locale::en, |i18n| {
        let view = view! { <p>"before "{t!(i18n, click_to_change_lang)}" after"</p> };
        assert_eq_rendered!(view, "<p>before Click to change language after</p>");
    });
}

#[test]
fn subkey() {
    with_context(Locale::en, |i18n| {
        assert_eq_rendered!(t!(i18n, subkeys.subkey_1), "subkey_1");
        i18n.set_locale(Locale::fr);
        assert_eq_rendered!(t!(i18n, subkeys.subkey_1), "subkey_1");
    });
}

#[test]
fn scoped_context() {
    with_context(Locale::en, |i18n| {
        let scoped = scope_i18n!(i18n, subkeys);
        assert_eq_rendered!(t!(scoped, subkey_1), "subkey_1");
        assert_eq_rendered!(t!(scoped, subkey_3, count = 4), "4");
    });
}

#[test]
fn bool_and_number_literals() {
    with_context(Locale::en, |i18n| {
        assert_eq_rendered!(t!(i18n, same_lit_type), "true");
        assert_eq_rendered!(t!(i18n, mixed_lit_type), "59.89");
        i18n.set_locale(Locale::fr);
        assert_eq_rendered!(t!(i18n, same_lit_type), "false");
        assert_eq_rendered!(t!(i18n, mixed_lit_type), "true");
    });
}

#[test]
fn interpolated_key() {
    with_context(Locale::en, |i18n| {
        assert_eq_rendered!(t!(i18n, click_count, count = 3), "You clicked 3 times");
        i18n.set_locale(Locale::fr);
        assert_eq_rendered!(t!(i18n, click_count, count = 3), "Vous avez cliqué 3 fois");
    });
}
