use rand::{SeedableRng, Rng};
use super::ga::{Individual, tournament_selection, crossover, mutation};
use super::employee::Employee;

const N: usize = 300; // 個体数

const GENERATION: usize = 200; // 繰り返す世代数
const THRESHOLD: usize = 0; // 閾値


pub fn solve(emps: &Vec<Employee>, required_people: usize) -> Individual {
    let day_off = emps.iter().map(|e| e.day_off.clone()).collect::<Vec<_>>();

    let l = day_off[0].len() * day_off.len(); // 遺伝子長
    let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(0); // PRNGの初期化

    // 初期個体群の生成
    let mut population = (0..N).map(|_| {
        (0..l).map(|_| rng.gen_range(0u8..2)).collect::<Individual>()
    }).collect::<Vec<_>>();

    population = evolve(population, rng, &day_off, required_people);

    let bestfit = population.iter().min_by_key(|ind| ind.fitness(&day_off, required_people)).unwrap().clone();
    bestfit
}

fn evolve<R: Rng>(population: Vec<Individual>, mut rng: R, day_off: &Vec<Vec<u8>>, required_people: usize) -> Vec<Individual> {
    let mut population = population;
    let mut generation = 0;

    while generation < GENERATION {
        let bestfit = population.iter().min_by_key(|ind| ind.fitness(&day_off, required_people)).unwrap();
        log::info!("Gen {:03}: Penalty {:3}", generation, bestfit.fitness(&day_off, required_people));

        if bestfit.fitness(&day_off, required_people) <= THRESHOLD {
            break;
        }

        // 世代情報を更新
        generation += 1;

        // トーナメント選択を行い2つのグループに分ける
        let (a, b) = tournament_selection(&population, &mut rng, day_off.to_vec(), required_people);

        // 二点交叉により次世代を生成
        let mut next = crossover(a, b, &mut rng);

        // 突然変異 (1とのXORをとる)
        mutation(&mut next, &mut rng, |g| {*g ^= 1;});

        population = next;
    }
    population
}