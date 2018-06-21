use std::iter::Peekable;

pub struct TakeWhilePeek<I: Iterator, F: FnMut(I::Item)> {
	iter: Peekable<I>
}