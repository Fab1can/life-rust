use macroquad::color::Color;
use macroquad::color_u8;
use macroquad::rand;

use std::collections::HashMap;

use crate::config::{FUNDAMENTAL_ELEMENT, FUNDAMENTAL_ELEMENT_INITIAL_AMOUNT, GRID_HEIGHT, GRID_WIDTH, GENE_MAX, GENE_MIN, ELEMENTS_MAX, ELEMENTS_MIN};
use crate::gene::{Gene, random_gene};
use crate::element::{ElementKind, random_element_kind};

use std::hash::{DefaultHasher, Hash, Hasher};

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

    pub fn reproduce(&mut self, x: u16, y: u16) -> Creature {
        let new_genes = self.genes.iter().map(|gene| gene.clone()).collect();
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

    pub fn get_genes(&self) -> &Vec<Gene> {
        &self.genes
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn set_x(&mut self, x: u16) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: u16) {
        self.y = y;
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