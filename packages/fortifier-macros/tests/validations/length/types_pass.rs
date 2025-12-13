use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

use fortifier::{LengthError, Validate, ValidationErrors};
use indexmap::{IndexMap, IndexSet};

#[derive(Validate)]
struct LengthData<'a> {
    #[validate(length(min = 1))]
    r#str: &'a str,
    #[validate(length(min = 1))]
    string: String,
    #[validate(length(min = 1))]
    array: [usize; 0],
    #[validate(length(min = 1))]
    slice: &'a [usize],
    #[validate(length(min = 1))]
    b_tree_map: BTreeMap<usize, usize>,
    #[validate(length(min = 1))]
    b_tree_set: BTreeSet<usize>,
    #[validate(length(min = 1))]
    hash_map: HashMap<usize, usize>,
    #[validate(length(min = 1))]
    hash_set: HashSet<String>,
    #[validate(length(min = 1))]
    index_map: IndexMap<usize, usize>,
    #[validate(length(min = 1))]
    index_set: IndexSet<String>,
    #[validate(length(min = 1))]
    linked_list: LinkedList<String>,
    #[validate(length(min = 1))]
    vec: Vec<usize>,
    #[validate(length(min = 1))]
    vec_deque: VecDeque<usize>,
}

fn main() {
    let data = LengthData {
        r#str: "",
        string: "".to_owned(),
        array: [],
        slice: &[],
        b_tree_map: BTreeMap::default(),
        b_tree_set: BTreeSet::default(),
        hash_map: HashMap::default(),
        hash_set: HashSet::default(),
        index_map: IndexMap::default(),
        index_set: IndexSet::default(),
        linked_list: LinkedList::default(),
        vec: Vec::default(),
        vec_deque: VecDeque::default(),
    };

    assert_eq!(
        data.validate_sync(),
        Err(ValidationErrors::from_iter([
            LengthDataValidationError::Str(LengthError::Min { min: 1, length: 0 }),
            LengthDataValidationError::String(LengthError::Min { min: 1, length: 0 }),
            LengthDataValidationError::Array(LengthError::Min { min: 1, length: 0 }),
            LengthDataValidationError::Slice(LengthError::Min { min: 1, length: 0 }),
            LengthDataValidationError::BTreeMap(LengthError::Min { min: 1, length: 0 }),
            LengthDataValidationError::BTreeSet(LengthError::Min { min: 1, length: 0 }),
            LengthDataValidationError::HashMap(LengthError::Min { min: 1, length: 0 }),
            LengthDataValidationError::HashSet(LengthError::Min { min: 1, length: 0 }),
            LengthDataValidationError::IndexMap(LengthError::Min { min: 1, length: 0 }),
            LengthDataValidationError::IndexSet(LengthError::Min { min: 1, length: 0 }),
            LengthDataValidationError::LinkedList(LengthError::Min { min: 1, length: 0 }),
            LengthDataValidationError::Vec(LengthError::Min { min: 1, length: 0 }),
            LengthDataValidationError::VecDeque(LengthError::Min { min: 1, length: 0 }),
        ]))
    );
}
