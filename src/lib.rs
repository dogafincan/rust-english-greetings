use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NameError {
    #[error("expected a non-empty string")]
    EmptyNameArgument,
    #[error("expected at least one name")]
    EmptyNamesArgument,
}

pub fn random_greeting(name: &str) -> Result<String, NameError> {
    if name.is_empty() {
        return Err(NameError::EmptyNameArgument);
    }

    let greetings = [
        format!("Hi, {name}. Welcome!"),
        format!("Great to see you, {name}!"),
        format!("Hail, {name}! Well met!"),
    ];

    let random_index = fastrand::usize(0..greetings.len());

    Ok(greetings
        .into_iter()
        .nth(random_index)
        .expect("element in greetings not found"))
}

pub fn random_greetings(names: &[&str]) -> Result<Vec<String>, NameError> {
    if names.is_empty() {
        return Err(NameError::EmptyNamesArgument);
    }

    let mut greetings = Vec::new();

    for name in names.iter() {
        let greeting = random_greeting(name)?;
        greetings.push(greeting);
    }

    Ok(greetings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_greeting() {
        fastrand::seed(1);
        let result = random_greeting("Doga").unwrap();
        assert_eq!(result, "Hail, Doga! Well met!");
    }

    #[test]
    fn test_random_greeting_empty_string() {
        let result = random_greeting("").unwrap_err();
        assert_eq!(result, NameError::EmptyNameArgument);
    }

    #[test]
    fn test_random_greetings() {
        fastrand::seed(1);
        let result = random_greetings(&["Ji-an", "Hiroto", "Somchai"]).unwrap();

        assert_eq!(
            result,
            vec![
                "Hail, Ji-an! Well met!",
                "Great to see you, Hiroto!",
                "Great to see you, Somchai!"
            ]
        );
    }

    #[test]
    fn test_random_greetings_empty_array() {
        let result = random_greetings(&[]).unwrap_err();
        assert_eq!(result, NameError::EmptyNamesArgument);
    }

    #[test]
    fn test_random_greetings_empty_string() {
        let result = random_greetings(&[""]).unwrap_err();
        assert_eq!(result, NameError::EmptyNameArgument);
    }
}
