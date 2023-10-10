use rand::{SeedableRng, Rng};
use super::build::{GA, Individual};
use crate::roster::employee::EmployeeDayoffTable;

const N: usize = 200; // 個体数

const GENERATION: usize = 500; // 繰り返す世代数
const THRESHOLD: usize = 0; // 閾値

// パラメータ
const TOURNAMENT_SIZE: usize = 4; // トーナメントサイズ
const CROSS_PROB: f64 = 0.8; // 交叉確率
const MUTATION_PROB: f64 = 0.002; // 突然変異確率

pub fn solve(day_off: &EmployeeDayoffTable, required_people: &Vec<usize>) -> Individual {
    let l = day_off[0][1..].len() * day_off.len(); // 遺伝子長
    let mut rng = rand_xoshiro::Xoshiro256StarStar::from_entropy(); // PRNGの初期化

    // 初期個体群の生成
    let population = (0..N).map(|_| {
        (0..l).map(|_| rng.gen_range(0u8..2)).collect::<Individual>()
    }).collect::<_>();

    let bestfit = GA::new(
        population, day_off.to_vec(), required_people.to_vec(), GENERATION, THRESHOLD)
        .run(&mut rng, TOURNAMENT_SIZE, CROSS_PROB, MUTATION_PROB);
    
    bestfit
}