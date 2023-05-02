//! # Traits
//! This modules contains basic traits related to succinct data structures.
//! The train `Length` provides information about the length of the
//! underlying bit vector, independently of its implementation.
//! 
//! Traits are collected into a module so you can do `use sux::traits::*;`
//! for ease of use. 

/// A trait specifying abstractly the length of the bit vector underlying
/// a succint data structure.
pub trait BitLength {
	/// Return the length in bits of the underlying bit vector.
	fn len(&self) -> usize;
}

/// Rank over a bit vector.
pub trait Rank: BitLength {
	/// Return the number of ones preceding the specified position.
	/// 
	/// # Arguments
	/// * `pos` : `usize` - The position to query.
	fn rank(&self, pos: usize) -> usize {
		unsafe { self.rank_unchecked(pos.min(self.len())) }
	}

	/// Return the number of ones preceding the specified position.
	/// 
	/// # Arguments
	/// * `pos` : `usize` - The position to query, which must be between 0 (included ) and the [length of the underlying bit vector](`Length::len`) (included).
	unsafe fn rank_unchecked(&self, pos: usize) -> usize;
}

/// Rank zeros over a bit vector.
pub trait RankZero: Rank + BitLength {
	/// Return the number of zeros preceding the specified position.
	/// 
	/// # Arguments
	/// * `pos` : `usize` - The position to query.
	fn rank_zero(&self, pos: usize) -> usize {
		pos - self.rank(pos)
	}
	/// Return the number of zeros preceding the specified position.
	/// 
	/// # Arguments
	/// * `pos` : `usize` - The position to query, which must be between 0 and the [length of the underlying bit vector](`Length::len`) (included).
	unsafe fn rank_zero_unchecked(&self, pos: usize) -> usize {
		pos - self.rank_unchecked(pos)
	}
}
/// Select over a bit vector.
pub trait Select: BitLength {
	/// Return the position of the one of given rank.
	/// 
	/// # Arguments
	/// * `rank` : `usize` - The rank to query. If there is no
	/// one of given rank, this function return `None`.
	fn select(&self, rank: usize) -> Option<usize>;

	/// Return the position of the one of given rank.
	/// 
	/// # Arguments
	/// * `rank` : `usize` - The rank to query, which must be
	/// between zero (included) and the number of ones in the underlying bit vector (excluded).
	unsafe fn select_unchecked(&self, rank: usize) -> usize;
}


/// Select zeros over a bit vector.
pub trait SelectZero: BitLength {
	/// Return the position of the zero of given rank.
	/// 
	/// # Arguments
	/// * `rank` : `usize` - The rank to query. If there is no
	/// zero of given rank, this function return `None`.
	fn select_zero(&self, i: usize) -> Option<usize>;

	/// Return the position of the zero of given rank.
	/// 
	/// # Arguments
	/// * `rank` : `usize` - The rank to query, which must be
	/// between zero (included) and the number of zeroes in the underlying bit vector (excluded).
	unsafe fn select_zero_unchecked(&self, i: usize) -> usize;
}
