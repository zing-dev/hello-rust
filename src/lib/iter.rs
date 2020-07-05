pub mod iter_test {
    use std::iter;

    #[test]
    fn repeat() {
        let mut fours = iter::repeat(4);
        println!("{:?}", fours);
        assert_eq!(Some(4), fours.next());
        assert_eq!(Some(4), fours.next());
        assert_eq!(Some(4), fours.next());
        assert_eq!(Some(4), fours.next());
        assert_eq!(Some(4), fours.next());

        // yup, still four
        assert_eq!(Some(4), fours.next());
        assert_eq!(Some(4), fours.next());

        // that last example was too many fours. Let's only have four fours.
        let mut four_fours = iter::repeat(4).take(4);

        assert_eq!(Some(4), four_fours.next());
        assert_eq!(Some(4), four_fours.next());
        assert_eq!(Some(4), four_fours.next());
        assert_eq!(Some(4), four_fours.next());

        // ... and now we're done
        assert_eq!(None, four_fours.next());
    }

    #[test]
    fn repeat_with() {
        #[derive(PartialEq, Debug)]
        struct Expensive;

        // a particular value forever:
        let mut things = iter::repeat_with(|| Expensive);

        assert_eq!(Some(Expensive), things.next());
        assert_eq!(Some(Expensive), things.next());
        assert_eq!(Some(Expensive), things.next());
        assert_eq!(Some(Expensive), things.next());
        assert_eq!(Some(Expensive), things.next());

        // From the zeroth to the third power of two:
        let mut curr = 1;
        let mut pow2 = iter::repeat_with(|| {
            let tmp = curr;
            curr *= 2;
            tmp
        })
        .take(4);

        assert_eq!(Some(1), pow2.next());
        assert_eq!(Some(2), pow2.next());
        assert_eq!(Some(4), pow2.next());
        assert_eq!(Some(8), pow2.next());

        // ... and now we're done
        assert_eq!(None, pow2.next());
    }

    #[test]
    fn empty() {
        // this could have been an iterator over i32, but alas, it's just not.
        let mut nope = iter::empty::<i32>();
        assert_eq!(None, nope.next());
    }

    #[test]
    fn once() {
        // one is the loneliest number
        let mut one = iter::once(1);

        assert_eq!(Some(1), one.next());

        // just one, that's all we get
        assert_eq!(None, one.next());
    }

    #[test]
    fn once_with() {
        // one is the loneliest number
        let mut one = iter::once_with(|| 1);
        assert_eq!(Some(1), one.next());
        // just one, that's all we get
        assert_eq!(None, one.next());
    }

    #[test]
    fn from_fn() {
        let mut count = 0;
        let counter = iter::from_fn(move || {
            // Increment our count. This is why we started at zero.
            count += 1;

            // Check to see if we've finished counting or not.
            if count < 6 {
                Some(count)
            } else {
                None
            }
        });
        assert_eq!(counter.collect::<Vec<_>>(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn iterator() {
        /// An iterator which counts from one to five
        struct Counter {
            count: usize,
        }

        // we want our count to start at one, so let's add a new() method to help.
        // This isn't strictly necessary, but is convenient. Note that we start
        // `count` at zero, we'll see why in `next()`'s implementation below.
        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        // Then, we implement `Iterator` for our `Counter`:

        impl Iterator for Counter {
            // we will be counting with usize
            type Item = usize;

            // next() is the only required method
            fn next(&mut self) -> Option<Self::Item> {
                // Increment our count. This is why we started at zero.
                self.count += 1;

                // Check to see if we've finished counting or not.
                if self.count < 6 {
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        // And now we can use it!

        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
}
