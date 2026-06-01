use crate::creature::{Creature, random_creature};
use crate::gene::{Gene};
use crate::element::{Element, ElementKind, random_element};
use crate::config::{CREATURE_COUNT, ELEMENT_COUNT, FUNDAMENTAL_ELEMENT, FUNDAMENTAL_ELEMENT_CONSUMPTION_AMOUNT, GRID_HEIGHT, GRID_WIDTH, SENSE_BASE_MULTIPLIER, SIZE_BASE_MULTIPLIER, SPEED_BASE_MULTIPLIER, VERBOSE, THREADS_COUNT};

use std::cmp::{min};
use std::thread;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use macroquad::rand;

struct CreatureResult {
    alive: bool,
    offsprings: Vec<Creature>,
    ejected_elements: Vec<Element>
}

impl CreatureResult {
    fn new() -> Self {
        CreatureResult { alive: true, offsprings: Vec::new(), ejected_elements: Vec::new() }
    }
}

struct GeneResult {
    produced: bool,
    ejected_elements: Vec<Element>
}

pub struct World{
    creatures: Vec<Creature>,
    elements: Vec<Element>,
    tick: u128
}

impl World {
    pub fn new() -> Self {
        World { creatures: Vec::new(), elements: Vec::new(), tick: 0 }
    }

    pub fn get_around(&self, x: u16, y: u16) -> (u16, u16) {
        let directions = [(-1i8, -1i8), (-1i8, 0i8), (-1i8, 1i8), (0i8, -1i8), (0i8, 1i8), (1i8, -1i8), (1i8, 0i8), (1i8, 1i8)];
        let direction = directions[rand::gen_range(0, directions.len())];
        let _x : u16 = x.saturating_add_signed(direction.0 as i16);
        let _y : u16 = y.saturating_add_signed(direction.1 as i16);
        ( if _x >= GRID_WIDTH { GRID_WIDTH - 1 } else { _x } , if _y >= GRID_HEIGHT { GRID_HEIGHT - 1 } else { _y })
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
        let mut threads_creatures = vec![Vec::new(); THREADS_COUNT];
        let creatures = self.creatures.clone();

        for (i, creature) in creatures.iter().enumerate() {
            threads_creatures[i % THREADS_COUNT].push(creature);
        }

        let self_ref = &*self;

        // Raccogli gli indici delle creature morte da ogni thread
        let results: Vec<(Vec<usize>, Vec<Creature>, Vec<Element>)> = thread::scope(|s| {
            let handles: Vec<_> = threads_creatures
                .into_iter()
                .map(|thread_creatures| {
                    s.spawn(move || {
                        let mut dead = Vec::new();
                        let mut offsprings = Vec::new();
                        let mut ejected_elements = Vec::new();
                        for i in 0..thread_creatures.len() {
                            let mut creature = thread_creatures[i].clone();
                            let result = self_ref.produce_creature(&mut creature);
                            if !(result.alive) {
                                if VERBOSE {
                                    println!(
                                        "Creature at ({}, {}) died. (Genome hash: {})",
                                        creature.get_x(),
                                        creature.get_y(),
                                        STANDARD.encode(creature.get_genome_hash().to_string())
                                    );
                                }
                                dead.push(i);
                            }else{
                                offsprings.extend(result.offsprings);
                                ejected_elements.extend(result.ejected_elements);
                            }
                        }
                        (dead, offsprings, ejected_elements)
                    })
                })
                .collect();

            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });

        // Raccogli tutti gli indici morti e rimuovili in ordine inverso
        let threads_results : Vec<(Vec<usize>, Vec<Creature>, Vec<Element>)> = results.into_iter().collect();
        let mut creatures_to_remove = threads_results.iter().flat_map(|(dead, _, _)| dead.clone()).collect::<Vec<usize>>();
        let offsprings = threads_results.iter().flat_map(|(_, offsprings, _)| offsprings.clone()).collect::<Vec<Creature>>();
        let ejected_elements = threads_results.iter().flat_map(|(_, _, ejected)| ejected.clone()).collect::<Vec<Element>>();
        creatures_to_remove.sort_unstable();
        creatures_to_remove.dedup();
        for &i in creatures_to_remove.iter().rev() {
            self.creatures.remove(i);
        }
        self.creatures.extend(offsprings);
        self.elements.extend(ejected_elements);

        self.tick += 1;
    }

    fn produce_creature(&self, creature: &mut Creature) -> CreatureResult {
        let mut result = CreatureResult::new();
        creature.remove_elements(FUNDAMENTAL_ELEMENT, FUNDAMENTAL_ELEMENT_CONSUMPTION_AMOUNT);
        if creature.get_elements()[&FUNDAMENTAL_ELEMENT] == 0 {
            result.alive = false;
            return result;
        }
        let mut produced_genes = Vec::new();
        let mut critical_produced_genes = Vec::new();
        let mut creature = creature.clone();
        for gene in creature.get_genes() {
            if gene.get_frequency_exponent() < 0 && self.tick % (2u32.pow((-gene.get_frequency_exponent()) as u32)) as u128 != 0 {
                continue;
            } else if gene.get_frequency_exponent() >= 0 {
                for _ in 0..(2u32.pow(gene.get_frequency_exponent() as u32)) {
                    let mut creature = creature.clone();
                    let gene_result = self.produce_gene(&gene, &mut creature);
                    if gene_result.produced {
                        result.ejected_elements.extend(gene_result.ejected_elements);
                        produced_genes.push(gene);
                        if gene.is_critical() {
                            critical_produced_genes.push(gene);
                        }
                        if gene.will_reproduce() {
                            let (new_x, new_y) = self.get_around(creature.get_x(), creature.get_y());
                            let new_creature = creature.clone().reproduce(new_x, new_y);
                            if VERBOSE {
                                println!("Creature at ({}, {}) reproduced to ({}, {}). (Genome hash: {})", creature.get_x(), creature.get_y(), new_x, new_y, STANDARD.encode(creature.get_genome_hash().to_string()));
                            }
                            result.offsprings.push(new_creature);
                        }
                    } else {
                        if gene.is_critical() {
                            result.alive = false;
                            return result;
                        }
                    }
                }
            }
        }

        let mut size = SIZE_BASE_MULTIPLIER;
        let mut speed = SPEED_BASE_MULTIPLIER;
        let mut sense = SENSE_BASE_MULTIPLIER;
        for gene in produced_genes.clone() {
            size *= 2f32.powi(gene.get_size_multiplier_exponent() as i32);
            speed *= 2f32.powi(gene.get_speed_multiplier_exponent() as i32);
            sense *= 2f32.powi(gene.get_sense_multiplier_exponent() as i32);
        }
        if size < 1f32 {
            result.alive = false;
            return result;
        }
        let target_element: Option<ElementKind>;
        if critical_produced_genes.is_empty() {
            if produced_genes.is_empty() {
                target_element = None;
            } else {
                target_element = Some(produced_genes[rand::gen_range(0, produced_genes.len())].get_target_element());
            }
        } else {
            target_element = Some(critical_produced_genes[rand::gen_range(0, critical_produced_genes.len())].get_target_element());
        }
        if target_element.is_some() {
            let (move_x, move_y) = self.compute_move(&mut creature, target_element.unwrap(), speed, sense);
            let old_x = creature.get_x();
            let old_y = creature.get_y();
            let new_x = old_x.saturating_add_signed(move_x as i16);
            let new_y = old_y.saturating_add_signed(move_y as i16);
            creature.set_x(if new_x<GRID_WIDTH {new_x} else {GRID_WIDTH-1});
            creature.set_y(if new_y<GRID_HEIGHT {new_y} else {GRID_HEIGHT-1});
        }
        self.consume_elements(&mut creature);
        return result;
    }

    fn produce_gene(&self, gene: &Gene, creature: &mut Creature) -> GeneResult {
        for (kind, quantity) in gene.get_elements_needed().iter() {
            if creature.get_elements().get(kind).copied().unwrap_or(0) < *quantity {
                let luck = rand::gen_range(0.0, 1.0) < 0.5;
                if (gene.is_critical() && !luck) || !gene.is_critical() || creature.get_elements().get(kind).copied().unwrap_or(0)==0 {
                    return GeneResult { produced: false, ejected_elements: Vec::new() };
                }
            }
        }

        for (kind, quantity) in gene.get_elements_needed().iter() {
            let consumed_quantity = min(*quantity, creature.get_elements().get(kind).copied().unwrap_or(0));
            creature.remove_elements(*kind, consumed_quantity);
        }

        for (kind, quantity) in gene.get_elements_produced().iter() {
            creature.add_elements(*kind, *quantity);
        }

        let mut ejected_elements = Vec::new();

        for (kind, quantity) in gene.get_elements_ejected().iter() {
            let ejected_quantity = min(*quantity, creature.get_elements().get(kind).copied().unwrap_or(0));
            creature.remove_elements(*kind, ejected_quantity);
            for _ in 0..ejected_quantity {
                let (new_x, new_y) = self.get_around(creature.get_x(), creature.get_y());
                ejected_elements.push(Element::new(new_x, new_y, *kind));
            }
        }
        return GeneResult { produced: true, ejected_elements };
    }

    fn compute_move(&self, creature: &mut Creature, target: ElementKind, speed: f32, sense: f32) -> (i32, i32) {
        let mut min_dist_sq = sense * sense;
        let mut element: Option<&Element> = None;
        let mut dx: i32 = 0;
        let mut dy: i32 = 0;

        for _element in &self.elements {
            if _element.get_kind().ne(&target) {
                continue;
            }
            dx = i32::from(*_element.get_x()) - i32::from(creature.get_x());
            dy = i32::from(*_element.get_y()) - i32::from(creature.get_y());
            let dist_sq : f32 = (dx * dx + dy * dy) as f32;
            if dist_sq == 0f32 {
                continue;
            }
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                element = Some(_element);
            }
        }
        
        if let Some(_element) = element {
            let dist = f32::max(1f32, min_dist_sq.sqrt());
            let move_x = (speed * dx as f32 / dist) as i32;
            let move_y = (speed * dy as f32 / dist) as i32;
            return (move_x, move_y);
        } else {
            return (0, 0);
        }
    }

    fn consume_elements(&self, creature: &mut Creature) -> Vec<usize> {
        let mut elements_to_remove = Vec::new();
        for i in 0..self.elements.len() {
            let _element = &self.elements[i];
            let dx = i32::from(*_element.get_x()) - i32::from(creature.get_x());
            let dy = i32::from(*_element.get_y()) - i32::from(creature.get_y());
            let dist_sq = dx * dx + dy * dy;
            if dist_sq == 0 {
                creature.add_elements(_element.get_kind(), 1);
                elements_to_remove.push(i);
            }
        }
        elements_to_remove
    }

    pub fn get_creatures(&self) -> &Vec<Creature> {
        &self.creatures
    }

    pub fn get_elements(&self) -> &Vec<Element> {
        &self.elements
    }
}