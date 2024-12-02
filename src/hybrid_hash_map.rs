use num_traits::int::PrimInt;
use std::collections::HashMap;
// use std::default::Default;
// use std::hash::RandomState;
use num_traits::AsPrimitive;
use num_traits::FromPrimitive;
use num_traits::Num;
use std::fmt::Debug;

pub trait SmallPrim
where
    Self: AsPrimitive<Self::UnsignedType>,
    Self::UnsignedType: Copy + AsPrimitive<usize>,
{
    type UnsignedType;

    fn to_usize(&self) -> usize {
        // NOTE: First `as_()` converts to UnsignedType to avoid sign extension of negitive values
        // And the second `as_()` coverts to usize from the UnsignedType
        self.as_().as_()
    }
}

impl SmallPrim for u8 {
    type UnsignedType = u8;
}
impl SmallPrim for i8 {
    type UnsignedType = u8;
}
impl SmallPrim for u16 {
    type UnsignedType = u16;
}
impl SmallPrim for i16 {
    type UnsignedType = u16;
}

pub trait HybridMapInterface<K, V, S> {
    fn bump(&mut self, key: K);
}

impl<K, V, S> HybridMapInterface<K, V, S> for TableMap<K, V, S>
where
    K: SmallPrim,
    V: Num + std::ops::AddAssign,
{
    fn bump(&mut self, key: K) {
        self.table[key.to_usize()] += V::one();
    }
}

impl<V, S> HybridMapInterface<u8, V, S> for HybridHashMap<u8, V, S>
where
    V: num_traits::One + std::ops::AddAssign + num_traits::Zero,
    S: std::hash::BuildHasher,
{
    fn bump(&mut self, key: u8) {
        if key < self.table_max_val {
            self.table[key as usize] += V::one();
        } else {
            let stat_entry = self.hash_map.entry(key).or_insert(V::zero());
            *stat_entry += V::one();
        }
    }
}

impl<V, S> HybridMapInterface<u16, V, S> for HybridHashMap<u16, V, S>
where
    V: num_traits::One + std::ops::AddAssign + num_traits::Zero,
    S: std::hash::BuildHasher,
{
    fn bump(&mut self, key: u16) {
        if key < self.table_max_val {
            self.table[key as usize] += V::one();
        } else {
            let stat_entry = self.hash_map.entry(key).or_insert(V::zero());
            *stat_entry += V::one();
        }
    }
}

impl<V, S> HybridMapInterface<u32, V, S> for HybridHashMap<u32, V, S>
where
    V: num_traits::One + std::ops::AddAssign + num_traits::Zero,
    S: std::hash::BuildHasher,
{
    fn bump(&mut self, key: u32) {
        if key < self.table_max_val {
            self.table[key as usize] += V::one();
        } else {
            let stat_entry = self.hash_map.entry(key).or_insert(V::zero());
            *stat_entry += V::one();
        }
    }
}

impl<V, S> HybridMapInterface<u64, V, S> for HybridHashMap<u64, V, S>
where
    V: num_traits::One + std::ops::AddAssign + num_traits::Zero,
    S: std::hash::BuildHasher,
{
    fn bump(&mut self, key: u64) {
        if key < self.table_max_val {
            self.table[key as usize] += V::one();
        } else {
            let stat_entry = self.hash_map.entry(key).or_insert(V::zero());
            *stat_entry += V::one();
        }
    }
}

impl<V, S> HybridMapInterface<i8, V, S> for HybridHashMap<i8, V, S>
where
    V: num_traits::One + std::ops::AddAssign + num_traits::Zero,
    S: std::hash::BuildHasher,
{
    fn bump(&mut self, key: i8) {
        if key < self.table_max_val && key >= self.table_min_val {
            self.table[(key as i16 - self.table_min_val as i16) as usize] += V::one();
        } else {
            let stat_entry = self.hash_map.entry(key).or_insert(V::zero());
            *stat_entry += V::one();
        }
    }
}

impl<V, S> HybridMapInterface<i16, V, S> for HybridHashMap<i16, V, S>
where
    V: num_traits::One + std::ops::AddAssign + num_traits::Zero,
    S: std::hash::BuildHasher,
{
    fn bump(&mut self, key: i16) {
        if key < self.table_max_val && key >= self.table_min_val {
            self.table[(key as i32 - self.table_min_val as i32) as usize] += V::one();
        } else {
            let stat_entry = self.hash_map.entry(key).or_insert(V::zero());
            *stat_entry += V::one();
        }
    }
}

impl<V, S> HybridMapInterface<i32, V, S> for HybridHashMap<i32, V, S>
where
    V: num_traits::One + std::ops::AddAssign + num_traits::Zero,
    S: std::hash::BuildHasher,
{
    fn bump(&mut self, key: i32) {
        if key < self.table_max_val && key >= self.table_min_val {
            self.table[(key as i64 - self.table_min_val as i64) as usize] += V::one();
        } else {
            let stat_entry = self.hash_map.entry(key).or_insert(V::zero());
            *stat_entry += V::one();
        }
    }
}

impl<V, S> HybridMapInterface<i64, V, S> for HybridHashMap<i64, V, S>
where
    V: num_traits::One + std::ops::AddAssign + num_traits::Zero + Debug,
    S: std::hash::BuildHasher,
{
    fn bump(&mut self, key: i64) {
        // println!("key={:?}", key);
        // println!("table_max_val={:?}", self.table_max_val);
        // println!("table_min_val={:?}", self.table_min_val);
        // println!("diff={:?}", (key as i128 - self.table_min_val as i128));
        // println!("as usize={:?}", (key as i128 - self.table_min_val as i128) as usize);
        // println!("table={:?}", self.table);

        if key < self.table_max_val && key >= self.table_min_val {
            self.table[(key as i128 - self.table_min_val as i128) as usize] += V::one();
        } else {
            let stat_entry = self.hash_map.entry(key).or_insert(V::zero());
            *stat_entry += V::one();
        }
    }
}

pub trait HybridMapType<V, S> {
    type Out;
}

impl<V, S> HybridMapType<V, S> for u8 {
    type Out = TableMap<u8, V, S>;
}

impl<V, S> HybridMapType<V, S> for i8 {
    type Out = TableMap<i8, V, S>;
}

impl<V, S> HybridMapType<V, S> for u16 {
    type Out = TableMap<u16, V, S>;
}

impl<V, S> HybridMapType<V, S> for i16 {
    type Out = TableMap<i16, V, S>;
}

impl<V, S> HybridMapType<V, S> for u32 {
    type Out = HybridHashMap<u32, V, S>;
}

impl<V, S> HybridMapType<V, S> for i32 {
    type Out = HybridHashMap<i32, V, S>;
}

impl<V, S> HybridMapType<V, S> for u64 {
    type Out = HybridHashMap<u64, V, S>;
}

impl<V, S> HybridMapType<V, S> for i64 {
    type Out = HybridHashMap<i64, V, S>;
}

// pub fn make_map<K, V, S>() -> impl HybridMapInterface<K, V, S>
pub fn make_map<K, V, S>() -> <K as HybridMapType<V, S>>::Out
where
    K: HybridMapType<V, S>,
    <K as HybridMapType<V, S>>::Out: std::default::Default,
    <K as HybridMapType<V, S>>::Out: HybridMapInterface<K, V, S>,
{
    <K as HybridMapType<V, S>>::Out::default()
}

use std::marker::PhantomData;

#[derive(Debug)]
pub struct TableMap<K, V, S> {
    table: Vec<V>,

    x: PhantomData<K>,
    y: PhantomData<S>,
}

impl<K, V, S> Default for TableMap<K, V, S>
where
    V: std::default::Default + std::clone::Clone,
{
    fn default() -> TableMap<K, V, S> {
        let size: usize = 2usize.pow(std::mem::size_of::<K>() as u32 * 8);
        TableMap::<K, V, S> { table: vec![V::default(); size], x: Default::default(), y: Default::default() }
    }
}

#[derive(Debug)]
pub struct HybridHashMap<K, V, S> {
    pub table: Vec<V>,
    table_max_val: K,
    table_min_val: K,
    pub hash_map: HashMap<K, V, S>,
}

impl<K, V, S> Default for HybridHashMap<K, V, S>
where
    V: std::default::Default + std::clone::Clone,
    S: std::default::Default,
    K: PrimInt + FromPrimitive,
{
    fn default() -> HybridHashMap<K, V, S> {
        let mut table_size: usize = 2usize.pow(std::mem::size_of::<K>().min(2) as u32 * 8);

        let is_signed = K::min_value() < K::zero();
        let (table_min_val, table_max_val): (K, K) = match std::mem::size_of::<K>() {
            0 => {
                unreachable!("non-zero size type expected")
            }
            1 => match is_signed {
                true => (K::from_i8(i8::MIN).expect("K is i8"), K::from_i8(i8::MAX).expect("K is i8")),
                false => (K::from_u8(u8::MIN).expect("K is u8"), K::from_u8(u8::MAX).expect("K is u8")),
            },
            2.. => match is_signed {
                // true => {
                //     table_size = 0;
                //     (K::from_i16(0).expect("K is i16+"), K::from_i16(0).expect("K is i16+"))
                // }
                // false => {
                //     table_size = 0;
                //     (K::from_u16(0).expect("K is u16+"), K::from_u16(0).expect("K is u16+"))
                true => {
                    table_size = 4000;
                    (K::from_i16(-1999).expect("K is i16+"), K::from_i16(2000).expect("K is i16+"))
                }
                false => {
                    table_size = 1024;
                    (K::from_u16(0).expect("K is u16+"), K::from_u16(512).expect("K is u16+"))
                } // true => (K::from_i16(i16::MIN).expect("K is i16+"), K::from_i16(i16::MAX).expect("K is i16+")),
                  // false => (K::from_u16(u16::MIN).expect("K is u16+"), K::from_u16(u16::MAX).expect("K is u16+")),
            },
        };

        HybridHashMap::<K, V, S> {
            table_min_val: table_min_val,
            table_max_val: table_max_val,
            table: vec![V::default(); table_size],
            hash_map: Default::default(),
        }
    }
}
