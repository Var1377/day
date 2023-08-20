#[derive(Debug, Clone)]
pub struct TextAutocompleter<T, F> where F: Fn(&T) -> String {
    f: F,
    t: Vec<T>
}

impl<'a, T, F> TextAutocompleter<T, F> where F: Fn(&T) -> String {
    pub fn new(f: F, t: Vec<T>) -> Self {
        Self { f, t }
    }
}

impl<T, F> inquire::Autocomplete for TextAutocompleter<T, F> where F: Fn(&T) -> String + Clone, T: Clone {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        let mut suggestions = Vec::new();
        for t in self.t.iter() {
            let s = (self.f)(t);
            if s.contains(input) {
                suggestions.push(s);
            }
        }
        Ok(suggestions)
    }

    fn get_completion(
            &mut self,
            input: &str,
            highlighted_suggestion: Option<String>,
        ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
            Ok(Some(highlighted_suggestion.unwrap_or(input.to_string())))
    }
}