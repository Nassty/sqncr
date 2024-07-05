use crate::app::Color;
use std::collections::HashMap;
use yewdux::prelude::Store;

#[derive(Clone, PartialEq, Eq, Debug, Store)]
pub struct State {
    pub count: usize,
    pub current: Color,
    pub selected: Vec<Option<Color>>,
    pub budget: HashMap<Color, usize>,
}

impl Default for State {
    fn default() -> Self {
        let k = 9;
        Self {
            count: k,
            current: Color::Red,
            selected: vec![None; k],
            budget: Self::populate_budget(),
        }
    }
}

impl State {
    pub fn populate_budget() -> HashMap<Color, usize> {
        let mut budget = HashMap::new();
        budget.insert(Color::Red, 3);
        budget.insert(Color::Green, 3);
        budget.insert(Color::Blue, 3);
        budget
    }
    pub fn validate(&self, period: usize) -> bool {
        let resps = self
            .selected
            .iter()
            .cloned()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        if Self::validate_contiguous(resps.as_slice()) {
            false
        } else {
            Self::validate_responses(resps.as_slice(), period)
        }
    }
    pub(crate) fn validate_contiguous(selected: &[Color]) -> bool {
        selected.windows(2).any(|w| w.iter().all(|x| *x == w[0]))
    }
    pub(crate) fn validate_responses(selected: &[Color], period: usize) -> bool {
        if selected[0] == selected[1] {
            return false;
        }

        if selected.len() < period * 2 {
            return true;
        }
        let buffer = &selected[..period];
        let mut skip = true;
        let rest: Vec<_> = selected[period..]
            .iter()
            .cloned()
            .skip_while(|x| {
                if *x == buffer[0] {
                    skip = false;
                }
                skip
            })
            .collect::<Vec<_>>();
        if rest.as_slice() == buffer {
            false
        } else {
            Self::validate_responses(&selected[1..], period)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_contiguous() {
        let selected = vec![Color::Red, Color::Red, Color::Green];
        assert!(State::validate_contiguous(&selected));
        let selected = vec![Color::Red, Color::Green, Color::Green];
        assert!(State::validate_contiguous(&selected));
        let selected = vec![Color::Red, Color::Green, Color::Blue];
        assert!(!State::validate_contiguous(&selected));
    }

    #[test]
    fn test_validate_responses() {
        let responses = vec![Color::Red, Color::Green, Color::Blue];
        assert!(State::validate_responses(&responses, 3));
        let responses = vec![Color::Red, Color::Green, Color::Red];
        assert!(State::validate_responses(&responses, 3));
        let responses = vec![
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ];
        assert!(!State::validate_responses(&responses, 3));
        let responses = vec![
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Red,
        ];
        assert!(State::validate_responses(&responses, 3));
        let responses = vec![Color::Red, Color::Red, Color::Blue];
        assert!(!State::validate_responses(&responses, 3));
    }
}
