use crate::stream::Stream;

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

/// Future for the [`any`](super::StreamExt::any) method.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct AnyFuture<'a, St: ?Sized, F> {
    stream: &'a mut St,
    f: F,
}

impl<'a, St: ?Sized, F> AnyFuture<'a, St, F> {
    pub(super) fn new(stream: &'a mut St, f: F) -> Self {
        Self { stream, f }
    }
}

impl<St: ?Sized + Unpin, F> Unpin for AnyFuture<'_, St, F> {}

impl<St, F> Future for AnyFuture<'_, St, F>
where
    St: ?Sized + Stream + Unpin,
    F: FnMut(St::Item) -> bool,
{
    type Output = bool;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let next = futures_core::ready!(Pin::new(&mut self.stream).poll_next(cx));

        match next {
            Some(v) => {
                if (&mut self.f)(v) {
                    Poll::Ready(true)
                } else {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
            None => Poll::Ready(false),
        }
    }
}
