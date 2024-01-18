// Driver to build a CDAWG on a corpus.
// Eventually, this should probably be merged with main.

use anyhow::Result;

use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::cmp::Ord;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Debug;
use std::io::{BufReader, Read};
use std::rc::Rc;
use std::cell::RefCell;

use io::Save;

use clap::Parser;
use std::fs;
use std::mem::size_of;

use kdam::{tqdm, BarExt};

use io;
use super::Args;

use cdawg::Cdawg;
use cdawg::cdawg_edge_weight::CdawgEdgeWeight;
use evaluator::Evaluator;

use graph::avl_graph::edge::Edge;
use graph::avl_graph::node::Node;
use graph::indexing::DefaultIx;
use graph::memory_backing::{DiskBacking, MemoryBacking, RamBacking};
use graph::memory_backing::disk_backing::disk_vec::DiskVec;

use tokenize::{NullTokenIndex, PretrainedTokenizer, TokenIndex, Tokenize};

type N = super::N;
type E = CdawgEdgeWeight<DefaultIx>;

// Confusingly, E here is the token type.
pub fn build_cdawg<Mb>(args: Args, mb: Mb) -> Result<()>
where
    Mb: MemoryBacking<N, CdawgEdgeWeight<DefaultIx>, DefaultIx>,
    Cdawg<N, DefaultIx, Mb>: io::Save,
{
    // TODO: Support token types with more bits?
    let mut index: Box<dyn Tokenize<u16>> = if args.tokenizer == "whitespace" {
        Box::new(TokenIndex::new())
    } else if args.tokenizer == "null" {
        Box::new(NullTokenIndex::new())
    } else {
        Box::new(PretrainedTokenizer::new(&args.tokenizer))
    };

    println!("Sizes:");
    println!("\tIx:\t{}B", size_of::<DefaultIx>());
    println!("\tN:\t{}B", size_of::<N>());
    println!("\tE:\t{}B", size_of::<E>());
    println!("\tNode:\t{}B", size_of::<Node<N, DefaultIx>>());
    println!("\tEdge:\t{}B", size_of::<Edge<E, DefaultIx>>());

    println!("Opening train file...");
    let train_file = fs::File::open(args.train_path.as_str())?;
    let n_bytes = train_file.metadata().unwrap().len();
    let est_n_tokens = (args.tokens_per_byte * (n_bytes as f64)).round() as usize;
    let eval_threshold = if args.n_eval == 0 {
        0
    } else {
        est_n_tokens / args.n_eval
    };
    let buf_size: usize = min(n_bytes.try_into().unwrap(), args.buf_size);

    let test_raw: String = fs::read_to_string(args.test_path.as_str()).expect("Error loading test");
    index.build(&test_raw); // Either the tokenizer must be pretrained or test must contain all tokens!
    let mut test: Vec<_> = index.tokenize(&test_raw);
    // let mut test: Vec<usize> = test_raw.split_whitespace().map(|x| index.add(x)).collect();
    let old_test_len = test.len();
    if args.truncate_test > 0 {
        test = test[0..args.truncate_test].to_vec();
    }
    let mut evaluator = Evaluator::new(&test, args.max_length);
    println!("#(test): {}/{}", test.len(), old_test_len);

    let n_nodes = (args.nodes_ratio * (est_n_tokens as f64)).ceil() as usize;
    let n_edges = (args.edges_ratio * (est_n_tokens as f64)).ceil() as usize;
    let max_length: Option<u64> = if !args.max_state_length.is_negative() {
        Some(args.max_state_length.try_into().unwrap())
    } else {
        None
    };

    // Maintain a DiskVec that we update incrementally (whenever we read a token, set it).
    if args.train_vec_path.is_none() {
        panic!("CDAWG requires train-vec-path");
    }
    println!("# tokens: {}", est_n_tokens);
    println!("Opening train vector...");
    let train_vec: Vec<u16> = Vec::with_capacity(est_n_tokens);
    // let train_vec: DiskVec<u16> = DiskVec::new(&args.train_vec_path.unwrap(), est_n_tokens)?;
    let train_vec_rc = Rc::new(RefCell::new(train_vec));

    let mut cdawg: Cdawg<N, DefaultIx, Mb> =
        Cdawg::with_capacity_mb(train_vec_rc.clone(), mb, n_nodes, n_edges);

    let mut state = cdawg.get_source();
    let mut start: usize = 1;
    let mut idx: usize = 0;
    let mut pbar = tqdm!(total = est_n_tokens);
    let mut train_reader = BufReader::with_capacity(buf_size, train_file);
    let mut buffer = vec![0; buf_size];
    loop {
        let n_bytes_read = train_reader.read(&mut buffer).unwrap();
        if n_bytes_read == 0 {
            break;
        }
        let text = std::str::from_utf8(&buffer);
        let tokens = index.tokenize(text.unwrap());
        for token in &tokens {
            // *token for Vec, token for DiskVec
            let _ = train_vec_rc.borrow_mut().push(*token);
            idx += 1;
            (state, start) = cdawg.update(state, start, idx);
            pbar.update(1);
        }
    }

    eprintln!();
    println!("Completed!");
    println!(
        "  token/byte: {:.2} (tokens={})",
        (idx as f64) / (n_bytes as f64),
        idx
    );
    println!(
        "  node/token: {:.2} (nodes={})",
        (cdawg.node_count() as f64) / (idx as f64),
        cdawg.node_count()
    );
    println!(
        "  edge/token: {:.2} (edges={})",
        (cdawg.edge_count() as f64) / (idx as f64),
        cdawg.edge_count()
    );
    println!("  Balance ratio: {}", cdawg.balance_ratio(1));

    if !args.save_path.is_empty() {
        println!("Saving DAWG...");
        cdawg.save(&args.save_path).unwrap();  // FIXME
        println!("Successfully saved DAWG to {}!", &args.save_path);
    }
    Ok(())
}