use bracket_algorithm_traits::prelude::BaseMap;
#[cfg(feature = "threaded")]
use rayon::prelude::*;
#[allow(unused_imports)]
use smallvec::SmallVec;
use std::convert::TryInto;

pub struct FloydWarshallMap {
    pub map: Vec<f32>,
    size_x: usize,
    size_y: usize,
    max_depth: f32,
}

#[allow(dead_code)]
impl FloydWarshallMap {
    pub fn new<T>(
        size_x: T,
        size_y: T,
        map: &dyn BaseMap,
        max_depth: f32,
    ) -> FloydWarshallMap
    where
        T: TryInto<usize>,
    {
        let sz_x: usize = size_x.try_into().ok().unwrap();
        let sz_y: usize = size_y.try_into().ok().unwrap();
        let result: Vec<f32> = vec![f32::MAX; sz_x * sz_y];
        let mut f = FloydWarshallMap {
            map: result,
            size_x: sz_x,
            size_y: sz_y,
            max_depth,
        };
        //FloydWarshallMap::build(&mut f, starts, map);
        f
    }
}