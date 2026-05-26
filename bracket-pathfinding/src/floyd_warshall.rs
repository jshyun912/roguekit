use bracket_algorithm_traits::prelude::BaseMap;
#[allow(unused_imports)]
use smallvec::SmallVec;
use std::convert::TryInto;

pub struct FloydWarshallMap {
    pub depth_map: Vec<f32>,
    size_x: usize,
    size_y: usize,
    max_depth: f32,
}

#[allow(dead_code)]
impl FloydWarshallMap {
    /// Construct a new FloydWarshall map, ready to run. You must specify the map size, and link to an implementation
    /// of a BaseMap trait that can generate exits lists. It then builds the map, giving you a result.
    pub fn new<T>(size_x: T, size_y: T, map: &dyn BaseMap, max_depth: f32) -> FloydWarshallMap
    where
        T: TryInto<usize>,
    {
        let sz_x: usize = size_x.try_into().ok().unwrap();
        let sz_y: usize = size_y.try_into().ok().unwrap();
        let result: Vec<f32> = vec![f32::MAX; (sz_x * sz_y) * (sz_x * sz_y)];
        let mut f = FloydWarshallMap {
            depth_map: result,
            size_x: sz_x,
            size_y: sz_y,
            max_depth,
        };
        FloydWarshallMap::build(&mut f, map);
        f
    }

    
    fn idx_helper(start_idx: usize, end_idx: usize, mapsize: usize) -> usize {
        start_idx * mapsize + end_idx
    }

    pub fn build(fm: &mut FloydWarshallMap, map: &dyn BaseMap) {
        let mapsize: usize = fm.size_x * fm.size_y;

        for start_idx in 0..mapsize {
            for end_idx in 0..mapsize {
                let ste_idx = Self::idx_helper(start_idx, end_idx, mapsize);
                fm.depth_map[ste_idx] = fm.max_depth;
            }
        }

        for start_idx in 0..mapsize {
            for (end_idx, depth) in map.get_available_exits(start_idx) {
                let ste_idx = Self::idx_helper(start_idx, end_idx, mapsize);
                fm.depth_map[ste_idx] = depth;
            }
        }

        for mid_idx in 0..mapsize {
            for start_idx in 0..mapsize {
                let stm_idx = Self::idx_helper(start_idx, mid_idx, mapsize);
                for end_idx in 0..mapsize {
                    let ste_idx = Self::idx_helper(start_idx, end_idx, mapsize);
                    let mte_idx = Self::idx_helper(mid_idx, end_idx, mapsize);
                    let new_depth = fm.depth_map[stm_idx] + fm.depth_map[mte_idx];
                    let prev_depth = fm.depth_map[ste_idx];

                    fm.depth_map[ste_idx] = f32::min(new_depth, prev_depth);
                }
            }
        }
    }


    /// Helper for traversing maps as path-finding. Provides the index of the lowest available
    /// exit from the specified position index, or None if there isn't one.
    /// You would use this for pathing TOWARDS a starting node.
    pub fn find_lowest_exit(
        fm: &FloydWarshallMap,
        position: usize,
        map: &dyn BaseMap,
    ) -> Option<usize> {
        let exits = map.get_available_exits(position);
        let mapsize = fm.size_x * fm.size_y;

        if exits.is_empty() {
            return None;
        }

        let mut lowest_depth = fm.max_depth;
        let mut lowest_exit = 0;

        for (exit, _) in exits {
            let pos = Self::idx_helper(position, exit, mapsize);
            if lowest_depth > fm.depth_map[pos] {
                lowest_exit = exit;
                lowest_depth = fm.depth_map[pos]
            }
        }

        Some(lowest_exit)
    }

    /// Helper for traversing maps as path-finding. Provides the index of the highest available
    /// exit from the specified position index, or None if there isn't one.
    /// You would use this for pathing AWAY from a starting node, for example if you are running
    /// away.
    pub fn find_highest_exit(
        fm: &FloydWarshallMap,
        position: usize,
        map: &dyn BaseMap,
    ) -> Option<usize> {
        let exits = map.get_available_exits(position);
        let mapsize = fm.size_x * fm.size_y;

        if exits.is_empty() {
            return None;
        }

        let mut highest_depth = f32::MIN;
        let mut highest_exit = 0;

        for (exit, _) in exits {
            let pos = Self::idx_helper(position, exit, mapsize);
            if highest_depth < fm.depth_map[pos] {
                highest_exit = exit;
                highest_depth = fm.depth_map[pos]
            }
        }

        Some(highest_exit)
    }
}

#[cfg(test)]
mod test {


}
