use iota_task::*;
use float_cmp::ApproxEqUlps;

#[test]
fn good_database() {
    let file = std::fs::File::open("tests/good_database.txt")
        .expect("Could not open file");

    let dag = parse_database(file);

    assert!(avg_depth(&dag).approx_eq_ulps(&1.333333, 3));
    assert!(avg_txs_depth(&dag).approx_eq_ulps(&2.5, 1));
    assert!(avg_in_refs(&dag).approx_eq_ulps(&1.6666666, 1));
    assert_eq!(total_tips(&dag), 2);

    assert_eq!(None,    dag[Index::new(0)].timestamp);
    assert_eq!(Some(0), dag[Index::new(1)].timestamp);
    assert_eq!(Some(0), dag[Index::new(2)].timestamp);
    assert_eq!(Some(1), dag[Index::new(3)].timestamp);
    assert_eq!(Some(3), dag[Index::new(4)].timestamp);
    assert_eq!(Some(2), dag[Index::new(5)].timestamp);
}

#[test]
fn not_bipartite() {
    let file = std::fs::File::open("tests/good_database.txt")
        .expect("Could not open file");

    let dag = parse_database(file);

    assert_ne!(true, is_bipartite(&dag));
}

#[test]
fn bipartite() {
    let file = std::fs::File::open("tests/bipartite.txt")
        .expect("Could not open file");

    let dag = parse_database(file);

    assert_eq!(true, is_bipartite(&dag));
}

#[test]
fn zero_nodes() {
    let file = std::fs::File::open("tests/zero_nodes.txt")
        .expect("Could not open file");

    let dag = parse_database(file);

    assert!(avg_depth(&dag).approx_eq_ulps(&0., 1));
    assert!(avg_txs_depth(&dag).is_nan());
    assert!(avg_in_refs(&dag).approx_eq_ulps(&0., 1));
    assert_eq!(total_tips(&dag), 1);
}

#[test]
#[should_panic(expected = "Could not open file")]
fn file_does_not_exist() {
    process_file("some_random_name");
}

#[test]
fn process_good_file() {
    process_file("tests/good_database.txt");
    // TODO capture the output and parse it. But I am a little bit tired to do this
}

#[test]
#[should_panic(expected = "File is empty")]
fn empty_file() {
    let file = std::fs::File::open("tests/empty_file.txt")
        .expect("Could not open file");

    let _dag = parse_database(file);
}

#[test]
#[should_panic(expected = "N is not an int")]
fn n_is_not_an_int() {
    let file = std::fs::File::open("tests/n_is_not_an_int.txt")
        .expect("Could not open file");

    let _dag = parse_database(file);
}

#[test]
#[should_panic(expected = "The DAG would cycle adding left parent")]
fn detect_cycle_left() {
    let file = std::fs::File::open("tests/database_cycle_left.txt")
        .expect("Could not open file");

    let _dag = parse_database(file);
}

#[test]
#[should_panic(expected = "The DAG would cycle adding right parent")]
fn detect_cycle_right() {
    let file = std::fs::File::open("tests/database_cycle_right.txt")
        .expect("Could not open file");

    let _dag = parse_database(file);
}

#[test]
#[should_panic(expected = "nodes but there are only")]
fn not_enough_lines() {
    let file = std::fs::File::open("tests/database_not_enought_lines.txt")
        .expect("Could not open file");

    let _dag = parse_database(file);
}

#[test]
#[should_panic(expected = "there are more data left in the file")]
fn too_many_lines() {
    let file = std::fs::File::open("tests/database_too_many_lines.txt")
        .expect("Could not open file");

    let _dag = parse_database(file);
}
