use std::sync::{Arc, Mutex};

use leptos::{IntoView, tachys::reactive_graph::ReactiveFunction};

use super::{LitWrapper, Literal};
use crate::{I18nContext, Locale, Scope};

/// A key accessor result that renders without any interpolated value.
///
/// Implemented for the literal wrapper, and by the generated code for the
/// builder of a key whose literal kind differs between locales. A key with
/// interpolated values has no implementation: it goes through the closure
/// expansion of `t!` with its values supplied.
#[doc(hidden)]
#[diagnostic::on_unimplemented(
    message = "this key has interpolated values, pass them: `t!(i18n, key, value = ...)`",
    label = "returns a builder, not a plain translation"
)]
pub trait PlainTranslation: Copy {
    type View: IntoView + 'static;
    fn into_view(self) -> Self::View;
}

impl<T: Literal + 'static> PlainTranslation for LitWrapper<T> {
    type View = T;

    fn into_view(self) -> T {
        self.inner()
    }
}

/// The reactive text of one plain key, read from a context.
///
/// The accessor travels as a function pointer, so the type depends only on the
/// locale, the scope and the accessor's return type, never on the call site.
/// Leptos renders it through its `ReactiveFunction` implementations, which are
/// then monomorphized once per crate instead of once per `t!` invocation.
#[doc(hidden)]
pub struct TranslatedText<L: Locale, S: Scope<L>, R> {
    ctx: I18nContext<L, S>,
    key: fn(S::Keys) -> R,
}

impl<L: Locale, S: Scope<L>, R: PlainTranslation> TranslatedText<L, S, R> {
    pub const fn new(ctx: I18nContext<L, S>, key: fn(S::Keys) -> R) -> Self {
        TranslatedText { ctx, key }
    }

    fn get(&self) -> R::View {
        (self.key)(self.ctx.get_keys()).into_view()
    }
}

impl<L: Locale, S: Scope<L>, R> Clone for TranslatedText<L, S, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Locale, S: Scope<L>, R> Copy for TranslatedText<L, S, R> {}

impl<L: Locale, S: Scope<L>, R: PlainTranslation + 'static> ReactiveFunction
    for TranslatedText<L, S, R>
{
    type Output = R::View;

    fn invoke(&mut self) -> Self::Output {
        self.get()
    }

    fn into_shared(self) -> Arc<Mutex<dyn FnMut() -> Self::Output + Send>> {
        Arc::new(Mutex::new(move || self.get()))
    }
}
