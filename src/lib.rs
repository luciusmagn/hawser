#![no_std]
use core::fmt;
use core::cmp::{Eq, PartialEq};
use core::slice::{from_raw_parts, SliceExt};
use core::str::{FromStr, from_utf8, Utf8Error};
use core::ops::{Drop, Add, AddAssign, Index, Deref};

#[repr(C)]
pub struct buffer_t {
	len: usize,
	alloc: *mut u8,
	data: *mut u8,
}

pub struct Hawser(*mut buffer_t);
#[derive(Debug, Eq, PartialEq)]
pub enum HawserError {
	CouldntResize,
	SubStringNotFound,
	BadRange,
	CouldntCompact,
}
pub type String = Hawser;
pub type StringError = HawserError;

extern "C" {
	pub fn buffer_new() -> *mut buffer_t; //
	pub fn buffer_new_with_size(n: usize) -> *mut buffer_t; //
	pub fn buffer_new_with_string(c: *const u8) -> *mut buffer_t; //
	pub fn buffer_new_with_string_length(str: *mut u8, len: usize) -> *mut buffer_t; //
	pub fn buffer_new_with_copy(str: *const u8) -> *mut buffer_t; //
	pub fn buffer_size(str: *mut buffer_t) -> usize; //
	pub fn buffer_length(str: *mut buffer_t) -> usize; //
	pub fn buffer_free(str: *mut buffer_t); //
	pub fn buffer_prepend(str: *mut buffer_t, str: *const u8) -> i32; //
	pub fn buffer_append(str: *mut buffer_t, str: *const u8) -> i32; //
	pub fn buffer_prepend_n(str: *mut buffer_t, str: *const u8, len: usize) -> i32; //
	pub fn buffer_append_n(str: *mut buffer_t, str: *const u8, len: usize) -> i32; //
	pub fn buffer_equals(str: *mut buffer_t, other: *mut buffer_t) -> i32; //
	pub fn buffer_indexof(str: *mut buffer_t, str: *const u8) -> isize; //
	pub fn buffer_slice(str: *mut buffer_t, from: usize, to: usize) -> *mut buffer_t; //
	pub fn buffer_compact(str: *mut buffer_t) -> isize; //
	pub fn buffer_fill(str: *mut buffer_t, c: u32); //
	pub fn buffer_clear(str: *mut buffer_t); //
	pub fn buffer_trim_left(str: *mut buffer_t); //
	pub fn buffer_trim_right(str: *mut buffer_t); //
	pub fn buffer_trim(str: *mut buffer_t); //
}

impl Hawser {
	pub fn new() -> Hawser {
		unsafe { Hawser(buffer_new()) }
	}

	pub fn with_size(size: usize) -> Hawser {
		unsafe { Hawser(buffer_new_with_size(size)) }
	}

	pub fn with_cstr(n: *mut u8) -> Hawser {
		unsafe { Hawser(buffer_new_with_string(n)) }
	}

	pub fn with_cstr_len(n: *mut u8, len: usize) -> Hawser {
		unsafe { Hawser(buffer_new_with_string_length(n, len)) }
	}

	pub fn with_cstr_copy(n: *const u8) -> Hawser {
		unsafe { Hawser(buffer_new_with_copy(n)) }
	}

	pub fn size(&self) -> usize {
		unsafe { buffer_size(self.0) }
	}

	pub fn len(&self) -> usize {
		unsafe { buffer_length(self.0) }
	}

	pub fn prepend_cstr(&mut self, str: *const u8) -> Result<(), HawserError> {
		unsafe {
			if buffer_prepend(self.0, str) == 0 {
				Ok(())
			} else {
				Err(HawserError::CouldntResize)
			}
		}
	}

	pub fn prepend_cstr_n(&mut self, str: *const u8, len: usize) -> Result<(), HawserError> {
		unsafe {
			if buffer_prepend_n(self.0, str, len) == 0 {
				Ok(())
			}
			else {
				Err(HawserError::CouldntResize)
			}
		}
	}

	pub fn append_cstr(&mut self, str: *const u8) -> Result<(), HawserError> {
		unsafe {
			if buffer_append(self.0, str) == 0 {
				Ok(())
			} else {
				Err(HawserError::CouldntResize)
			}
		}
	}

	pub fn append_cstr_n(&mut self, str: *const u8, len: usize) -> Result<(), HawserError> {
		unsafe {
			if buffer_append_n(self.0, str, len) == 0 {
				Ok(())
			}
			else {
				Err(HawserError::CouldntResize)
			}
		}
	}

	pub fn prepend(&mut self, str: &str) -> Result<(), HawserError> {
		self.prepend_cstr_n(str.as_ptr(), str.len())
	}

	pub fn append(&mut self, str: &str) -> Result<(), HawserError> {
		self.append_cstr_n(str.as_ptr(), str.len())
	}

	pub fn index_of(&self, str: *const u8) -> Result<usize, HawserError> {
		match unsafe { buffer_indexof(self.0, str) } {
			x if x >= 0 => Ok(x as usize),
			_ => Err(HawserError::SubStringNotFound),
		}
	}

	pub fn slice(&self, from: usize, to: usize) -> Result<Hawser, HawserError> {
		match unsafe { buffer_slice(self.0, from, to) } {
			x if !x.is_null() => Ok(Hawser(x)),
			_ => Err(HawserError::BadRange),
		}
	}

	pub fn compact(&mut self) -> Result<usize, HawserError> {
		match unsafe { buffer_compact(self.0) } {
			x if x >= 0 => Ok(x as usize),
			_ => Err(HawserError::CouldntCompact),
		}
	}

	pub fn fill(&mut self, c: char) {
		unsafe { buffer_fill(self.0, c as u32) }
	}

	pub fn clear(&mut self) {
		unsafe { buffer_clear(self.0) }
	}

	pub fn trim_left(&mut self) {
		unsafe { buffer_trim_left(self.0) }
	}

	pub fn trim_right(&mut self) {
		unsafe { buffer_trim_right(self.0) }
	}

	pub fn trim(&mut self) {
		unsafe { buffer_trim(self.0) }
	}

	pub fn to_str(&self) -> Result<&str, Utf8Error> {
		unsafe {
			let len = (*self.0).len;
			from_utf8(from_raw_parts((*self.0).data, len))
		}
	}
}

impl<'a> From<&'a str> for Hawser {
	fn from(str: &str) -> Hawser {
		str.parse().unwrap()
	}
}

impl From<*const u8> for Hawser {
	fn from(str: *const u8) -> Hawser {
		Hawser::with_cstr_copy(str)
	}
}

impl From<*mut u8> for Hawser {
	fn from(str: *mut u8) -> Hawser {
		Hawser::with_cstr(str)
	}
}

impl<'a> From<&'a [u8]> for Hawser {
	fn from(slc: &'a [u8]) -> Hawser {
		Hawser::with_cstr_copy(slc.as_ptr())
	}
}

impl Drop for Hawser {
	fn drop(&mut self) {
		unsafe { buffer_free(self.0) }
	}
}

impl Eq for Hawser {}
impl PartialEq for Hawser {
	fn eq(&self, other: &Hawser) -> bool {
		unsafe { buffer_equals(self.0, other.0) == 1 }
	}
}

impl FromStr for Hawser {
	type Err = HawserError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut ss = Hawser::with_size(64);
		match ss.append(s) {
			Ok(_) => Ok(ss),
			Err(e) => Err(e),   		
		}
	}
}

impl fmt::Display for Hawser {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.to_str().unwrap_or("<corrupted>"))
	}
}

impl fmt::Debug for Hawser {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.to_str().unwrap_or("<corrupted>"))
	}
}

impl<T> Add<T> for Hawser
	where T: Into<Hawser>
{
	type Output = Hawser;
	fn add(self, other: T) -> Self::Output {
		let mut new = unsafe { Hawser::with_cstr_copy((*self.0).data) };
		new.append(&other.into())
			.expect("invalid char pointer");
		new
	}
}

impl<T> AddAssign<T> for Hawser
	where T: Into<Hawser>
{
	fn add_assign(&mut self, other: T) {
		self.append(&other.into())
			.expect("invalid char pointer")
	}
}

impl Index<usize> for Hawser {
	type Output = u8;
	fn index(&self, idx: usize) -> &Self::Output {
		let slc = unsafe { from_raw_parts((*self.clone().0).data, self.len()) };
		&slc[idx]
	}
}

impl Clone for Hawser {
	fn clone(&self) -> Hawser {
		match self.len() {
			0 => Hawser::new(),
			x => self.slice(0, x - 1).unwrap_or("invalid_data".into()),
		}
	}
}

impl Deref for Hawser {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		&self.to_str().unwrap_or("invalid_data")[..self.len()]
	}
}

impl SliceExt {
	type Item = char;
	fn split_at(&self, mid: usize) -> (&[Self::Item], &[Self::Item]) {
		self.to_str().split_at(mid)
	}

	fn iter(&self) -> Iter<Self::Item> {
		self.to_str().iter()
	}

	fn split<P>(&self, pred: P) -> Split<Self::Item, P>
	where
		P: FnMut(&Self::Item) -> bool
	{
		self.to_str().split(pred)
	}
	fn rsplit<P>(&self, pred: P) -> RSplit<Self::Item, P>
	where
		P: FnMut(&Self::Item) -> bool
	{
		self.to_str().rsplit(pred)
	}
	fn splitn<P>(&self, n: usize, pred: P) -> SplitN<Self::Item, P>
	where
		P: FnMut(&Self::Item) -> bool
	{
		self.to_str().splitn(pred)
	}
	fn rsplitn<P>(&self, n: usize, pred: P) -> RSplitN<Self::Item, P>
	where
		P: FnMut(&Self::Item) -> bool
	{
		self.to_str().rsplitn(pred)
	}
	fn windows(&self, size: usize) -> Windows<Self::Item> {
		self.to_str().windows(size)
	}

	fn chunks(&self, size: usize) -> Chunks<Self::Item> {
		self.to_str().chunks(size)
	}
	fn get<I>(&self, index: I) -> Option<&I::Output>
	where
		I: SliceIndex<Self>
	{
		self.to_str().get(index)
	}

	fn first(&self) -> Option<&Self::Item> {
		self.to_str().first()
	}

	fn split_first(&self) -> Option<(&Self::Item, &[Self::Item])> {
		self.to_str().split_first()
	}

	fn split_last(&self) -> Option<(&Self::Item, &[Self::Item])> {
		self.to_str().split_last()
	}

	fn last(&self) -> Option<&Self::Item> {
		self.to_str().last()
	}
	unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
	where
		I: SliceIndex<Self>
	{
		self.to_str().get_unchecked(index)
	}
	fn as_ptr(&self) -> *const Self::Item;
	fn binary_search<Q: ?Sized>(&self, x: &Q) -> Result<usize, usize>
	where
		Self::Item: Borrow<Q>,
		Q: Ord;
	fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
	where
		F: FnMut(&'a Self::Item) -> Ordering;
	fn binary_search_by_key<'a, B, F, Q: ?Sized>(
		&'a self, 
		b: &Q, 
		f: F
	) -> Result<usize, usize>
	where
		F: FnMut(&'a Self::Item) -> B,
		B: Borrow<Q>,
		Q: Ord;
	fn len(&self) -> usize;
	fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
	where
		I: SliceIndex<Self>;
	fn iter_mut(&mut self) -> IterMut<Self::Item>;
	fn first_mut(&mut self) -> Option<&mut Self::Item>;
	fn split_first_mut(
		&mut self
	) -> Option<(&mut Self::Item, &mut [Self::Item])>;
	fn split_last_mut(&mut self) -> Option<(&mut Self::Item, &mut [Self::Item])>;
	fn last_mut(&mut self) -> Option<&mut Self::Item>;
	fn split_mut<P>(&mut self, pred: P) -> SplitMut<Self::Item, P>
	where
		P: FnMut(&Self::Item) -> bool;
	fn rsplit_mut<P>(&mut self, pred: P) -> RSplitMut<Self::Item, P>
	where
		P: FnMut(&Self::Item) -> bool;
	fn splitn_mut<P>(&mut self, n: usize, pred: P) -> SplitNMut<Self::Item, P>
	where
		P: FnMut(&Self::Item) -> bool;
	fn rsplitn_mut<P>(&mut self, n: usize, pred: P) -> RSplitNMut<Self::Item, P>
	where
		P: FnMut(&Self::Item) -> bool;
	fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<Self::Item>;
	fn swap(&mut self, a: usize, b: usize);
	fn split_at_mut(
		&mut self, 
		mid: usize
	) -> (&mut [Self::Item], &mut [Self::Item]);
	fn reverse(&mut self);
	unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
	where
		I: SliceIndex<Self>;
	fn as_mut_ptr(&mut self) -> *mut Self::Item;
	fn contains(&self, x: &Self::Item) -> bool
	where
		Self::Item: PartialEq;
	fn starts_with(&self, needle: &[Self::Item]) -> bool
	where
		Self::Item: PartialEq;
	fn ends_with(&self, needle: &[Self::Item]) -> bool
	where
		Self::Item: PartialEq;
	fn rotate(&mut self, mid: usize);
	fn clone_from_slice(&mut self, src: &[Self::Item])
	where
		Self::Item: Clone;
	fn copy_from_slice(&mut self, src: &[Self::Item])
	where
		Self::Item: Copy;
	fn swap_with_slice(&mut self, src: &mut [Self::Item]);
	fn sort_unstable(&mut self)
	where
		Self::Item: Ord;
	fn sort_unstable_by<F>(&mut self, compare: F)
	where
		F: FnMut(&Self::Item, &Self::Item) -> Ordering;
	fn sort_unstable_by_key<B, F>(&mut self, f: F)
	where
		F: FnMut(&Self::Item) -> B,
		B: Ord;
}

#[test]
fn create_string() {
	let string = Hawser::new();
	let string2 = Hawser::from("hello");

	assert!(string != string2);

	let string3 = Hawser::from("hello");

	assert!(string2 == string3);
}

#[test]
fn append_prepend() {
	let mut string1 = Hawser::from("hello ");
	let string2 = Hawser::from("hello world");

	assert!(string1.append("world").is_ok());

	assert_eq!(string1, string2);

	let mut string3 = Hawser::from(" world");
	assert!(string3.prepend("hello").is_ok());

	assert_eq!(string3, string2);
}

#[test]
fn deref() {
	let h = Hawser::from("howdy");

	fn test(s: &str) {
		assert_eq!(s, "howdy");
	}

	test(&h);
}

#[test]
fn index() {
	let h = Hawser::from("howdy");

	assert_eq!(h[1] as char, 'o');
}

#[test]
fn concat() {
	let mut ho = Hawser::from("ho");
	let wdy = Hawser::from("wdy");

	assert_eq!(ho + wdy, Hawser::from("howdy"));
}

#[test]
fn assign_append() {
	let mut ho = Hawser::from("ho");
	ho += "wdy";
	ho += "!";

	assert_eq!(ho, Hawser::from("howdy!"));
}

#[test]
fn iter_through() {
	let howdy = Hawser::from("howdy");
	let other = Hawser::from("");
	for c in howdy.iter() {
		other.append((c as char).into().unwrap());
	}
}
