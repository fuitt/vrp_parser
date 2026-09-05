use vrp_parser::{ProblemType, read_from_solomon_f64};

#[test]
#[ignore]
fn test_read_from_solomon_succeeds() {
    let filename = "tests/data/solomon_format/read_solomon.txt";

    let sut = read_from_solomon_f64(filename).unwrap();

    assert_eq!(sut.name(), "TestInstance");
    assert_eq!(sut.dimension(), 3);
    assert_eq!(sut.problem_type(), ProblemType::Cvrptw);
    assert_eq!(sut.depots(), &[0]);
    assert_eq!(sut.capacity(), &Some(200.0));
    assert_eq!(sut.demands(), &Some(vec![0.0, 10.0, 30.0]));
    assert_eq!(sut.vehicle_count(), Some(3));
    assert_eq!(
        sut.node_coords(),
        &Some(vec![(40.0, 50.0), (45.0, 68.0), (45.0, 70.0)])
    );
    assert_eq!(
        sut.time_windows(),
        &Some(vec![(0.0, 1000.0), (912.0, 967.0), (825.0, 870.0)])
    );
    assert_eq!(sut.service_times(), &Some(vec![0.0, 90.0, 90.0]));
    // d(0,1) = sqrt(5^2 + 18^2) = sqrt(349)
    // d(0,2) = sqrt(5^2 + 20^2) = sqrt(425)
    // d(1,2) = sqrt(0^2 + 2^2)  = sqrt(4) = 2.0
    let w = sut.edge_weights();
    assert_eq!(w[0][0], 0.0);
    assert!((w[0][1] - 349_f64.sqrt()).abs() < 1e-10);
    assert!((w[0][2] - 425_f64.sqrt()).abs() < 1e-10);
    assert!((w[1][0] - 349_f64.sqrt()).abs() < 1e-10);
    assert_eq!(w[1][1], 0.0);
    assert_eq!(w[1][2], 2.0);
    assert!((w[2][0] - 425_f64.sqrt()).abs() < 1e-10);
    assert_eq!(w[2][1], 2.0);
    assert_eq!(w[2][2], 0.0);
}
