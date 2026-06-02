use crate::creature::{Creature, random_creature};
use crate::element::{Element, random_element};
use crate::config::{CREATURE_COUNT, ELEMENT_COUNT, GRID_HEIGHT, GRID_WIDTH, VERBOSE, THREADS_COUNT};

use std::thread;
use base64::{engine::general_purpose::STANDARD, Engine as _};

pub struct World{
    creatures: Vec<Creature>,
    elements: Vec<Element>,
    tick: u128
}

impl World {
    pub fn new() -> Self {
        World { creatures: Vec::new(), elements: Vec::new(), tick: 0 }
    }

    pub fn reset(&mut self){
        self.creatures.clear();
        self.elements.clear();
        self.tick = 0;
        for _ in 0..CREATURE_COUNT {
            let creature = random_creature();
            self.creatures.push(creature);
        }
        for _ in 0..ELEMENT_COUNT {
             let element = random_element(GRID_WIDTH, GRID_HEIGHT);
             self.elements.push(element);
        }
    }

    pub fn update(&mut self) {
        let mut threads_creatures: Vec<Vec<&mut Creature>> = (0..THREADS_COUNT).map(|_| Vec::new()).collect();

        for (i, creature) in self.creatures.iter_mut().enumerate() {
            threads_creatures[i % THREADS_COUNT].push(creature);
        }

        // Extract values before thread scope to avoid borrow conflicts
        let tick = self.tick;
        let elements = &self.elements;

        // Raccogli gli indici delle creature morte da ogni thread
        let results: Vec<(Vec<usize>, Vec<Creature>, Vec<usize>, Vec<Element>)> = thread::scope(|s| {
            let handles: Vec<_> = threads_creatures
                .into_iter()
                .map(|mut thread_creatures| {
                    s.spawn(move || {
                        let mut dead = Vec::new();
                        let mut offsprings = Vec::new();
                        let mut consumed_element_indices = Vec::new();
                        let mut ejected_elements = Vec::new();
                        for i in 0..thread_creatures.len() {
                            let result = thread_creatures[i].produce(tick, elements);
                            if !(result.alive) {
                                if VERBOSE {
                                    println!(
                                        "Creature at ({}, {}) died. (Genome hash: {})",
                                        thread_creatures[i].get_x(),
                                        thread_creatures[i].get_y(),
                                        STANDARD.encode(thread_creatures[i].get_genome_hash().to_string())
                                    );
                                }
                                dead.push(i);
                            }else{
                                offsprings.extend(result.offsprings);
                                consumed_element_indices.extend(result.consumed_element_indices);
                                ejected_elements.extend(result.ejected_elements);
                            }
                        }
                        (dead, offsprings, consumed_element_indices, ejected_elements)
                    })
                })
                .collect();

            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });

        // Raccogli tutti gli indici morti e rimuovili in ordine inverso
        let threads_results : Vec<(Vec<usize>, Vec<Creature>, Vec<usize>, Vec<Element>)> = results.into_iter().collect();
        let mut creatures_to_remove = Vec::new();
        let mut offsprings = Vec::new();
        let mut consumed_element_indices = Vec::new();
        let mut ejected_elements = Vec::new();
        for (dead, off, consumed, ejected) in threads_results.iter() {
            creatures_to_remove.extend(dead.to_owned());
            offsprings.extend(off.to_owned());
            consumed_element_indices.extend(consumed.to_owned());
            ejected_elements.extend(ejected.to_owned());
        }
        creatures_to_remove.sort_unstable();
        creatures_to_remove.dedup();
        for &i in creatures_to_remove.iter().rev() {
            let last_creature = self.creatures.pop().expect("ERROR IN LAST CREATURE EXTRACTION");
            if i < self.creatures.len() {
                self.creatures[i] = last_creature;
            }
        }
        consumed_element_indices.sort_unstable();
        consumed_element_indices.dedup();
        for &i in consumed_element_indices.iter().rev() {
            let last_element = self.elements.pop().expect("ERROR IN LAST ELEMENT EXTRACTION");
            if i < self.elements.len() {
                self.elements[i] = last_element;
            }
        }
        self.creatures.extend(offsprings);
        self.elements.extend(ejected_elements);

        self.tick += 1;
    }

    


    pub fn get_creatures(&self) -> &Vec<Creature> {
        &self.creatures
    }

    pub fn get_elements(&self) -> &Vec<Element> {
        &self.elements
    }
}