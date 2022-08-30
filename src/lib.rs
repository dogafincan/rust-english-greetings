use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NameError {
    #[error("name cannot be an empty string")]
    EmptyName,
    #[error("no names found")]
    NoNames
}

pub fn random_greeting(name: &str) -> Result<String, NameError> {
    if name.is_empty() {
        return Err(NameError::EmptyName);
    }

    let greetings = [
        format!("Hi, {}. Welcome!", name),
        format!("Great to see you, {}!", name),
        format!("Hail, {}! Well met!", name),
    ];

    let random_index = fastrand::usize(0..3);

    Ok(greetings
        .into_iter()
        .nth(random_index)
        .expect("element in greetings not found"))
}

pub fn random_greetings(names: &[&str]) -> Result<Vec<String>, NameError> {
    if names.is_empty() {
        return Err(NameError::NoNames);
    }
    let mut greetings = Vec::new();

    for name in names.iter() {
        match random_greeting(name) {
            Ok(greeting) => greetings.push(greeting),
            Err(err) => return Err(err),
        };
    }

    Ok(greetings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_random_greeting() {
        fastrand::seed(1);
        let result = random_greeting("").unwrap_err();

        assert_eq!(result, NameError::EmptyName);
    }

    #[test]
    fn it_returns_random_greetings() {
        fastrand::seed(1);
        let result = random_greetings(&["Doga", "Ji-yoon"]).unwrap();

        assert_eq!(
            result,
            vec!["Hail, Doga! Well met!", "Great to see you, Ji-yoon!"]
        );
    }
}
