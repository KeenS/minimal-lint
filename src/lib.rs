// force-host
#![allow(deprecated)]
#![feature(plugin_registrar)]
#![feature(box_syntax, rustc_private)]

extern crate syntax;

// Load rustc as a plugin to get macros
#[macro_use]
extern crate rustc;
#[macro_use]
extern crate rustc_session;
extern crate rustc_driver;
extern crate rustc_errors;

use rustc::lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};
use rustc_driver::plugin::Registry;
use rustc_errors::Applicability;
use syntax::ast;

declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");

declare_lint_pass!(Pass => [TEST_LINT]);

impl EarlyLintPass for Pass {
    fn check_item(&mut self, cx: &EarlyContext, it: &ast::Item) {
        if it.ident.name.as_str() == "lintme" {
            let mut diag = cx.struct_span_lint(TEST_LINT, it.span, "item is named 'lintme'");
            let span = it.ident.span;
            diag.span_suggestion(
                span,
                "you can rename it",
                "lintyou".into(),
                Applicability::MachineApplicable,
            )
            .emit()
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.lint_store.register_lints(&[&TEST_LINT]);
    reg.lint_store.register_early_pass(|| box Pass);
}
