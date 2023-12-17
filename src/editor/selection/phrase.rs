use std::cmp::min;

use crate::{
    conversion::{Break, Composition, Interval, Symbol},
    dictionary::{Dictionary, Phrases},
};

#[derive(Debug)]
pub(crate) struct PhraseSelector<D> {
    begin: usize,
    end: usize,
    forward_select: bool,
    buffer: Vec<Symbol>,
    selections: Vec<Interval>,
    breaks: Vec<Break>,
    dict: D,
}

impl<D> PhraseSelector<D>
where
    D: Dictionary,
{
    pub(crate) fn new(forward_select: bool, com: Composition, dict: D) -> PhraseSelector<D> {
        PhraseSelector {
            begin: 0,
            end: com.buffer.len(),
            forward_select,
            buffer: com.buffer,
            selections: com.selections,
            breaks: com.breaks,
            dict,
        }
    }

    pub(crate) fn init(&mut self, cursor: usize) {
        if self.forward_select {
            self.begin = cursor;
            self.end = self.next_break_point(cursor);
        } else {
            self.end = min(cursor + 1, self.buffer.len());
            self.begin = self.previous_break_point(cursor);
        }
        loop {
            let syllables = &self.buffer[self.begin..self.end];
            if self.dict.lookup_phrase(syllables).next().is_some() {
                break;
            }
            if self.forward_select {
                self.end -= 1;
            } else {
                self.begin += 1;
            }
        }
    }

    pub(crate) fn next(&mut self) {
        loop {
            if self.forward_select {
                self.end -= 1;
                if self.begin == self.end {
                    self.end = self.next_break_point(self.begin);
                }
            } else {
                self.begin += 1;
                if self.begin == self.end {
                    self.begin = self.previous_break_point(self.begin);
                }
            }
            let syllables = &self.buffer[self.begin..self.end];
            if self.dict.lookup_phrase(syllables).next().is_some() {
                break;
            }
        }
    }

    fn next_break_point(&self, mut cursor: usize) -> usize {
        loop {
            if self.buffer.len() == cursor {
                break;
            }
            if self.buffer[cursor].is_syllable() {
                cursor += 1;
            }
        }
        cursor
    }

    fn previous_break_point(&self, mut cursor: usize) -> usize {
        let selection_ends: Vec<_> = self.selections.iter().map(|sel| sel.end).collect();
        loop {
            if cursor == 0 {
                return 0;
            }
            if self.buffer.len() == cursor {
                cursor -= 1;
            }
            if selection_ends.binary_search(&cursor).is_ok() {
                break;
            }
            if self.breaks.binary_search(&Break(cursor)).is_ok() {
                break;
            }
            if self.buffer[cursor].is_syllable() {
                cursor -= 1;
            }
        }
        cursor
    }

    pub(crate) fn candidates(&self) -> Phrases<'_> {
        self.dict.lookup_phrase(&self.buffer[self.begin..self.end])
    }

    pub(crate) fn interval(&self, phrase: String) -> Interval {
        Interval {
            start: self.begin,
            end: self.end,
            phrase,
        }
    }
}
