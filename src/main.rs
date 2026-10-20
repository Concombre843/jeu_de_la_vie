use std::collections::HashMap;

use egui_macroquad::{draw, egui, ui};
use macroquad::prelude::*;

const TIMER_RATE: f32 = 0.01;
const CELL_SIZE: f32 = 4.0;
const MAP_SIZE: i32 = 150;
const PANEL_WIDTH: f32 = MAP_SIZE as f32;
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Clone, Copy, Debug)]
enum Cell {
    Alive,
    Dead,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

struct Timer {
    max: f32,
    current: f32,
}
impl Timer {
    fn new(max: f32) -> Timer {
        Timer { max, current: max }
    }
    fn tick(&mut self) {
        self.current -= get_frame_time();
    }
    fn finished(&self) -> bool {
        self.current <= 0.0
    }
    fn reset(&mut self) {
        self.current = self.max
    }
}

#[macroquad::main(config)]

async fn main() {
    let mut map: HashMap<Position, Cell> = HashMap::new();
    create_map(&mut map);
    let mut turn = 0;

    let mut timer = Timer::new(TIMER_RATE);
    loop {
        timer.tick();

        if timer.finished() {
            turn += 1;
            timer.reset();

            let mut next = HashMap::new();
            for (p, _) in &map {
                next.insert(*p, process_cell(p, &map));
            }
            map = next;
        }
        clear_background(BLACK);

        draw_map(&mut map);
        draw_stats(turn, &map);
        next_frame().await
    }
}

fn process_cell(pos: &Position, map: &HashMap<Position, Cell>) -> Cell {
    match read_cell(&pos, &map) {
        Some(cell) => {
            let count = count_alive_around(&pos, &map);

            if (matches!(cell, Cell::Alive) && do_survive(count)) || do_born(count) {
                Cell::Alive
            } else if do_die(count) {
                Cell::Dead
            } else {
                Cell::Dead
            }
        }
        None => Cell::Dead,
    }
}

//=======================

fn do_survive(count: usize) -> bool {
    count == 2 || count == 3
}

fn do_die(count: usize) -> bool {
    count < 2 || count > 3
}

fn do_born(count: usize) -> bool {
    count == 3
}

fn read_cell(pos: &Position, map: &HashMap<Position, Cell>) -> Option<Cell> {
    match map.get(pos) {
        Some(s) => Some(*s),
        None => None,
    }
}
fn count_alive_around(target_pos: &Position, map: &HashMap<Position, Cell>) -> usize {
    DIRECTIONS
        .iter()
        .map(|d| Position::new(target_pos.x + d.0, target_pos.y + d.1))
        .filter(|p| match read_cell(p, &map) {
            Some(Cell::Alive) => true,
            Some(Cell::Dead) => false,
            None => false,
        })
        .count()
}

fn create_map(map: &mut HashMap<Position, Cell>) {
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            map.insert(Position { x, y }, Cell::Dead);
        }
    }

    rand::srand(miniquad::date::now() as u64);
    let nb_alives = MAP_SIZE * MAP_SIZE / 3;
    for _ in 0..nb_alives {
        let pos = Position::new(rand::gen_range(0, MAP_SIZE), rand::gen_range(0, MAP_SIZE));
        map.insert(pos, Cell::Alive);
    }
}
//=====================================

fn draw_stats(turn: i32, map: &HashMap<Position, Cell>) {
    let alives = map.values().filter(|c| matches!(c, Cell::Alive)).count();
    ui(|ctx| {
        egui::SidePanel::right("panel")
            .exact_width(PANEL_WIDTH)
            .show(ctx, |ui| {
                ui.label(egui::RichText::new(format!("turn : {}", turn)).size(24.0));
                ui.label(egui::RichText::new(format!("alives : {}", alives)).size(24.0));
            });
    });
    draw();
}

fn draw_map(map: &mut HashMap<Position, Cell>) {
    map.iter()
        .filter(|(_, c)| matches!(c, Cell::Alive))
        .for_each(|(p, _)| {
            let color = WHITE;
            draw_cell(*p, color);
        });
}

fn draw_cell(pos: Position, color: Color) {
    draw_rectangle(
        pos.x as f32 * CELL_SIZE,
        pos.y as f32 * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE,
        color,
    );
}

fn config() -> Conf {
    Conf {
        window_resizable: false,
        window_width: (CELL_SIZE * MAP_SIZE as f32 + PANEL_WIDTH) as i32,
        window_height: (CELL_SIZE * MAP_SIZE as f32) as i32,
        ..Default::default()
    }
}
