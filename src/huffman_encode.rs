use crate::huffman_encode::SymbolOrChildren::Children;
use crate::huffman_encode::SymbolOrChildren::Symbol;
use crate::stats::Stats;
use bit_vec::BitVec;
use fnv_rs::FnvHashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

enum SymbolOrChildren<T> {
    Children { left: NodeRef<T>, right: NodeRef<T> },
    Symbol(T),
}

struct HuffNode<T> {
    symbol_or_children: SymbolOrChildren<T>,
    weight: u32,
}

type NodeRef<T> = Box<HuffNode<T>>;
type ByteNode = HuffNode<u8>;
type SymbolMap = FnvHashMap<u8, BitVec>;

impl<T> Ord for HuffNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight) // Note: reversed to make min-priority queue
    }
}
impl<T> PartialOrd for HuffNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> PartialEq for HuffNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}
impl<T> Eq for HuffNode<T> {}

fn tree_to_symbol_table_impl(node: &ByteNode, symbol_map: &mut SymbolMap, bit_rep: BitVec) {
    match &node.symbol_or_children {
        Children { left, right } => {
            let mut left_rep = bit_rep.clone();
            let mut right_rep = bit_rep; // no clone, move bit_rep

            left_rep.push(false);
            right_rep.push(true);

            tree_to_symbol_table_impl(left, symbol_map, left_rep);
            tree_to_symbol_table_impl(right, symbol_map, right_rep);
        }
        Symbol(symbol) => {
            symbol_map.insert(*symbol, bit_rep);
        }
    }
}

fn tree_to_symbol_table(root: &ByteNode) -> SymbolMap {
    let mut symbol_map = SymbolMap::default();
    tree_to_symbol_table_impl(root, &mut symbol_map, BitVec::new());
    return symbol_map;
}

fn build_tree(stats: &Stats) -> ByteNode {
    let mut heap = BinaryHeap::new();
    for (byte, count) in &stats.frequency_map {
        let node = ByteNode { symbol_or_children: Symbol(*byte), weight: *count };
        heap.push(node);
    }

    while heap.len() >= 2 {
        let elm_1 = heap.pop().expect("heap should have 2+ elements");
        let elm_2 = heap.pop().expect("heap should have 1+ elements");

        let node = ByteNode {
            weight: elm_1.weight + elm_2.weight,
            symbol_or_children: Children { left: Box::new(elm_1), right: Box::new(elm_2) },
        };
        heap.push(node);
    }
    assert!(heap.len() == 1, "Only 1 element in heap");
    heap.pop().expect("Only 1 element in heap")
}

pub fn make_symbol_table(stats: &Stats) -> SymbolMap {
    tree_to_symbol_table(&build_tree(stats))
}
