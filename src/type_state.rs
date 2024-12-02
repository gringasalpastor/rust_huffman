use state_shift::{impl_state, type_state};
// use crate::hybrid_hash_map::HybridMapInterface;
use crate::hybrid_hash_map::HybridMapType;
use crate::hybrid_hash_map;
// use std::hash::BuildHasherDefault;
// use fnv_rs::Fnv64;
use std::fmt::Debug;

type FreqMapType<K> = <K as HybridMapType<u32, fnv_rs::FnvBuildHasher>>::Out;


#[type_state(
    states = (Initial, DataType, Delta), 
    slots = (Initial) 
)]

pub struct ColBuilder {
    datatype : ColType,
}

#[derive(Debug)]
struct Col<T> 
where T : hybrid_hash_map::HybridMapType<u32, std::hash::BuildHasherDefault<fnv_rs::Fnv64>>,
FreqMapType<T> : Debug{
    freq_map : FreqMapType<T>,
}

pub trait ColInterface {
    fn f(&mut self);
}

impl ColInterface for Col::<u8>
{
    fn f(&mut self) {
    }

}

#[impl_state]
impl ColBuilder {

    #[require(Initial)]
    pub fn new(datatype: ColType) -> ColBuilder {
        ColBuilder { datatype : datatype}
    }

    #[require(Initial)]
    #[switch_to(Delta)]
    pub fn delta(datatype: ColType) -> ColBuilder {
        ColBuilder { datatype : datatype}
    }

    #[require(A)]
    pub fn build(self) -> impl ColInterface
    {
        match self.datatype
        {
            ColType::U8 => Col::<u8> {
                freq_map: Default::default(),
            }
        }
    }

}

#[derive(Debug)]
pub enum ColType {
    #[allow(unused)]
    U8
}
