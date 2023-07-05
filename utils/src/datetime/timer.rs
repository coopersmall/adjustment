use std::ops::Sub;

pub trait Clock<T>
where
    T: Iterator<Item = T>
        + DoubleEndedIterator<Item = T>
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Sub<Output = T>,
{
    fn start(moment: T) -> T {
        moment
    }

    fn stop(moment: T, stop: usize) -> Option<T> {
        let position = match moment.position(|x| x == moment) {
            Some(position) => position,
            None => return None,
        };
        moment.nth(position + stop)
    }

    fn tick(moment: T) -> Option<T> {
        moment.next()
    }

    fn back(moment: T) -> Option<T> {
        moment.next_back()
    }

    fn duration(start: T, stop: T) -> Option<T> {
        Some(stop - start)
    }
}
