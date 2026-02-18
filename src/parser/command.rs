#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /**
     * GOON command
     */
    Set {
        key: String,
        value: String,
    },
    /**
     * EDGE command
     * get a value given a key
     */
    Get {
        key: String,
    },
    /**
     * BUST
     * remove a value given a ey
     */
    Delete {
        key: String,
    },
    Quit,
}

impl Command {
    pub fn parse(input: &str) -> Result<Command, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            return Err("Empty command".to_string());
        }

        match parts[0].to_uppercase().as_str() {
            "GOON" => {
                if parts.len() < 3 {
                    return Err("GOON requires key and value".to_string());
                }
                Ok(Command::Set {
                    key: parts[1].to_string(),
                    value: parts[2].to_string(),
                })
            }
            "EDGE" => {
                if parts.len() < 2 {
                    return Err("EDGE requires a key".to_string());
                }
                Ok(Command::Get {
                    key: parts[1].to_string(),
                })
            }
            "BUST" => {
                if parts.len() < 2 {
                    return Err("UNGOON requires a key".to_string());
                }
                Ok(Command::Delete {
                    key: parts[1].to_string(),
                })
            }
            "QUIT" | "EXIT" | "FUCKOFF" => Ok(Command::Quit {}),
            _ => Err(format!("Unknown command [{}]", parts[0])),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set() {
        let cmd = Command::parse("GOON name Gooner").unwrap();
        assert_eq!(
            cmd,
            Command::Set {
                key: "name".to_string(),
                value: "Gooner".to_string()
            }
        );
    }

    #[test]
    fn test_parse_get() {
        let cmd = Command::parse("EDGE name").unwrap();
        assert_eq!(
            cmd,
            Command::Get {
                key: "name".to_string()
            }
        );
    }

    #[test]
    fn test_parse_invalid() {
        assert!(Command::parse("INVALID").is_err());
    }
}
