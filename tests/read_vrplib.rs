use vrp_parser::{LoadError, ProblemType, read_from_vrplib};

#[test]
#[ignore]
fn test_read_from_vaplib_succeeds() {
    let filename = "tests/data/vrplib_format/read_vrplib_succeeds.txt";

    let sut = read_from_vrplib(filename).unwrap();

    assert_eq!(sut.name(), "This is a name.");
    assert_eq!(sut.dimension(), 3);
    assert_eq!(sut.problem_type(), ProblemType::CVRP);
    assert_eq!(sut.depots(), &[1]);
    assert_eq!(sut.capacity(), &Some(2));
    assert_eq!(sut.demands(), &Some(vec![0, 11, 12]));
    assert_eq!(sut.node_coords(), &Some(vec![(0, 0), (7, 8), (9, 10)]));
    assert_eq!(
        sut.edge_weights(),
        &[vec![0, 4, 5], vec![4, 0, 6], vec![5, 6, 0]]
    );
}

#[test]
#[ignore]
fn test_read_from_vrplib_fails() {
    let filename = "tests/data/invalid_vrplib_format/edge_weights_not_given.txt";

    let sut = read_from_vrplib(filename);

    match sut {
        Err(LoadError::Parse(_)) => {}
        _ => panic!(),
    }
}

#[test]
#[ignore]
fn test_read_ortec_n242_k12_instance_succeeds() {
    // An example in rustdocs
    let filename = "dat/ORTEC-n242-k12.vrp";

    let sut = read_from_vrplib(filename).unwrap();

    assert_eq!(sut.dimension(), 242);
}
