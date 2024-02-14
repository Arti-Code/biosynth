#![allow(unused)]


use crate::sketch::AgentSketch;
use crate::settings::*;

pub struct Ranking {
    general: Vec<AgentSketch>,
    school: Vec<AgentSketch>,
    max_size: usize,
    max_school_size: usize,
    school_limit: i32,
}


impl Ranking {

    pub fn new(max_size: usize, max_school_size: usize, school_limit: i32) -> Self {
        Ranking {
            general: vec![],
            school: vec![],
            max_size,
            max_school_size,
            school_limit,
        }
    }

    fn update_general(&mut self) {
        let settings = settings();
        self.general.sort_by(|a, b| b.points.total_cmp(&a.points));
        let general_copy = self.general.to_vec();
        for elem1 in general_copy.iter() {
            self.general.retain(|elem2| {
                if elem1.specie == elem2.specie {
                    if elem1.points == elem2.points {
                        return true;
                    } else if elem2.points < elem1.points {
                        return false;
                    } else {
                        return true;
                    }
                } else {
                    return true;
                }
            });
        }
        if self.general.len() > self.max_size {
            self.general.pop();
        }
    }

    fn update_school(&mut self) {
        let settings = settings();
        self.school.sort_by(|a, b| b.points.total_cmp(&a.points));
        let school_copy = self.school.to_vec();
        for elem1 in school_copy.iter() {
            self.school.retain(|elem2| {
                if elem1.specie == elem2.specie {
                    if elem1.points == elem2.points {
                        return true;
                    } else if elem2.points < elem1.points {
                        return false;
                    } else {
                        return true;
                    }
                } else {
                    return true;
                }
            });
        }
        if self.school.len() > self.max_school_size {
            self.school.pop();
        }
    }


}