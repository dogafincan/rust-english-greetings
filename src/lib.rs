pub fn random_greeting(name: &str) -> String {
    let greetings = [
        format!("Hi, {}. Welcome!", name),
        format!("Great to see you, {}!", name),
        format!("Hail, {}! Well met!", name),
    ];

    let random_index = fastrand::usize(0..3);

    greetings
        .into_iter()
        .nth(random_index)
        .expect("Missing element")
}

pub fn random_greetings(names: &[&str]) -> Vec<String> {
    let mut greetings = Vec::new();

    for name in names.iter() {
        greetings.push(random_greeting(name));
    }

    greetings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_random_greeting() {
        fastrand::seed(1);
        let result = random_greeting("Doga");
        assert_eq!(result, "Hail, Doga! Well met!");
    }

    #[test]
    fn it_returns_random_greetings() {
        fastrand::seed(1);
        let result = random_greetings(&["Doga", "Ji-yoon"]);

        assert_eq!(
            result,
            ["Hail, Doga! Well met!", "Great to see you, Ji-yoon!"]
        );
    }
}
