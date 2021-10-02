use crate::{DegreeType, LetterType};


use super::TensorKey;

pub struct TensorBasisIterator<const NLETTERS: DegreeType> {
    current: Option<TensorKey<NLETTERS>>,
    max_depth: DegreeType,
}

impl<const NLETTERS: DegreeType> TensorBasisIterator<NLETTERS>
{
    pub fn new() -> Self
    {
        Self {
            current: Some(TensorKey::<NLETTERS>::new()),
            max_depth: TensorKey::<NLETTERS>::max_depth(),
        }
    }

    pub fn starting_from(key: impl Into<TensorKey<NLETTERS>>) -> Self
    {
        Self {
            current: Some(key.into()),
            max_depth: TensorKey::<NLETTERS>::max_depth(),
        }
    }
}


impl<const NLETTERS: DegreeType> Iterator for TensorBasisIterator<NLETTERS>
{
    type Item = TensorKey<NLETTERS>;

    fn next(&mut self) -> Option<Self::Item>
    {
        if let Some(current) = self.current.as_mut() {
            let this_key = current.clone();

            let mut letter_i = 0;
            loop {
                if letter_i == current.size() {
                    if letter_i == self.max_depth {
                        self.current = None;
                    } else {
                        current.push_front(1);
                    }
                    break;
                }

                let letter = current.get_letter(letter_i);
                if letter == (NLETTERS as LetterType) {
                    current.set_letter(letter_i, 1);
                    letter_i += 1;
                } else {
                    current.set_letter(letter_i, letter + 1);
                    break;
                }
            }
            Some(this_key)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    type Key = TensorKey<3>;
    type IterT = TensorBasisIterator<3>;

    #[test]
    fn test_iterator_first_key() {
        let mut itr = IterT::new();

        assert_eq!(itr.next(), Some(Key::new()));
    }

    #[test]
    fn test_iterator_advance_letters()
    {
        let start = Key::from_letter(1);
        let mut itr = IterT::starting_from(start);

        assert_eq!(itr.next(), Some(Key::from_letter(1)));
        assert_eq!(itr.next(), Some(Key::from_letter(2)));
        assert_eq!(itr.next(), Some(Key::from_letter(3)));
    }

    #[test]
    fn test_iterator_move_to_level_2() {
        let start = Key::from_letter(3);
        let mut itr = IterT::starting_from(start);

        assert_eq!(itr.next(), Some(Key::from_letter(3)));

        assert_eq!(itr.next(), Some(Key::from_letters(&[1, 1])));
    }


    #[test]
    fn test_iterator_13_to_21() {
        let start = Key::from_letters(&[1, 3]);
        let mut itr = IterT::starting_from(start);

        assert_eq!(itr.next(), Some(Key::from_letters(&[1, 3])));
        assert_eq!(itr.next(), Some(Key::from_letters(&[2, 1])));
    }
}