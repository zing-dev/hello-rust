pub fn vector() {
    let mut v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    v.push(1);
    println!("{:?}", v);
    v.push(2);
    println!("{:?}", v);//[1, 2]
    v.pop();
    println!("{:?}", v);//[1]
    v.extend(&[10, 12, 13, 14]);
    println!("{:?}", v); //[1, 10, 12, 13, 14]

    println!("{} {} {}", v[0], v[1], v[2]);

    for i in &v {
        print!("{}\t", i);
    }
    println!();
    println!("{:?}", v.get(1));
    println!("{:?}", v);
    v.insert(0, 21);
    v.insert(3, 22);
    println!("{:?}", v);//[21, 1, 10, 22, 12, 13, 14]
    println!("{}", v.len()); //7
    v.reverse();
    println!("{:?}", v);//[14, 13, 12, 22, 10, 1, 21]

    let mut v2 = vec![30, 31, 32, 33];
    v.append(&mut v2);
    println!("{:?}", v);//[14, 13, 12, 22, 10, 1, 21, 30, 31, 32, 33]
    println!("{:?}", v.as_slice());

    println!("{}", v.contains(&1)); //true
    println!("{}", v.ends_with(&vec![33])); //true

    v.truncate(8);
    println!("{:?}", v);//[14, 13, 12, 22, 10, 1, 21, 30]

    println!("{:?}", v.first());//Some(14)
    println!("{:?}", v.last());//Some(30)
    println!("{}", v.is_empty()); //false

    let mut iter = v.iter();
    println!("{:?}", iter.next());//Some(14)
    println!("{:?}", iter.next());//Some(13)
}

pub fn string() {
    let string = String::new();
    println!("string -> {}", string);
    println!("{}", String::from("hello world"));
    let mut str1 = String::from("hello rust");
    str1.push('.');
    str1.push('哈');
    println!("{}", str1);
    println!("{}", str1.len());
    println!("{}", '哈'.to_string().len());
}

#[allow(unused_variables)]
pub fn string2() {

    //There are multiple ways to create a new [`String`] from a string literal:
    let s = "Hello".to_string();
    let s = String::from("world");
    let s: String = "also this".into();

    let message = s + " world!";

    let sparkle_heart = vec![240, 159, 146, 150];
    // We know these bytes are valid, so we'll use `unwrap()`.
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    assert_eq!("💖", sparkle_heart);
    let bytes = sparkle_heart.into_bytes();
    assert_eq!(bytes, [240, 159, 146, 150]);

    let mut hello = String::from("Hello, ");
    hello.push('w');
    hello.push_str("orld!");
}

pub fn string3() {
    use std::mem;
    let story = String::from("Once upon a time...");
    let ptr = story.as_ptr();
    let len = story.len();
    let capacity = story.capacity();
// story has nineteen bytes
    assert_eq!(19, len);
// Now that we have our parts, we throw the story away.
    mem::forget(story);
// We can re-build a String out of ptr, len, and capacity. This is all
// unsafe because we are responsible for making sure the components are
// valid:
    let s = unsafe {
        String::from_raw_parts(ptr as *mut _, len, capacity)
    };
    assert_eq!(String::from("Once upon a time..."), s);

    let mut s = String::new();
    println!("{}", s.capacity());
    for _ in 0..5 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    let mut s = String::with_capacity(25);
    println!("{}", s.capacity());
    for _ in 0..10 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    /////////////////////////////////////////////
    // some invalid bytes, in a vector
    let bytes = vec![0, 159];
    let value = String::from_utf8(bytes);
    assert!(value.is_err());
    assert_eq!(vec![0, 159], value.unwrap_err().into_bytes());

    let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,
        0xD800, 0x0069, 0x0063];
    assert!(String::from_utf16(v).is_err());
    println!("{}", String::from_utf16(v).is_ok());
    println!("{:?}", String::from("哈哈").into_bytes());
    println!("{}", String::from_utf8(String::from("哈哈").into_bytes()).unwrap());


    let s = String::from("hello");
    let bytes = s.into_bytes();
    println!("{:?}", bytes);
    assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);

    let mut s = String::from("foobar");
    let s_mut_str = s.as_mut_str();
    s_mut_str.make_ascii_uppercase();
    assert_eq!("FOOBAR", s_mut_str);


    //reserve
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
// s now has a length of 2 and a capacity of 10
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
// Since we already have an extra 8 capacity, calling this...
    s.reserve(8);
    println!("{}", s);
// ... doesn't actually increase.
    assert_eq!(10, s.capacity());


    //push
    let mut s = String::from("abc");
    s.push('1');
    s.push('2');
    s.push('3');
    assert_eq!("abc123", s);

    //as_bytes
    let s = String::from("hello");
    assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());

    //truncate
    let mut s = String::from("hello");
    s.truncate(2);
    assert_eq!("he", s);

    //pop
    let mut s = String::from("foo");
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('f'));
    assert_eq!(s.pop(), None);

    //remove
    let mut s = String::from("foo");
    assert_eq!(s.remove(0), 'f');
    assert_eq!(s.remove(1), 'o');
    assert_eq!(s.remove(0), 'o');
    assert_eq!(s, "");


    //retain
    let mut s = String::from("f_o_ob_ar");
    s.retain(|c| c != '_');
    assert_eq!(s, "foobar");

    let mut s = String::from("abcde");
    let keep = [false, true, true, false, true];
    let mut i = 0;
    s.retain(|_| (keep[i], i += 1).0);
    assert_eq!(s, "bce");

    //insert
    let mut s = String::with_capacity(3);
    s.insert(0, 'f');
    s.insert(1, 'o');
    s.insert(2, 'o');
    assert_eq!("foo", s);

    //insert_str
    let mut s = String::from("bar");
    s.insert_str(0, "foo");
    assert_eq!("foobar", s);

    //len
    let a = String::from("foo");
    assert_eq!(a.len(), 3);

    //is_empty
    let mut v = String::new();
    assert!(v.is_empty());
    v.push('a');
    assert!(!v.is_empty());

    //split_off
    let mut hello = String::from("Hello, World!");
    let world = hello.split_off(7);
    assert_eq!(hello, "Hello, ");
    assert_eq!(world, "World!");

    //clear
    let mut s = String::from("foo");
    s.clear();
    assert!(s.is_empty());
    assert_eq!(0, s.len());
    assert_eq!(3, s.capacity());

    //drain
    let mut s = String::from("α is alpha, β is beta");
    let beta_offset = s.find('β').unwrap_or(s.len());
    // Remove the range up until the β from the string
    let t: String = s.drain(..beta_offset).collect();
    assert_eq!(t, "α is alpha, ");
    assert_eq!(s, "β is beta");
    // A full range clears the string
    s.drain(..);
    assert_eq!(s, "");


    //replace_range
    let mut s = String::from("α is alpha, β is beta");
    let beta_offset = s.find('β').unwrap_or(s.len());
    // Replace the range up until the β from the string
    s.replace_range(0..beta_offset, "Α is capital alpha; ");
    assert_eq!(s, "Α is capital alpha; β is beta");

    //as_bytes
    let bytes = vec![0, 159];
    let value = String::from_utf8(bytes);
    assert_eq!(&[0, 159], value.unwrap_err().as_bytes());

    //into_bytes
    // some invalid bytes, in a vector
    let bytes = vec![0, 159];
    let value = String::from_utf8(bytes);
    assert_eq!(vec![0, 159], value.unwrap_err().into_bytes());

    //utf8_error
    // some invalid bytes, in a vector
    let bytes = vec![0, 159];
    let error = String::from_utf8(bytes).unwrap_err().utf8_error();
    // the first byte is invalid here
    assert_eq!(1, error.valid_up_to());
}

pub fn map() {
    use std::collections::HashMap;
// Type inference lets us omit an explicit type signature (which
// would be `HashMap<String, String>` in this example).
    let mut book_reviews = HashMap::new();
// Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );
// Check for a specific one.
// When collections store owned values (String), they can still be
// queried using references (&str).
    if !book_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.",
                 book_reviews.len());
    }
// oops, this review has a lot of spelling mistakes, let's delete it.
    book_reviews.remove("The Adventures of Sherlock Holmes");
// Look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed.", book)
        }
    }
// Look up the value for a key (will panic if the key is not found).
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);
// Iterate over everything.
    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
    }

    ///////////////////////////////////////////////////////////////////////////
    // type inference lets us omit an explicit type signature (which
// would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();
    fn random_stat_buff() -> u8 {
        // could actually return some random value here - let's just return
        // some fixed value for now
        42
    }
// insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);
// insert a key using a function that provides a new value only if it
// doesn't already exist
    player_stats.entry("defence").or_insert_with(random_stat_buff);
// update a key, guarding against the key possibly not being set
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += random_stat_buff();

    for (player, stat) in &player_stats {
        println!("{}: \"{}\"", player, stat);
    }

    //////////////////////////////////////////////////////////////////////////////
    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Viking {
        name: String,
        country: String,
    }
    impl Viking {
        /// Creates a new Viking.
        fn new(name: &str, country: &str) -> Viking {
            Viking { name: name.to_string(), country: country.to_string() }
        }
    }
    // Use a HashMap to store the vikings' health points.
    let mut vikings = HashMap::new();
    vikings.insert(Viking::new("Einar", "Norway"), 25);
    vikings.insert(Viking::new("Olaf", "Denmark"), 24);
    vikings.insert(Viking::new("Harald", "Iceland"), 12);
    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }

    /////////////////////////////////////////////////////////////////////////////////
    let timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)].
        iter().
        cloned().
        collect();
    for (resource, timber) in &timber_resources {
        println!("{} ==>  {}", resource, timber);
    }

    //capacity
    let map: HashMap<i32, i32> = HashMap::with_capacity(100);
    assert!(map.capacity() >= 100);

    //keys
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    for key in map.keys() {
        println!("{}", key);
    }

    //values
    for val in map.values() {
        println!("{}", val);
    }

    //values_mut
    for val in map.values_mut() {
        *val = *val + 10;
    }
    for val in map.values() {
        println!("{}", val);
    }

    //iter
    for (key, val) in map.iter() {
        println!("key: {} val: {}", key, val);
    }

    //iter_mut
    // Update all values
    for (_, val) in map.iter_mut() {
        *val *= 2;
    }
    for (key, val) in &map {
        println!("key: {} val: {}", key, val);
    }

    //len
    let mut a = HashMap::new();
    assert_eq!(a.len(), 0);
    a.insert(1, "a");
    assert_eq!(a.len(), 1);

    //is_empty
    let mut a = HashMap::new();
    assert!(a.is_empty());
    a.insert(1, "a");
    assert!(!a.is_empty());

    //drain
    let mut a = HashMap::new();
    a.insert(1, "a");
    a.insert(2, "b");
    for (k, v) in a.drain().take(2) {
        println!("key: {} val: {}", k, v);
        assert!(k == 1 || k == 2);
        assert!(v == "a" || v == "b");
    }
    assert!(a.is_empty());

    //clear
    let mut a = HashMap::new();
    a.insert(1, "a");
    a.clear();
    assert!(a.is_empty());

    //entry
    let mut letters = HashMap::new();
    for ch in "a short treatise on fungi".chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }
    assert_eq!(letters[&'s'], 2);
    assert_eq!(letters[&'t'], 3);
    assert_eq!(letters[&'u'], 1);
    assert_eq!(letters[&' '], 4);
    assert_eq!(letters.get(&'y'), None);

    //get
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), None);

    //contains_key
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.contains_key(&1), true);
    assert_eq!(map.contains_key(&2), false);

    //get_mut
    let mut map = HashMap::new();
    map.insert(1, "a");
    if let Some(x) = map.get_mut(&1) {
        *x = "b";
    }
    assert_eq!(map[&1], "b");

    //insert
    let mut map = HashMap::new();
    assert_eq!(map.insert(37, "a"), None);
    assert_eq!(map.is_empty(), false);
    map.insert(37, "b");
    assert_eq!(map.insert(37, "c"), Some("b"));
    assert_eq!(map[&37], "c");

    //remove
    let mut map = HashMap::new();
    map.insert(1, "a");
    println!("{:?}", map);
    assert_eq!(map.remove(&1), Some("a"));
    println!("{:?}", map);
    assert_eq!(map.remove(&1), None);

    //remove_entry
    let mut map = HashMap::new();
    map.insert(1, "a");
    println!("{:?}", map);
    assert_eq!(map.remove_entry(&1), Some((1, "a")));
    println!("{:?}", map);
    assert_eq!(map.remove(&1), None);

    //retain
    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    println!("{:?}", map);
    map.retain(|&k, v| {
        *v *= 2;
        k % 2 == 0
    });
    println!("{:?}", map);
    assert_eq!(map.len(), 4);

    //into_iter
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    // Not possible with .iter()
    let vec: Vec<(&str, i32)> = map.into_iter().collect();
    println!("{:?}", vec);

    //or_insert
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(3);
    assert_eq!(map["poneyland"], 3);
    *map.entry("poneyland").or_insert(10) *= 2;
    assert_eq!(map["poneyland"], 6);

    //or_insert_with
    let mut map: HashMap<&str, String> = HashMap::new();
    let s = "hoho".to_string();
    map.entry("poneyland").or_insert_with(|| s);
    assert_eq!(map["poneyland"], "hoho".to_string());

    //key
    let mut map: HashMap<&str, u32> = HashMap::new();
    assert_eq!(map.entry("poneyland").key(), &"poneyland");
    println!("{:?}", map.entry("poneyland"));
    println!("{}", map.entry("poneyland").key());
    println!("{}", map.entry("fuck").key());
    println!("{:?}", map);

    //and_modify
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland")
        .and_modify(|e| { *e += 1 })
        .or_insert(42);
    assert_eq!(map["poneyland"], 42);
    map.entry("poneyland")
        .and_modify(|e| { *e += 1 })
        .or_insert(42);
    assert_eq!(map["poneyland"], 43);

    //or_default
    let mut map: HashMap<&str, Option<u32>> = HashMap::new();
    map.entry("poneyland").or_default();
    assert_eq!(map["poneyland"], None);

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(12);
    assert_eq!(map.entry("poneyland").key(), &"poneyland");

    use std::collections::hash_map::Entry;
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(12);
    if let Entry::Occupied(o) = map.entry("poneyland") {
        // We delete the entry from the map.
        println!("{:?}",o);
        o.remove_entry();
    }
    assert_eq!(map.contains_key("poneyland"), false);
}

pub fn set(){
    use std::collections::HashSet;
    // Type inference lets us omit an explicit type signature (which
    // would be `HashSet<String>` in this example).
    let mut books = HashSet::new();
    // Add some books.
    books.insert("A Dance With Dragons".to_string());
    books.insert("To Kill a Mockingbird".to_string());
    books.insert("The Odyssey".to_string());
    books.insert("The Great Gatsby".to_string());
    // Check for a specific one.
    if !books.contains("The Winds of Winter") {
        println!("We have {} books, but The Winds of Winter ain't one.",
                 books.len());
    }
    // Remove a book.
    books.remove("The Odyssey");
    // Iterate over everything.
    for book in &books {
        println!("{}", book);
    }

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Viking {
        name: String,
        power: usize,
    }
    let mut vikings = HashSet::new();
    vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
    vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
    vikings.insert(Viking { name: "Olaf".to_string(), power: 4 });
    vikings.insert(Viking { name: "Harald".to_string(), power: 8 });
    // Use derived implementation to print the vikings.
    for x in &vikings {
        println!("{:?}", x);
    }

    //new
    let _set: HashSet<i32> = HashSet::new();

    //with_capacity capacity
    let set: HashSet<i32> = HashSet::with_capacity(10);
    assert!(set.capacity() >= 10);

    //iter
    let mut set = HashSet::new();
    set.insert("a");
    set.insert("b");
    set.insert("c");
    set.insert("d");
    for x in set.iter() {
        // Will print in an arbitrary order.
        println!("{}", x);
    }

    //len
    let mut v = HashSet::new();
    assert_eq!(v.len(), 0);
    v.insert(1);
    assert_eq!(v.len(), 1);

    //is_empty
    let mut v = HashSet::new();
    assert!(v.is_empty());
    v.insert(1);
    assert!(!v.is_empty());

    //drain
    let mut set: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    assert!(!set.is_empty());
    // print 1, 2, 3 in an arbitrary order
    for i in set.drain() {
        println!("{}", i);
    }
    assert!(set.is_empty());

    //clear
    let mut v = HashSet::new();
    v.insert(1);
    v.clear();
    assert!(v.is_empty());

    //with_hasher
    use std::collections::hash_map::RandomState;
    let s = RandomState::new();
    let mut set = HashSet::with_hasher(s);
    set.insert(2);

    //with_capacity_and_hasher
    let s = RandomState::new();
    let mut set = HashSet::with_capacity_and_hasher(10, s);
    set.insert(1);

    //hasher
    let hasher = RandomState::new();
    let set: HashSet<i32> = HashSet::with_hasher(hasher);
    let hasher: &RandomState = set.hasher();

    //reserve
    let mut set: HashSet<i32> = HashSet::new();
    set.reserve(10);
    assert!(set.capacity() >= 10);

    //shrink_to_fit
    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    assert!(set.capacity() >= 100);
    set.shrink_to_fit();
    assert!(set.capacity() >= 2);

    //difference
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
    // Can be seen as `a - b`.
    for x in a.difference(&b) {
        println!("difference a {}", x); // Print 1
    }
    for x in b.difference(&a) {
        println!("difference b {}", x); // Print 1
    }
    let diff: HashSet<_> = a.difference(&b).collect();
    assert_eq!(diff, [1].iter().collect());
    // Note that difference is not symmetric,
    // and `b - a` means something else:
    let diff: HashSet<_> = b.difference(&a).collect();
    assert_eq!(diff, [4].iter().collect());

    //symmetric_difference
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
    // Print 1, 4 in arbitrary order.
    for x in a.symmetric_difference(&b) {
        println!("symmetric_difference a {}", x);
    }
    for x in b.symmetric_difference(&a) {
        println!("symmetric_difference b {}", x);
    }
    let diff1: HashSet<_> = a.symmetric_difference(&b).collect();
    let diff2: HashSet<_> = b.symmetric_difference(&a).collect();
    assert_eq!(diff1, diff2);
    assert_eq!(diff1, [1, 4].iter().collect());

    //intersection
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
    // Print 2, 3 in arbitrary order.
    for x in a.intersection(&b) {
        println!("{}", x);
    }
    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, [2, 3].iter().collect());

    //union
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
    // Print 1, 2, 3, 4 in arbitrary order.
    for x in a.union(&b) {
        println!("union {}", x);
    }
    let union: HashSet<_> = a.union(&b).collect();
    assert_eq!(union, [1, 2, 3, 4].iter().collect());

    let set: HashSet<_> = [1, 2, 3].iter().clone().collect();
    assert_eq!(set.contains(&1), true);
    assert_eq!(set.contains(&4), false);

    //get
    let set: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    assert_eq!(set.get(&2), Some(&2));
    assert_eq!(set.get(&4), None);

    //is_disjoint
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let mut b = HashSet::new();
    assert_eq!(a.is_disjoint(&b), true);
    b.insert(4);
    assert_eq!(a.is_disjoint(&b), true);
    b.insert(1);
    assert_eq!(a.is_disjoint(&b), false);

    //is_subset
    let sup: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let mut set = HashSet::new();
    assert_eq!(set.is_subset(&sup), true);
    set.insert(2);
    assert_eq!(set.is_subset(&sup), true);
    set.insert(4);
    assert_eq!(set.is_subset(&sup), false);

    //is_superset
    let sub: HashSet<_> = [1, 2].iter().cloned().collect();
    let mut set = HashSet::new();
    assert_eq!(set.is_superset(&sub), false);
    set.insert(0);
    set.insert(1);
    assert_eq!(set.is_superset(&sub), false);
    set.insert(2);
    assert_eq!(set.is_superset(&sub), true);

    //insert
    let mut set = HashSet::new();
    assert_eq!(set.insert(2), true);
    assert_eq!(set.insert(2), false);
    assert_eq!(set.len(), 1);

    //replace
    let mut set = HashSet::new();
    set.insert(Vec::<i32>::new());
    assert_eq!(set.get(&[][..]).unwrap().capacity(), 0);
    set.replace(Vec::with_capacity(10));
    assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);

    //remove
    let mut set = HashSet::new();
    set.insert(2);
    assert_eq!(set.remove(&2), true);
    assert_eq!(set.remove(&2), false);

    //take
    let mut set: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    assert_eq!(set.take(&2), Some(2));
    assert_eq!(set.take(&2), None);

    //retain
    let xs = [1,2,3,4,5,6];
    let mut set: HashSet<i32> = xs.iter().cloned().collect();
    set.retain(|&k| k % 2 == 0);
    assert_eq!(set.len(), 3);
}

