#![warn(clippy::await_holding_tracing_entered_guard)]

use tracing::{info_span, Span};

async fn bad(span: &Span) -> u32 {
    let _guard = span.enter();
    baz().await
}

async fn bad_mut(span: &Span) -> u32 {
    let _guard = span.enter();
    baz().await
}

async fn good(span: &Span) -> u32 {
    {
        let _guard = span.enter();
        let y = 1;
    }
    baz().await;
    let _guard = span.enter();
    47
}

async fn baz() -> u32 {
    42
}

async fn also_bad(span: &Span) -> u32 {
    let first = baz().await;

    let _guard = span.enter();

    let second = baz().await;

    let third = baz().await;

    first + second + third
}

async fn less_bad(span: &Span) -> u32 {
    let first = baz().await;

    let _guard = span.enter();

    let second = baz().await;

    drop(_guard);

    let third = baz().await;

    first + second + third
}

async fn not_good(span: &Span) -> u32 {
    let first = baz().await;

    let second = {
        let _guard = span.enter();
        baz().await
    };

    let third = baz().await;

    first + second + third
}

#[allow(clippy::manual_async_fn)]
fn block_bad(span: &Span) -> impl std::future::Future<Output = u32> + '_ {
    async move {
        let _guard = span.enter();
        baz().await
    }
}

fn main() {
    let span = info_span!("");
    good(&span);
    bad(&span);
    bad_mut(&span);
    also_bad(&span);
    less_bad(&span);
    not_good(&span);
    block_bad(&span);
}
