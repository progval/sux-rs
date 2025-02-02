/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use core::sync::atomic::{AtomicUsize, Ordering};
use epserde::prelude::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};
use sux::bits::bit_vec::BitVec;

#[test]
fn test_bit_vec() {
    let n = 50;
    let n2 = 100;
    let u = 1000;

    let mut rng = SmallRng::seed_from_u64(0);

    let mut bm = BitVec::new(u);

    for _ in 0..10 {
        let mut values = (0..u).collect::<Vec<_>>();
        let (indices, _) = values.partial_shuffle(&mut rng, n2);

        for i in indices[..n].iter().copied() {
            bm.set(i, true);
        }

        for i in 0..u {
            assert_eq!(bm.get(i), indices[..n].contains(&i));
        }

        for i in indices[n..].iter().copied() {
            bm.set(i, true);
        }

        for i in 0..u {
            assert_eq!(bm.get(i), indices.contains(&i));
        }

        for i in indices[..n].iter().copied() {
            bm.set(i, false);
        }

        for i in 0..u {
            assert_eq!(bm.get(i), indices[n..].contains(&i));
        }

        for i in indices[n..].iter().copied() {
            bm.set(i, false);
        }

        for i in 0..u {
            assert!(!bm.get(i));
        }
    }

    let bm: BitVec<Vec<AtomicUsize>> = bm.into();
    for _ in 0..10 {
        let mut values = (0..u).collect::<Vec<_>>();
        let (indices, _) = values.partial_shuffle(&mut rng, n2);

        for i in indices[..n].iter().copied() {
            bm.set(i, true, Ordering::Relaxed);
        }

        for i in 0..u {
            assert_eq!(bm.get(i, Ordering::Relaxed), indices[..n].contains(&i));
        }

        for i in indices[n..].iter().copied() {
            bm.set(i, true, Ordering::Relaxed);
        }

        for i in 0..u {
            assert_eq!(bm.get(i, Ordering::Relaxed), indices.contains(&i));
        }

        for i in indices[..n].iter().copied() {
            bm.set(i, false, Ordering::Relaxed);
        }

        for i in 0..u {
            assert_eq!(bm.get(i, Ordering::Relaxed), indices[n..].contains(&i));
        }

        for i in indices[n..].iter().copied() {
            bm.set(i, false, Ordering::Relaxed);
        }

        for i in 0..u {
            assert!(!bm.get(i, Ordering::Relaxed));
        }
    }
}

#[test]
fn test_epserde() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut b = BitVec::new(200);
    for i in 0..200 {
        b.set(i, rng.next_u64() % 2 != 0);
    }

    let tmp_file = std::env::temp_dir().join("test_serdes_ef.bin");
    let mut file = std::io::BufWriter::new(std::fs::File::create(&tmp_file).unwrap());
    b.serialize(&mut file).unwrap();
    drop(file);

    let c = <BitVec<Vec<usize>>>::mmap(&tmp_file, epserde::des::Flags::empty()).unwrap();

    for i in 0..200 {
        assert_eq!(b.get(i), c.get(i));
    }
}
