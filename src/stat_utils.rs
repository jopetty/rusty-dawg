use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::fmt::Debug;

use dawg::Dawg;
use graph::indexing::{NodeIndex, DefaultIx};
use weight::Weight;
use graph::memory_backing::MemoryBacking;
use graph::avl_graph::node::NodeRef;

pub fn get_entropy<E, W, Mb>(dawg: &Dawg<E, W, DefaultIx, Mb>, state: NodeIndex) -> f64
where
    E: Eq + Ord + Serialize + for<'a> Deserialize<'a> + Copy + Debug,
    W: Weight + Serialize + for<'a> Deserialize<'a> + Clone,
    Mb: MemoryBacking<W, E, DefaultIx>,
{
    let denom = dawg.get_node(state).get_count();
    let mut sum_num = 0;
    let mut sum_prob = 0.;
    for next_state in dawg.get_graph().neighbors(state) {
        let num = dawg.get_node(next_state).get_count();
        if num > 0 {
            let prob = (num as f64) / (denom as f64);
            sum_prob -= prob * prob.log2();
            sum_num += num;
        }
    }
    if denom - sum_num > 0 {
        // Missing probability mass corresponding to <eos>
        let missing = ((denom - sum_num) as f64) / (denom as f64);
        sum_prob -= missing * missing.log2();
    }
    sum_prob
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use dawg::Dawg;
    use stat_utils::*;
    use tokenize::{TokenIndex, Tokenize};
    use weight::DefaultWeight;

    use graph::indexing::NodeIndex;

    #[test]
    fn test_get_entropy() {
        let mut dawg: Dawg<char, DefaultWeight> = Dawg::new();
        dawg.build(&['a', 'b']);
        // Approximately log_2(3)
        assert_eq!(get_entropy(&dawg, NodeIndex::new(0)), 1.584962500721156);
        assert_eq!(get_entropy(&dawg, NodeIndex::new(1)), 0.);
        assert_eq!(get_entropy(&dawg, NodeIndex::new(2)), 0.);
    }
}