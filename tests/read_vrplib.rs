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
    assert_eq!(
        sut.node_coords(),
        &Some(vec![(0.0, 0.0), (7.0, 8.0), (9.0, 10.0)])
    );
    assert_eq!(
        sut.edge_weights(),
        &[vec![0, 4, 5], vec![4, 0, 6], vec![5, 6, 0]]
    );
}

#[test]
#[ignore]
fn test_read_from_vaplib_euc2d() {
    let filename = "tests/data/vrplib_format/read_vrplib_euc2d.txt";

    let sut = read_from_vrplib(filename).unwrap();

    assert_eq!(sut.name(), "This is a name.");
    assert_eq!(sut.dimension(), 3);
    assert_eq!(sut.problem_type(), ProblemType::CVRP);
    assert_eq!(sut.depots(), &[1]);
    assert_eq!(sut.capacity(), &Some(2));
    assert_eq!(sut.demands(), &Some(vec![0, 11, 12]));
    assert_eq!(
        sut.node_coords(),
        &Some(vec![(0.0, 0.0), (7.0, 8.0), (9.0, 10.0)])
    );
    assert_eq!(
        sut.edge_weights(),
        &[vec![0, 11, 13], vec![11, 0, 3], vec![13, 3, 0]]
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
