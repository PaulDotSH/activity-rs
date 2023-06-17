use rand::prelude::*;
use thiserror::Error as ThisError;



#[derive(Debug, ThisError)]
#[error("There is a limit of 4 on the number of teams")]
pub struct TooManyTeamsError;

#[derive(Debug, ThisError)]
#[error("You've already seen every word for this activity type")]
pub struct NotEnoughWordsError;

pub struct Activity<'a> {
    describe: &'a str,
    mime: &'a str,
    draw: &'a str,
    describe_indexes: Vec<u32>,
    mime_indexes: Vec<u32>,
    draw_indexes: Vec<u32>,
    describe_word_index: u16,
    mime_word_index: u16,
    draw_word_index: u16,
    turn: u8,
    teams: Vec<Team>,
    score: u8, // Current word point score
}

struct Team {
    score: u8,
}

impl Team {
    fn new(score: u8) -> Team {
        Team { score }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ActivityType {
    Describe,
    Mime,
    Draw
}

impl Activity<'_> {
    pub fn add_team_score(&mut self) {
        self.teams[self.turn as usize].score += self.score;
        self.score = 0
    }

    pub fn get_next_word(&mut self) -> Result<(&str, u8, ActivityType), NotEnoughWordsError> {
        let (indexes, s, pos, a_type) = match thread_rng().gen_range(0..3) {
            0 => (&self.describe_indexes, self.describe, &mut self.describe_word_index, ActivityType::Describe),
            1 => (&self.mime_indexes, self.mime, &mut self.mime_word_index, ActivityType::Mime),
            _ => (&self.draw_indexes, self.draw, &mut self.draw_word_index, ActivityType::Draw)
        };

        println!("{}", pos);
        let (word, score) = Self::get_word_from_pos(s, indexes, *pos)?;
        *pos += 1;

        println!("{}", self.turn);
        if self.turn == self.teams.len() as u8 - 1 {
            self.turn = 0
        } else {
            self.turn += 1
        }

        Ok((word, score, a_type))
    }

    fn get_word_from_pos<'a>(s: &'a str, indexes: &Vec<u32>, pos: u16) -> Result<(&'a str, u8), NotEnoughWordsError> {
        if indexes.len() == pos as usize {
            return Err(NotEnoughWordsError);
        }

        // Handles the case for the first word
        let mut index: u32 = indexes[pos as usize] + 1;
        if index == 1 {
            index = 0;
        }

        // Find the end of the word, searches from the word start so shouldn't be slow
        let str_end = *(&s[index as usize..].find(',').unwrap_or(s.len() - index as usize)) as u32 + index;

        let score = s[str_end as usize - 1..str_end as usize].parse::<u8>().unwrap();
        let str = &s[index as usize..str_end as usize - 1];

        Ok((str, score))
    }


    pub fn reset(&mut self) {
        self.turn = 0;
        self.mime_word_index = 0;
        self.describe_word_index = 0;
        self.draw_word_index = 0;
        self.draw_indexes.shuffle(&mut thread_rng());
        self.describe_indexes.shuffle(&mut thread_rng());
        self.mime_indexes.shuffle(&mut thread_rng());
    }

    pub fn add_team(&mut self) -> Result<(), TooManyTeamsError> {
        if self.teams.len() == 4 {
            return Err(TooManyTeamsError);
        }

        Ok(self.teams.push(Team::new(0)))
    }

    // TODO: Benchmark how fast it is with a parallel iterator vs with sequential

    pub fn new<'a>(describe: &'a str, mime: &'a str, draw: &'a str, team_size: u8) -> Activity<'a> {
        // Returns indexes for each word
        fn parse(s: &str) -> Vec<u32> {
            (0..1).chain(s.match_indices(',').map(|x| x.0 as u32)).collect()
        }

        // TODO: Check if we cal do a parallel shuffle and benchmark that, also we should be able to sshuffle all the indexes in parallel and join on that

        let mut describe_indexes = parse(describe);
        describe_indexes.shuffle(&mut thread_rng());
        let mut mime_indexes = parse(describe);
        mime_indexes.shuffle(&mut thread_rng());
        let mut draw_indexes = parse(describe);
        draw_indexes.shuffle(&mut thread_rng());
        
        let teams = (0..team_size).map(|_| Team::new(0)).collect();

        Activity {
            mime,
            draw,
            describe,
            mime_indexes,
            draw_indexes,
            describe_indexes,
            describe_word_index: 0,
            draw_word_index: 0,
            mime_word_index: 0,
            turn: 0,
            teams,
            score: 0,
        }
    }
}