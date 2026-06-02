use macroquad::color::Color;
use macroquad::color_u8;
use macroquad::rand;

use base64::{engine::general_purpose::STANDARD, Engine as _};

use std::collections::HashMap;

use crate::config::{FUNDAMENTAL_ELEMENT, FUNDAMENTAL_ELEMENT_INITIAL_AMOUNT, FUNDAMENTAL_ELEMENT_CONSUMPTION_AMOUNT, GRID_HEIGHT, GRID_WIDTH, GENE_MAX, GENE_MIN, ELEMENTS_MAX, ELEMENTS_MIN, SIZE_BASE_MULTIPLIER, SPEED_BASE_MULTIPLIER, SENSE_BASE_MULTIPLIER, VERBOSE};
use crate::gene::{Gene, random_gene};
use crate::element::{ElementKind, random_element_kind, Element};
use crate::utils::get_around;

use std::hash::{DefaultHasher, Hash, Hasher};

use std::cmp::min;

pub struct CreatureResult {
    pub alive: bool,
    pub offsprings: Vec<Creature>,
    pub consumed_element_indices: Vec<usize>,
    pub ejected_elements: Vec<Element>
}

impl CreatureResult {
    fn new() -> Self {
        CreatureResult { alive: true, offsprings: Vec::new(), consumed_element_indices: Vec::new(), ejected_elements: Vec::new() }
    }
}

struct GeneResult {
    produced: bool,
    ejected_elements: Vec<Element>
}

pub struct Creature {
    x: u16,
    y: u16,
    genes: Vec<Gene>,
    elements: HashMap<ElementKind, u16>,
    color: Color,
    genome_hash: i32
}

impl Creature {
    fn new(x: u16, y: u16, genes: Vec<Gene>, elements: HashMap<ElementKind, u16>) -> Self {
        let mut hasher = DefaultHasher::new();
        genes.iter().for_each(|gene| gene.hash(&mut hasher));
        let mut new_elements = elements;
        *new_elements.entry(FUNDAMENTAL_ELEMENT).or_insert(0) += FUNDAMENTAL_ELEMENT_INITIAL_AMOUNT;
        let genome_hash = hasher.finish() as i32;
        let color = color_u8!(genome_hash%255, (genome_hash/255)%255, (genome_hash/65536)%255, 255);
        Creature { x, y, genes, elements: new_elements, color, genome_hash }
    }

    fn compute_move(&mut self, target: ElementKind, speed: f32, sense: f32, elements: &Vec<Element>) -> (i32, i32) {
        let mut min_dist_sq = sense * sense;
        let mut element: Option<&Element> = None;
        let mut dx: i32 = 0;
        let mut dy: i32 = 0;

        for _element in elements {
            if _element.get_kind().ne(&target) {
                continue;
            }
            dx = i32::from(*_element.get_x()) - i32::from(self.x);
            dy = i32::from(*_element.get_y()) - i32::from(self.y);
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

    fn consume_elements(&mut self, elements: &Vec<Element>) -> Vec<usize> {
        let mut elements_to_remove = Vec::new();
        for i in 0..elements.len() {
            let _element = &elements[i];
            let dx = i32::from(*_element.get_x()) - i32::from(self.x);
            let dy = i32::from(*_element.get_y()) - i32::from(self.y);
            let dist_sq = dx * dx + dy * dy;
            if dist_sq == 0 {
                self.add_elements(_element.get_kind(), 1);
                elements_to_remove.push(i);
            }
        }
        elements_to_remove
    }

    pub fn produce(&mut self, tick: u128, elements: &Vec<Element>) -> CreatureResult {
        let mut result = CreatureResult::new();
        self.remove_elements(FUNDAMENTAL_ELEMENT, FUNDAMENTAL_ELEMENT_CONSUMPTION_AMOUNT);
        if self.get_elements()[&FUNDAMENTAL_ELEMENT] == 0 {
            result.alive = false;
            return result;
        }
        let mut produced_gene_indices : Vec<usize> = Vec::new();
        let mut critical_produced_gene_indices : Vec<usize> = Vec::new();
        for gene in self.genes.clone() {
            if gene.get_frequency_exponent() < 0 && tick % (2u32.pow((-gene.get_frequency_exponent()) as u32)) as u128 != 0 {
                continue;
            } else if gene.get_frequency_exponent() >= 0 {
                for i in 0..(2usize.pow(gene.get_frequency_exponent() as u32)) {
                    let gene_result = self.produce_gene(&gene);
                    if gene_result.produced {
                        result.ejected_elements.extend(gene_result.ejected_elements);
                        produced_gene_indices.push(i);
                        if gene.is_critical() {
                            critical_produced_gene_indices.push(i);
                        }
                        if gene.will_reproduce() {
                            let (new_x, new_y) = get_around(self.x, self.y);
                            let new_creature = self.reproduce(new_x, new_y);
                            if VERBOSE {
                                println!("Creature at ({}, {}) reproduced to ({}, {}). (Genome hash: {})", self.x, self.y, new_x, new_y, STANDARD.encode(self.genome_hash.to_string()));
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
        for i in produced_gene_indices.clone() {
            let gene = &self.genes[i];
            size *= 2f32.powi(gene.get_size_multiplier_exponent() as i32);
            speed *= 2f32.powi(gene.get_speed_multiplier_exponent() as i32);
            sense *= 2f32.powi(gene.get_sense_multiplier_exponent() as i32);
        }
        if size < 1f32 {
            result.alive = false;
            return result;
        }
        let target_element: Option<ElementKind>;
        if critical_produced_gene_indices.is_empty() {
            if produced_gene_indices.is_empty() {
                target_element = None;
            } else {
                target_element = Some(self.genes[produced_gene_indices[rand::gen_range(0, produced_gene_indices.len())]].get_target_element());
            }
        } else {
            target_element = Some(self.genes[critical_produced_gene_indices[rand::gen_range(0, critical_produced_gene_indices.len())]].get_target_element());
        }
        if target_element.is_some() {
            let (move_x, move_y) = self.compute_move(target_element.unwrap(), speed, sense, elements);
            let old_x = self.x;
            let old_y = self.y;
            let new_x = old_x.saturating_add_signed(move_x as i16);
            let new_y = old_y.saturating_add_signed(move_y as i16);
            self.x = if new_x < GRID_WIDTH { new_x } else { GRID_WIDTH - 1 };
            self.y = if new_y < GRID_HEIGHT { new_y } else { GRID_HEIGHT - 1 };
        }
        let consumed_indices = self.consume_elements(elements);
        result.consumed_element_indices = consumed_indices;
        return result;
    }

    fn produce_gene(&mut self, gene: &Gene) -> GeneResult {
        for (kind, quantity) in gene.get_elements_needed().iter() {
            if self.elements.get(kind).copied().unwrap_or(0) < *quantity {
                let luck = rand::gen_range(0.0, 1.0) < 0.5;
                if (gene.is_critical() && !luck) || !gene.is_critical() || self.elements.get(kind).copied().unwrap_or(0)==0 {
                    return GeneResult { produced: false, ejected_elements: Vec::new() };
                }
            }
        }

        for (kind, quantity) in gene.get_elements_needed().iter() {
            let consumed_quantity = min(*quantity, self.elements.get(kind).copied().unwrap_or(0));
            self.remove_elements(*kind, consumed_quantity);
        }

        for (kind, quantity) in gene.get_elements_produced().iter() {
            self.add_elements(*kind, *quantity);
        }

        let mut ejected_elements = Vec::new();

        for (kind, quantity) in gene.get_elements_ejected().iter() {
            let ejected_quantity = min(*quantity, self.elements.get(kind).copied().unwrap_or(0));
            self.remove_elements(*kind, ejected_quantity);
            for _ in 0..ejected_quantity {
                let (new_x, new_y) = get_around(self.x, self.y);
                ejected_elements.push(Element::new(new_x, new_y, *kind));
            }
        }
        return GeneResult { produced: true, ejected_elements };
    }

    pub fn reproduce(&mut self, x: u16, y: u16) -> Creature {
        let new_genes = self.genes.iter().map(|gene| gene.to_owned()).collect();
        let new_elements = self.elements.iter().map(|(k, v)| (*k, *v/2)).collect();
        self.elements.iter_mut().for_each(|(_, v)| {*v = *v/2;});
        Creature::new(x, y, new_genes, new_elements)
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_genome_hash(&self) -> i32 {
        self.genome_hash
    }

    pub fn get_elements(&self) -> &HashMap<ElementKind, u16> {
        &self.elements
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn add_elements(&mut self, element_kind: ElementKind, amount: u16) {
        *self.elements.entry(element_kind).or_insert(0) += amount;
    }

    pub fn remove_elements(&mut self, element_kind: ElementKind, amount: u16) {
        let current_amount = self.elements.entry(element_kind).or_insert(0);
        *current_amount = current_amount.saturating_sub(amount);
    }
}

impl Clone for Creature {
    fn clone(&self) -> Self {
        Creature::new(self.x, self.y, self.genes.iter().map(|gene| gene.clone()).collect(), self.elements.iter().map(|(k, v)| (*k, *v)).collect())
    }
}

pub fn random_creature() -> Creature{
    let x = rand::gen_range(0, GRID_WIDTH);
    let y = rand::gen_range(0, GRID_HEIGHT);
    let gene_count = rand::gen_range(GENE_MIN, GENE_MAX);
    let genes = (0..gene_count).map(|_| random_gene()).collect();
    let element_count = rand::gen_range(ELEMENTS_MIN, ELEMENTS_MAX);
    let mut elements = HashMap::new();
    for _ in 0..element_count {
        let element = random_element_kind();
        *elements.entry(element).or_insert(0) += 1;
    }
    Creature::new(x, y, genes, elements)
}