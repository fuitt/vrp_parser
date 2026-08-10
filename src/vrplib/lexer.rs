use super::{Token, TokenError};

pub(crate) fn tokenize(line: &str) -> Result<Token, TokenError> {
    if line.contains(':') {
        let (key, value) = line.split_once(':').unwrap();
        let key = key.trim();
        let value = value.trim().to_string();
        match key {
            "NAME" => Ok(Token::Name(value)),
            "COMMENT" => Ok(Token::Comment(value)),
            "TYPE" => Ok(Token::Type(value)),
            "DIMENSION" => Ok(Token::Dimension(value)),
            "EDGE_WEIGHT_TYPE" => Ok(Token::EdgeWeightType(value)),
            "EDGE_WEIGHT_FORMAT" => Ok(Token::EdgeWeightFormat(value)),
            "NODE_COORD_TYPE" => Ok(Token::NodeCoordType(value)),
            "CAPACITY" => Ok(Token::Capacity(value)),
            _ => Err(TokenError::UnknownKey(key.to_string())),
        }
    } else {
        let line = line.trim();
        match line {
            "EDGE_WEIGHT_SECTION" => Ok(Token::EdgeWeightSection),
            "NODE_COORD_SECTION" => Ok(Token::NodeCoordSection),
            "DEMAND_SECTION" => Ok(Token::DemandSection),
            "DEPOT_SECTION" => Ok(Token::DepotSection),
            "-1" | "EOF" => Ok(Token::Eof),
            _ => {
                let data: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
                Ok(Token::Data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_name() {
        let s = "NAME : instance 001\n";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::Name("instance 001".to_string()));
    }

    #[test]
    fn test_parse_comment() {
        let s = "COMMENT : This is a test instance. \n";
        let value = tokenize(s).unwrap();
        assert_eq!(
            value,
            Token::Comment("This is a test instance.".to_string())
        );
    }

    #[test]
    fn test_parse_type() {
        let s = "TYPE : CVRP";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::Type("CVRP".to_string()));
    }

    #[test]
    fn test_parse_unknown_type_succeeds_as_token() {
        let s = "TYPE : foo_vrp";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::Type("foo_vrp".to_string()));
    }

    #[test]
    fn test_parse_dimension() {
        let s = "DIMENSION : 123";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::Dimension("123".to_string()));
    }

    #[test]
    fn test_parse_edge_weight_type() {
        let s = "EDGE_WEIGHT_TYPE : EXPLICIT";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::EdgeWeightType("EXPLICIT".to_string()));
    }

    #[test]
    fn test_parse_edge_weight_format() {
        let s = "EDGE_WEIGHT_FORMAT : LOWER_ROW";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::EdgeWeightFormat("LOWER_ROW".to_string()));
    }

    #[test]
    fn test_parse_node_coord_type() {
        let s = "NODE_COORD_TYPE : TWOD_COORDS";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::NodeCoordType("TWOD_COORDS".to_string()));
    }

    #[test]
    fn test_parse_capacity() {
        let s = "CAPACITY : 45";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::Capacity("45".to_string()));
    }

    #[test]
    fn test_parse_unknown_key_fails() {
        let s = "UNKNOWN_KEY : value";
        let value = tokenize(s).unwrap_err();
        assert_eq!(value, TokenError::UnknownKey("UNKNOWN_KEY".to_string()));
    }

    #[test]
    fn test_parse_edge_weight_section() {
        let s = "EDGE_WEIGHT_SECTION";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::EdgeWeightSection);
    }

    #[test]
    fn test_parse_node_coord_section() {
        let s = "NODE_COORD_SECTION";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::NodeCoordSection);
    }

    #[test]
    fn test_parse_demand_section() {
        let s = "DEMAND_SECTION";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::DemandSection);
    }

    #[test]
    fn test_parse_depot_section() {
        let s = "DEPOT_SECTION";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::DepotSection);
    }

    #[test]
    fn test_parse_eof_num() {
        let s = "-1";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::Eof);
    }

    #[test]
    fn test_parse_eof_string() {
        let s = "EOF";
        let value = tokenize(s).unwrap();
        assert_eq!(value, Token::Eof);
    }

    #[test]
    fn test_parse_data() {
        let s = "  1 2 3 4 5 6 7 8 9 10\n";
        let value = tokenize(s).unwrap();
        assert_eq!(
            value,
            Token::Data(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
                "10".to_string()
            ])
        );
    }
}
