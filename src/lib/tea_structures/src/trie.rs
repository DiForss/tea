use std::collections::BTreeMap;
use std::marker::PhantomData;

use functional::either::Either;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Trie<K, V>
	where K: Ord {
	map: BTreeMap<K, Either<Box<Trie<K, V>>, V>>
}

impl<K, V> Trie<K, V>
	where K: Ord {
	pub fn new() -> Self {
		Self {
			map: BTreeMap::new(),
		}
	}

	pub fn insert(&mut self, key: &[K], val: V) {
		for i in 0..key.len()-1 {
			let k = key[i];
				
			match self.map.entry(k).or_insert(Either::Left(Box::new(Trie::new()))) {
				&mut Either::Left(t) => t.insert(key.get(i..key.len() - 1).unwrap(), val),
				&mut Either::Right(_) => { self.map.insert(k, Either::Right(val)); },
			};
		}
	}
}

