use std::collections::HashMap;
use std::cmp::min;

use std::hash::{Hash, Hasher};

use crate::element::{ElementKind, random_element_kind};
use crate::config::{CRITICAL_PROBABILITY, ELEMENT_NEEDED_MAX, ELEMENT_NEEDED_MIN, REPRODUCE_PROBABILITY, ELEMENT_PRODUCED_MAX, ELEMENT_PRODUCED_MIN, ELEMENT_EJECTED_MAX, ELEMENT_EJECTED_MIN, CRITICAL_THRESHOLD_MULTIPLIER, REPRODUCTION_THRESHOLD_MULTIPLIER, SIZE_MULTIPLIER_EXPONENT_MAX, SIZE_MULTIPLIER_EXPONENT_MIN, SPEED_MULTIPLIER_EXPONENT_MAX, SPEED_MULTIPLIER_EXPONENT_MIN, SENSE_MULTIPLIER_EXPONENT_MAX, SENSE_MULTIPLIER_EXPONENT_MIN, FREQUENCY_EXPONENT_MAX, FREQUENCY_EXPONENT_MIN, MUTATION_PROBABILITY};

use macroquad::rand;

pub struct Gene{
    elements_needed: HashMap<ElementKind, u16>,
    elements_produced: HashMap<ElementKind, u16>,
    elements_ejected: HashMap<ElementKind, u16>,
    frequency_exponent: i16,
    size_multiplier_exponent: i16,
    speed_multiplier_exponent: i16,
    sense_multiplier_exponent: i16,
    target_element: ElementKind,
    critical: bool,
    reproduce: bool
}

impl Hash for Gene {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elements_needed.keys().for_each(|k| k.hash(state));
        self.elements_needed.values().for_each(|v| v.hash(state));
        self.elements_produced.keys().for_each(|k| k.hash(state));
        self.elements_produced.values().for_each(|v| v.hash(state));
        self.elements_ejected.keys().for_each(|k| k.hash(state));
        self.elements_ejected.values().for_each(|v| v.hash(state));
        self.frequency_exponent.hash(state);
        self.size_multiplier_exponent.hash(state);
        self.speed_multiplier_exponent.hash(state);
        self.sense_multiplier_exponent.hash(state);
        self.target_element.hash(state);
        self.critical.hash(state);
        self.reproduce.hash(state);
    }
}

impl Clone for Gene {
    fn clone(&self) -> Self {
        Gene {
            elements_needed: self.elements_needed.iter().map(|(k, v)| (*k, *v)).collect(),
            elements_produced: self.elements_produced.iter().map(|(k, v)| (*k, *v)).collect(),
            elements_ejected: self.elements_ejected.iter().map(|(k, v)| (*k, *v)).collect(),
            frequency_exponent: self.frequency_exponent,
            size_multiplier_exponent: self.size_multiplier_exponent,
            speed_multiplier_exponent: self.speed_multiplier_exponent,
            sense_multiplier_exponent: self.sense_multiplier_exponent,
            target_element: self.target_element,
            critical: self.critical,
            reproduce: self.reproduce
        }
    }
}

impl Gene {
    pub fn get_elements_needed(&self) -> &HashMap<ElementKind, u16> {
        &self.elements_needed
    }

    pub fn get_elements_produced(&self) -> &HashMap<ElementKind, u16> {
        &self.elements_produced
    }

    pub fn get_elements_ejected(&self) -> &HashMap<ElementKind, u16> {
        &self.elements_ejected
    }

    pub fn get_frequency_exponent(&self) -> i16 {
        self.frequency_exponent
    }

    pub fn get_size_multiplier_exponent(&self) -> i16 {
        self.size_multiplier_exponent
    }

    pub fn get_speed_multiplier_exponent(&self) -> i16 {
        self.speed_multiplier_exponent
    }

    pub fn get_sense_multiplier_exponent(&self) -> i16 {
        self.sense_multiplier_exponent
    }

    pub fn get_target_element(&self) -> ElementKind {
        self.target_element
    }

    pub fn is_critical(&self) -> bool {
        self.critical
    }

    pub fn will_reproduce(&self) -> bool {
        self.reproduce
    }

    pub fn mutate(&self) -> Gene {
        if rand::gen_range(0f32, 1f32) < MUTATION_PROBABILITY {
            random_gene()
        } else {
            self.clone()
        }
    }
}

pub fn random_gene() -> Gene {
    let critical: bool  = ((rand::rand() as f32)/u32::MAX as f32) < CRITICAL_PROBABILITY;

    let reproduce: bool = ((rand::rand() as f32)/u32::MAX as f32) < REPRODUCE_PROBABILITY;

    let frequency_exponent = rand::gen_range(FREQUENCY_EXPONENT_MIN, FREQUENCY_EXPONENT_MAX);

    let target_element: ElementKind = random_element_kind();

    let elements_ejected_count = rand::gen_range(ELEMENT_EJECTED_MIN, ELEMENT_EJECTED_MAX);
    let mut elements_ejected = HashMap::new();
    for _ in 0..elements_ejected_count {
        let element = random_element_kind();
        *elements_ejected.entry(element).or_insert(0) += 1;
    }

    let elements_needed_count = rand::gen_range(ELEMENT_NEEDED_MIN, ELEMENT_NEEDED_MAX);
    let mut elements_needed = HashMap::new();
    for _ in 0..elements_needed_count {
        let element = random_element_kind();
        *elements_needed.entry(element).or_insert(0) += 1;
    }

    let multipliers = (if critical { CRITICAL_THRESHOLD_MULTIPLIER } else { 1.0 }) * (if reproduce { REPRODUCTION_THRESHOLD_MULTIPLIER } else { 1.0 });
    let mut threshold = (if elements_needed_count > 0 { elements_needed_count as f32 } else { 1.0 }) * multipliers;

    let elements_produced_count = rand::gen_range(ELEMENT_PRODUCED_MIN, min(ELEMENT_PRODUCED_MAX, threshold as u16));
    let mut elements_produced = HashMap::new();
    for _ in 0..elements_produced_count {
        let element = random_element_kind();
        *elements_produced.entry(element).or_insert(0) += 1;
    }

    threshold /= if elements_produced_count > 0 { elements_produced_count as f32 } else { 1.0 };

    let size_multiplier_exponent = rand::gen_range(SIZE_MULTIPLIER_EXPONENT_MIN, min(SIZE_MULTIPLIER_EXPONENT_MAX, f32::log2(threshold) as i16));

    threshold /= 2f32.powi(size_multiplier_exponent as i32);

    let speed_multiplier_exponent = rand::gen_range(SPEED_MULTIPLIER_EXPONENT_MIN, min(SPEED_MULTIPLIER_EXPONENT_MAX, f32::log2(threshold) as i16));

    threshold /= 2f32.powi(speed_multiplier_exponent as i32);

    let sense_multiplier_exponent = rand::gen_range(SENSE_MULTIPLIER_EXPONENT_MIN, min(SENSE_MULTIPLIER_EXPONENT_MAX, f32::log2(threshold) as i16));

    Gene {
        elements_needed,
        elements_produced,
        elements_ejected,
        frequency_exponent,
        size_multiplier_exponent,
        speed_multiplier_exponent,
        sense_multiplier_exponent,
        target_element,
        critical,
        reproduce
    }
}