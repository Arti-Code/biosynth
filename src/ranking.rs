#![allow(unused)]


use std::path::Iter;
use std::vec::IntoIter;
use macroquad::rand::{self, ChooseRandom};
use crate::agent::Agent;
use crate::sketch::AgentSketch;
use crate::settings::*;

pub struct Ranking {
    pub general: Vec<AgentSketch>,
    pub school: Vec<AgentSketch>,
    max_size: usize,
    max_school_size: usize,
    school_max_gen: u32,
}


impl Ranking {

    pub fn new(max_size: usize, max_school_size: usize, school_max_gen: u32) -> Self {
        Ranking {
            general: vec![],
            school: vec![],
            max_size,
            max_school_size,
            school_max_gen,
        }
    }

    pub fn update(&mut self) {
        self.update_school();
        self.update_general();
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

    pub fn add_agent(&mut self, agent: AgentSketch) {
        let gen = agent.generation;
        if gen <= self.school_max_gen {
            self.school.push(agent);
        } else {
            self.general.push(agent);
        }
    }

    pub fn get_general_rank(&self) -> Vec<AgentSketch> {
        return self.general.clone();
    }

    pub fn get_school_rank(&self) -> Vec<AgentSketch> {
        return self.school.clone();
    }

    pub fn is_empty(&self) -> bool {
        return self.is_general_empty() && self.is_school_empty();
    }

    pub fn is_general_empty(&self) -> bool {
        self.general.is_empty()
    }

    pub fn is_school_empty(&self) -> bool {
        self.school.is_empty()
    }

    pub fn get_random_agent(&mut self) -> Option<AgentSketch> {
        let mut s: AgentSketch;
        if rand::gen_range(0, 2)  == 0 {
            if self.is_general_empty() {
                return None;
            }
            let i = self.general.len()-1;
            let idx = rand::gen_range(0, i);
            let agent = self.general.get_mut(idx).unwrap();
            s = agent.to_owned();
            agent.points -= agent.points*0.5;
            agent.points = agent.points.round();
        } else {
            if self.is_school_empty() {
                return None;
            }
            let i = self.school.len()-1;
            let idx = rand::gen_range(0, i);
            let agent = self.school.get_mut(idx).unwrap();
            s = agent.to_owned();
            agent.points -= agent.points*0.5;
            agent.points = agent.points.round();
        }
        return Some(s);
    }

    fn get_random_from_ranking(&mut self) -> AgentSketch {
        let i = self.general.len()-1;
        let idx = rand::gen_range(0, i);
        let agent = self.general.get_mut(idx).unwrap();
        let s = agent.to_owned();
        agent.points -= agent.points*0.5;
        agent.points = agent.points.round();
        return s;
    }

    fn get_random_from_school(&mut self) -> AgentSketch {
        let i = self.general.len()-1;
        let idx = rand::gen_range(0, i);
        let agent = self.general.get_mut(idx).unwrap();
        let s = agent.to_owned();
        agent.points -= agent.points*0.5;
        agent.points = agent.points.round();
        return s;
    }
}