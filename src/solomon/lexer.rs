#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Token {
    InstanceName(String),
    VehicleSection,
    CustomerSection,
    Data(Vec<String>),
}

pub(crate) fn tokenize(lines: &[String]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut name_found = false;
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !name_found {
            tokens.push(Token::InstanceName(trimmed.to_string()));
            name_found = true;
            continue;
        }
        let token = match trimmed {
            "VEHICLE" => Token::VehicleSection,
            "CUSTOMER" => Token::CustomerSection,
            _ => Token::Data(trimmed.split_whitespace().map(|s| s.to_string()).collect()),
        };
        tokens.push(token);
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_lines() -> Vec<String> {
        vec![
            "TestInstance".to_string(),
            "".to_string(),
            "VEHICLE".to_string(),
            "NUMBER     CAPACITY".to_string(),
            "  3          200".to_string(),
            "".to_string(),
            "CUSTOMER".to_string(),
            "CUST NO.  XCOORD.   YCOORD.    DEMAND   READY TIME  DUE DATE   SERVICE TIME"
                .to_string(),
            "".to_string(),
            "    0      40         50          0          0       1000         0".to_string(),
            "    1      45         68         10        912        967        90".to_string(),
        ]
    }

    #[test]
    fn test_tokenize_produces_instance_name_first() {
        let tokens = tokenize(&sample_lines());
        assert_eq!(tokens[0], Token::InstanceName("TestInstance".to_string()));
    }

    #[test]
    fn test_tokenize_produces_vehicle_section() {
        let tokens = tokenize(&sample_lines());
        assert_eq!(tokens[1], Token::VehicleSection);
    }

    #[test]
    fn test_tokenize_produces_customer_section() {
        let tokens = tokenize(&sample_lines());
        assert_eq!(tokens[4], Token::CustomerSection);
    }

    #[test]
    fn test_tokenize_produces_data_for_non_keyword_lines() {
        let tokens = tokenize(&sample_lines());
        assert_eq!(
            tokens[2],
            Token::Data(vec!["NUMBER".to_string(), "CAPACITY".to_string()])
        );
        assert_eq!(
            tokens[3],
            Token::Data(vec!["3".to_string(), "200".to_string()])
        );
    }

    #[test]
    fn test_tokenize_skips_blank_lines() {
        let tokens = tokenize(&sample_lines());
        assert!(
            !tokens
                .iter()
                .any(|t| matches!(t, Token::Data(d) if d.is_empty()))
        );
    }

    #[test]
    fn test_tokenize_empty_input_produces_no_tokens() {
        let tokens = tokenize(&[]);
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_tokenize_numeric_instance_name_is_not_confused_with_data() {
        let lines = vec!["101".to_string(), "VEHICLE".to_string()];
        let tokens = tokenize(&lines);
        assert_eq!(tokens[0], Token::InstanceName("101".to_string()));
        assert_eq!(tokens[1], Token::VehicleSection);
    }
}
