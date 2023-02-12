

#[test]
fn test_levenstein_distance() {
    // test levenstein_distance function in src/lib.rs
    assert_eq!(spell_corrector::levenstein_distance("kitten", "sitting"), 3);
    assert_eq!(spell_corrector::levenstein_distance("kitten", "kitten"), 0);
    assert_eq!(spell_corrector::levenstein_distance("kitten", "kittens"), 1);
}

#[test]
fn test_closest() {
    // test closest function in src/lib.rs
    let dictionary = spell_corrector::read_txt("count_1w.txt");
    assert_eq!(spell_corrector::closest("elephnt", dictionary), "elephant");
    assert_eq!(spell_corrector::closest("kitten", vec!["kitten".to_string()]), "kitten");
    assert_eq!(spell_corrector::closest("kitten", vec!["kittens".to_string()]), "kittens");
}

#[test]
fn test_read_txt() {
    // test read_txt function in src/lib.rs
    let dictionary = spell_corrector::read_txt("count_1w.txt");
    assert_eq!(dictionary.len(), 333333);
    assert_eq!(dictionary[0], "the");
    assert_eq!(dictionary[333332], "zzyzx");
}