extern crate tantivy;
extern crate itertools;
extern crate byteorder;
extern crate regex;

use tantivy::core::postings::{VecPostings, intersection};
use tantivy::core::postings::Postings;
use tantivy::core::analyzer::tokenize;
use tantivy::core::serial::*;
use tantivy::core::schema::*;
use tantivy::core::global::*;
use tantivy::core::writer::{IndexWriter, ClosedIndexWriter};
use tantivy::core::directory::{Directory, generate_segment_name, SegmentId};
use std::ops::DerefMut;
use tantivy::core::writer::SimplePostingsWriter;
use tantivy::core::postings::PostingsWriter;
use std::io::{ BufWriter, Write};
use regex::Regex;
use std::convert::From;

#[test]
fn test_intersection() {
    let left = VecPostings::new(vec!(1, 3, 9));
    let right = VecPostings::new(vec!(3, 4, 9, 18));
    let inter = intersection(&left, &right);
    let vals: Vec<DocId> = inter.iter().collect();
    assert_eq!(vals, vec!(3, 9));
}

#[test]
fn test_tokenizer() {
    let words: Vec<&str> = tokenize("hello happy tax payer!").collect();
    assert_eq!(words, vec!("hello", "happy", "tax", "payer"));
}

#[test]
fn test_indexing() {
    let directory = Directory::in_mem();
    {
        let mut index_writer = IndexWriter::open(&directory);
        {
            let mut doc = Document::new();
            doc.set(Field(1), "a b");
            index_writer.add(doc);
        }
        {
            let mut doc = Document::new();
            doc.set(Field(1), "a b c");
            index_writer.add(doc);
        }
        {
            let mut doc = Document::new();
            doc.set(Field(1), "a b c d");
            // TODO make iteration over Fields somehow sorted
            index_writer.add(doc);
        }
        let mut closed_index_writer:  ClosedIndexWriter = index_writer.close();
        let mut term_cursor = closed_index_writer.term_cursor();
        loop {
            if !term_cursor.advance() {
                break;
            }
            show_term(&term_cursor);
        }
        assert!(false);
    }
    {
        // TODO add index opening stuff
        // let index_reader = IndexReader::open(&directory);
    }
}


fn show_term<'a, T: TermCursor<'a>>(term_cursor: &T) {
    println!("{:?}", term_cursor.get_term());
    let doc_cursor = term_cursor.doc_cursor();
    for doc in doc_cursor {
        println!("doc({})", doc);
    }
}

// fn show_doc_cursor<'a, D: DocCursor>(mut doc_cursor: D) {
//     loop {
//         match doc_cursor.next() {
//             Some(doc) => {
//                 println!("       {}", doc);
//             },
//             None =>  {
//                 break;
//             }
//         }
//     }
// }


#[test]
fn test_new_segment() {
    let SegmentId(segment_name) = generate_segment_name();
    let segment_ptn = Regex::new(r"^_[a-z0-9]{8}$").unwrap();
    assert!(segment_ptn.is_match(&segment_name));
}