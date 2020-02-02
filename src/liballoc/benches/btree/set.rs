use std::collections::BTreeSet;

use rand::{thread_rng, Rng};
use test::Bencher;

fn random(n: usize) -> BTreeSet<usize> {
    let mut rng = thread_rng();
    let mut set = BTreeSet::new();
    while set.len() < n {
        set.insert(rng.gen());
    }
    assert_eq!(set.len(), n);
    set
}

fn neg(n: usize) -> BTreeSet<i32> {
    let set: BTreeSet<i32> = (-(n as i32)..=-1).collect();
    assert_eq!(set.len(), n);
    set
}

fn pos(n: usize) -> BTreeSet<i32> {
    let set: BTreeSet<i32> = (1..=(n as i32)).collect();
    assert_eq!(set.len(), n);
    set
}

fn stagger(n1: usize, factor: usize) -> [BTreeSet<u32>; 2] {
    let n2 = n1 * factor;
    let mut sets = [BTreeSet::new(), BTreeSet::new()];
    for i in 0..(n1 + n2) {
        let b = i % (factor + 1) != 0;
        sets[b as usize].insert(i as u32);
    }
    assert_eq!(sets[0].len(), n1);
    assert_eq!(sets[1].len(), n2);
    sets
}

macro_rules! set_bench {
    ($name: ident, $set_func: ident, $result_func: ident, $sets: expr) => {
        #[bench]
        pub fn $name(b: &mut Bencher) {
            // setup
            let sets = $sets;

            // measure
            b.iter(|| sets[0].$set_func(&sets[1]).$result_func())
        }
    };
}

#[bench]
pub fn clone_100(b: &mut Bencher) {
    let src = pos(100);
    b.iter(|| src.clone())
}

#[bench]
pub fn clone_100_and_clear(b: &mut Bencher) {
    let src = pos(100);
    b.iter(|| src.clone().clear())
}

#[bench]
pub fn clone_100_and_drain_half(b: &mut Bencher) {
    let src = pos(100);
    b.iter(|| {
        let mut set = src.clone();
        for i in set.iter().copied().filter(|i| i % 2 == 0).collect::<Vec<_>>() {
            set.remove(&i);
        }
        assert_eq!(set.len(), 100 / 2);
    })
}

#[bench]
pub fn clone_100_and_into_iter(b: &mut Bencher) {
    let src = pos(100);
    b.iter(|| src.clone().into_iter().count())
}

#[bench]
pub fn clone_100_and_pop_all(b: &mut Bencher) {
    let src = pos(100);
    b.iter(|| {
        let mut set = src.clone();
        while set.pop_first().is_some() {}
        set
    });
}

#[bench]
pub fn clone_100_and_remove_all(b: &mut Bencher) {
    let src = pos(100);
    b.iter(|| {
        let mut set = src.clone();
        while let Some(elt) = set.iter().copied().next() {
            set.remove(&elt);
        }
        set
    });
}

#[bench]
pub fn clone_100_and_remove_half(b: &mut Bencher) {
    let src = pos(100);
    b.iter(|| {
        let mut set = src.clone();
        for i in (2..=100 as i32).step_by(2) {
            set.remove(&i);
        }
        assert_eq!(set.len(), 100 / 2);
        set
    })
}

#[bench]
pub fn clone_10k(b: &mut Bencher) {
    let src = pos(10_000);
    b.iter(|| src.clone())
}

#[bench]
pub fn clone_10k_and_clear(b: &mut Bencher) {
    let src = pos(10_000);
    b.iter(|| src.clone().clear())
}

#[bench]
pub fn clone_10k_and_drain_half(b: &mut Bencher) {
    let src = pos(10_000);
    b.iter(|| {
        let mut set = src.clone();
        for i in set.iter().copied().filter(|i| i % 2 == 0).collect::<Vec<_>>() {
            set.remove(&i);
        }
        assert_eq!(set.len(), 10_000 / 2);
    })
}

#[bench]
pub fn clone_10k_and_into_iter(b: &mut Bencher) {
    let src = pos(10_000);
    b.iter(|| src.clone().into_iter().count())
}

#[bench]
pub fn clone_10k_and_pop_all(b: &mut Bencher) {
    let src = pos(10_000);
    b.iter(|| {
        let mut set = src.clone();
        while set.pop_first().is_some() {}
        set
    });
}

#[bench]
pub fn clone_10k_and_remove_all(b: &mut Bencher) {
    let src = pos(10_000);
    b.iter(|| {
        let mut set = src.clone();
        while let Some(elt) = set.iter().copied().next() {
            set.remove(&elt);
        }
        set
    });
}

#[bench]
pub fn clone_10k_and_remove_half(b: &mut Bencher) {
    let src = pos(10_000);
    b.iter(|| {
        let mut set = src.clone();
        for i in (2..=10_000 as i32).step_by(2) {
            set.remove(&i);
        }
        assert_eq!(set.len(), 10_000 / 2);
        set
    })
}

set_bench! {intersection_100_neg_vs_100_pos, intersection, count, [neg(100), pos(100)]}
set_bench! {intersection_100_neg_vs_10k_pos, intersection, count, [neg(100), pos(10_000)]}
set_bench! {intersection_100_pos_vs_100_neg, intersection, count, [pos(100), neg(100)]}
set_bench! {intersection_100_pos_vs_10k_neg, intersection, count, [pos(100), neg(10_000)]}
set_bench! {intersection_10k_neg_vs_100_pos, intersection, count, [neg(10_000), pos(100)]}
set_bench! {intersection_10k_neg_vs_10k_pos, intersection, count, [neg(10_000), pos(10_000)]}
set_bench! {intersection_10k_pos_vs_100_neg, intersection, count, [pos(10_000), neg(100)]}
set_bench! {intersection_10k_pos_vs_10k_neg, intersection, count, [pos(10_000), neg(10_000)]}
set_bench! {intersection_random_100_vs_100, intersection, count, [random(100), random(100)]}
set_bench! {intersection_random_100_vs_10k, intersection, count, [random(100), random(10_000)]}
set_bench! {intersection_random_10k_vs_100, intersection, count, [random(10_000), random(100)]}
set_bench! {intersection_random_10k_vs_10k, intersection, count, [random(10_000), random(10_000)]}
set_bench! {intersection_staggered_100_vs_100, intersection, count, stagger(100, 1)}
set_bench! {intersection_staggered_10k_vs_10k, intersection, count, stagger(10_000, 1)}
set_bench! {intersection_staggered_100_vs_10k, intersection, count, stagger(100, 100)}
set_bench! {difference_random_100_vs_100, difference, count, [random(100), random(100)]}
set_bench! {difference_random_100_vs_10k, difference, count, [random(100), random(10_000)]}
set_bench! {difference_random_10k_vs_100, difference, count, [random(10_000), random(100)]}
set_bench! {difference_random_10k_vs_10k, difference, count, [random(10_000), random(10_000)]}
set_bench! {difference_staggered_100_vs_100, difference, count, stagger(100, 1)}
set_bench! {difference_staggered_10k_vs_10k, difference, count, stagger(10_000, 1)}
set_bench! {difference_staggered_100_vs_10k, difference, count, stagger(100, 100)}
set_bench! {is_subset_100_vs_100, is_subset, clone, [pos(100), pos(100)]}
set_bench! {is_subset_100_vs_10k, is_subset, clone, [pos(100), pos(10_000)]}
set_bench! {is_subset_10k_vs_100, is_subset, clone, [pos(10_000), pos(100)]}
set_bench! {is_subset_10k_vs_10k, is_subset, clone, [pos(10_000), pos(10_000)]}
