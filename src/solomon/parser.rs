use super::lexer::Token;

/// Parsed data from a Solomon-formatted instance file.
#[derive(Debug)]
pub(crate) struct SolomonData {
    pub name: String,
    pub vehicle_count: usize,
    pub capacity: f64,
    pub node_coords: Vec<(f64, f64)>,
    pub demands: Vec<f64>,
    pub time_windows: Vec<(f64, f64)>,
    pub service_times: Vec<f64>,
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum SolomonParseError {
    #[error("unexpected end of file")]
    UnexpectedEof,
    #[error("missing VEHICLE section")]
    MissingVehicleSection,
    #[error("missing CUSTOMER section")]
    MissingCustomerSection,
    #[error("invalid vehicle count or capacity")]
    InvalidVehicleLine,
    #[error("invalid customer row")]
    InvalidCustomerRow,
    #[error("no customers found")]
    NoCustomers,
}

pub(crate) fn parse(tokens: &[Token]) -> Result<SolomonData, SolomonParseError> {
    let mut iter = tokens.iter();

    let name = match iter.next() {
        Some(Token::InstanceName(s)) => s.clone(),
        _ => return Err(SolomonParseError::UnexpectedEof),
    };

    loop {
        match iter.next() {
            Some(Token::VehicleSection) => break,
            Some(_) => continue,
            None => return Err(SolomonParseError::MissingVehicleSection),
        }
    }

    // skip "NUMBER  CAPACITY" sub-header
    iter.next().ok_or(SolomonParseError::UnexpectedEof)?;

    let (vehicle_count, capacity) = match iter.next() {
        Some(Token::Data(parts)) => parse_vehicle_data(parts)?,
        _ => return Err(SolomonParseError::InvalidVehicleLine),
    };

    loop {
        match iter.next() {
            Some(Token::CustomerSection) => break,
            Some(_) => continue,
            None => return Err(SolomonParseError::MissingCustomerSection),
        }
    }

    // skip column-header line ("CUST NO.  XCOORD. ...")
    iter.next().ok_or(SolomonParseError::UnexpectedEof)?;

    let mut node_coords = Vec::new();
    let mut demands = Vec::new();
    let mut time_windows = Vec::new();
    let mut service_times = Vec::new();

    for token in iter {
        match token {
            Token::Data(parts) => {
                let (x, y, demand, ready, due, svc) = parse_customer_data(parts)?;
                node_coords.push((x, y));
                demands.push(demand);
                time_windows.push((ready, due));
                service_times.push(svc);
            }
            _ => return Err(SolomonParseError::InvalidCustomerRow),
        }
    }

    if node_coords.is_empty() {
        return Err(SolomonParseError::NoCustomers);
    }

    Ok(SolomonData {
        name,
        vehicle_count,
        capacity,
        node_coords,
        demands,
        time_windows,
        service_times,
    })
}

fn parse_vehicle_data(parts: &[String]) -> Result<(usize, f64), SolomonParseError> {
    if parts.len() < 2 {
        return Err(SolomonParseError::InvalidVehicleLine);
    }
    let count = parts[0]
        .parse::<usize>()
        .map_err(|_| SolomonParseError::InvalidVehicleLine)?;
    let cap = parts[1]
        .parse::<f64>()
        .map_err(|_| SolomonParseError::InvalidVehicleLine)?;
    Ok((count, cap))
}

fn parse_customer_data(
    parts: &[String],
) -> Result<(f64, f64, f64, f64, f64, f64), SolomonParseError> {
    if parts.len() != 7 {
        return Err(SolomonParseError::InvalidCustomerRow);
    }
    let parse = |s: &String| {
        s.parse::<f64>()
            .map_err(|_| SolomonParseError::InvalidCustomerRow)
    };
    // parts[0] is CUST NO., ignored
    let x = parse(&parts[1])?;
    let y = parse(&parts[2])?;
    let demand = parse(&parts[3])?;
    let ready = parse(&parts[4])?;
    let due = parse(&parts[5])?;
    let svc = parse(&parts[6])?;
    Ok((x, y, demand, ready, due, svc))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> String {
        v.to_string()
    }

    fn sample_tokens() -> Vec<Token> {
        vec![
            Token::InstanceName(s("TestInstance")),
            Token::VehicleSection,
            Token::Data(vec![s("NUMBER"), s("CAPACITY")]),
            Token::Data(vec![s("3"), s("200")]),
            Token::CustomerSection,
            Token::Data(vec![
                s("CUST"),
                s("NO."),
                s("XCOORD."),
                s("YCOORD."),
                s("DEMAND"),
                s("READY"),
                s("TIME"),
                s("DUE"),
                s("DATE"),
                s("SERVICE"),
                s("TIME"),
            ]),
            Token::Data(vec![
                s("0"),
                s("40"),
                s("50"),
                s("0"),
                s("0"),
                s("1000"),
                s("0"),
            ]),
            Token::Data(vec![
                s("1"),
                s("45"),
                s("68"),
                s("10"),
                s("912"),
                s("967"),
                s("90"),
            ]),
            Token::Data(vec![
                s("2"),
                s("45"),
                s("70"),
                s("30"),
                s("825"),
                s("870"),
                s("90"),
            ]),
        ]
    }

    #[test]
    fn test_parse_succeeds() {
        let value = parse(&sample_tokens()).unwrap();
        assert_eq!(value.name, "TestInstance");
        assert_eq!(value.vehicle_count, 3);
        assert_eq!(value.capacity, 200.0);
        assert_eq!(
            value.node_coords,
            vec![(40.0, 50.0), (45.0, 68.0), (45.0, 70.0)]
        );
        assert_eq!(value.demands, vec![0.0, 10.0, 30.0]);
        assert_eq!(
            value.time_windows,
            vec![(0.0, 1000.0), (912.0, 967.0), (825.0, 870.0)]
        );
        assert_eq!(value.service_times, vec![0.0, 90.0, 90.0]);
    }

    #[test]
    fn test_parse_fails_on_empty_tokens() {
        let value = parse(&[]).unwrap_err();
        assert_eq!(value, SolomonParseError::UnexpectedEof);
    }

    #[test]
    fn test_parse_fails_when_vehicle_section_missing() {
        let tokens = vec![Token::InstanceName(s("TestInstance"))];
        let value = parse(&tokens).unwrap_err();
        assert_eq!(value, SolomonParseError::MissingVehicleSection);
    }

    #[test]
    fn test_parse_fails_when_customer_section_missing() {
        let tokens = vec![
            Token::InstanceName(s("TestInstance")),
            Token::VehicleSection,
            Token::Data(vec![s("NUMBER"), s("CAPACITY")]),
            Token::Data(vec![s("3"), s("200")]),
        ];
        let value = parse(&tokens).unwrap_err();
        assert_eq!(value, SolomonParseError::MissingCustomerSection);
    }

    #[test]
    fn test_parse_fails_on_invalid_vehicle_line() {
        let tokens = vec![
            Token::InstanceName(s("TestInstance")),
            Token::VehicleSection,
            Token::Data(vec![s("NUMBER"), s("CAPACITY")]),
            Token::Data(vec![s("three"), s("200")]),
            Token::CustomerSection,
        ];
        let value = parse(&tokens).unwrap_err();
        assert_eq!(value, SolomonParseError::InvalidVehicleLine);
    }

    #[test]
    fn test_parse_fails_on_invalid_customer_row() {
        let mut tokens = sample_tokens();
        tokens[7] = Token::Data(vec![
            s("1"),
            s("45"),
            s("68"),
            s("ten"),
            s("912"),
            s("967"),
            s("90"),
        ]);
        let value = parse(&tokens).unwrap_err();
        assert_eq!(value, SolomonParseError::InvalidCustomerRow);
    }
}
