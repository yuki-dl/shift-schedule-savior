use rand::{seq::SliceRandom, Rng};
use crate::roster::employee::EmployeeDayoffTable;

#[derive(Debug, Clone)]
pub struct Individual {
    vec: Vec<u8>,
}

impl Individual {
    // 適応度を計算 (値が小さいほど優れている)
    pub fn fitness(&self, e: &mut FitnessEval) -> usize {
        e.eval(&self.vec)
    }
}
impl std::iter::FromIterator<u8> for Individual {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let vec = iter.into_iter().collect();
        Individual { vec }
    }
}
impl std::ops::Deref for Individual {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
impl std::ops::DerefMut for Individual {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

pub struct GA {
    population: Vec<Individual>,
    day_off: EmployeeDayoffTable,
    required_people: Vec<usize>,
    fitness: Option<usize>,
    generation_limit: usize,
    threshold: usize
}
impl GA {
    pub fn new(
        population: Vec<Individual>, day_off: EmployeeDayoffTable, required_people: Vec<usize>,
        generation_limit: usize, threshold: usize
    ) -> Self {
        Self {
            population, day_off, required_people, fitness: None,
            generation_limit, threshold
        }
    }
    pub fn run<R: Rng>(
        &mut self, mut rng: R, selection_size: usize, cross_probability: f64, mutation_probability: f64
    ) -> Individual {
        /*
        - 1. 評価
        - 2. 選択
        - 3. 交叉
        - 4. 突然変異
        */
        let mut generation = 0;
        let mut e = FitnessEval {
            day_off: self.day_off.to_vec(),
            required_people: self.required_people.to_vec(),
            penalty: 0
        };

        loop {
            generation += 1;
            let bestfit = self.with_evaluation(&mut e);
            log::info!("Gen {:03}: Penalty {:3}", generation, self.fitness.unwrap());

            if self.fitness.unwrap() <= self.threshold {
                break bestfit;
            }

            let r = self.with_selection(&mut rng, selection_size, &mut e);
            self.population = self.with_crossover(&mut rng, r, cross_probability);
            self.with_mutation(&mut rng, |g| {*g ^= 1;}, mutation_probability);
            if generation >= self.generation_limit {
                break bestfit;
            }
        }
    }
    // 評価
    fn with_evaluation(&mut self, e: &mut FitnessEval) -> Individual {
        let ind = self.population.iter().min_by_key(|ind| ind.fitness(e)).unwrap();
        self.fitness = Some(ind.fitness(e));
        ind.clone()
    }
    // 選択(tournament)
    fn with_selection<R: Rng>(&self, mut rng: R, selection_size: usize, e: &mut FitnessEval) -> (Vec<&Individual>, Vec<&Individual>) {
        let n = self.population.len();
        let mut a = (0..n).map(|_| {
            self.population
                .choose_multiple(&mut rng, selection_size)
                .min_by_key(|ind| ind.fitness(e))
                .unwrap()
        }).collect::<Vec<_>>();
        let b = a.split_off(n/2);
        (a,b)
    }
    // 交叉(multi)
    fn with_crossover<R: Rng>(
        &self, rng: &mut R, results: (Vec<&Individual>, Vec<&Individual>), cross_probability: f64
    ) -> Vec<Individual> {
        let (a, b) = results;
        let l = a[0].vec.len();
    
        let mut next = Vec::new();
        for (a,b) in a.into_iter().zip(b.into_iter()) {
            if rng.gen::<f64>() <= cross_probability {
                let i = rng.gen_range(1..l-1);
                let j = rng.gen_range(i+1..l);
                next.push(a[..i].iter().chain(b[i..j].iter()).chain(a[j..].iter()).map(|g| g.clone()).collect());
                next.push(b[..i].iter().chain(a[i..j].iter()).chain(b[j..].iter()).map(|g| g.clone()).collect());
            } else {
                next.push(a.clone());
                next.push(b.clone());
            }
        }
        next
    }
    // 突然変異
    fn with_mutation<R, M>(&mut self, rng: &mut R, mutate: M, mutation_probability: f64) where
        R: Rng, M: Fn(&mut u8)
    {
        for ind in self.population.iter_mut() {
            for g in &mut ind.iter_mut() {
                if rng.gen::<f64>() < mutation_probability {
                    mutate(g);
                }
            }
        }
    }
}

// 評価(適合度関数)
/*
制約条件(出勤: 1, 休暇: 0)
- 1. 各営業日における出勤者数をm名で割り当てる
- 2. 各従業員の希望休を反映する
- 3. 飛び石連休を作らない
- 4. 5連勤を作らない
*/

#[derive(Debug, Clone)]
pub struct FitnessEval {
    day_off: EmployeeDayoffTable,
    required_people: Vec<usize>,
    penalty: usize
}
impl FitnessEval {
    fn eval(&mut self, g: &Vec<u8>) -> usize {
        let days = self.day_off[0][1..].len();
        let chunks = g.chunks(days).map(|v| v.to_vec()).collect::<Vec<_>>();
        self.penalty = 0;

        self.eval_assign(days, &chunks); // 1.
        self.eval_dayoff(g); // 2.
        self.eval_consecutive_dayoff(days, &chunks); // 3.
        self.eval_consecutive_5work(days, &chunks); // 4.
        self.penalty
    }
    fn eval_assign(&mut self, days: usize, chunks: &Vec<Vec<u8>>) {
        for d in 0..days {
            let mut assign = vec![0; 3];
            for i in 0..chunks.len() {
                match self.day_off[i][0] {
                    2u8 => assign[0] += chunks[i][d] as usize,
                    3u8 => assign[1] += chunks[i][d] as usize,
                    4u8 => assign[2] += chunks[i][d] as usize,
                    5u8 => {
                        assign[0] += chunks[i][d] as usize;
                        assign[1] += chunks[i][d] as usize;
                    },
                    6u8 => {
                        assign[1] += chunks[i][d] as usize;
                        assign[2] += chunks[i][d] as usize;
                    },
                    _ => unreachable!()
                };
            }
            if assign == *self.required_people {
                continue;
            }
            let p = assign
                .iter()
                .zip(self.required_people.to_vec())
                .map(|(a, r)| *a != r)
                .map(|b| if b {1} else {0} ).collect::<Vec<usize>>();
            self.penalty += p.iter().sum::<usize>();
        }
    }
    fn eval_dayoff(&mut self, g: &Vec<u8>) {
        let day_off = self.day_off.iter().flat_map(|x| x[1..].to_vec()).collect::<Vec<_>>();
        let l = day_off.len();

        let mut eval_arr = vec![0u8; l];
        
        // !(!arr || DAYOFF)を行い、trueになった数だけpenaltyに加算
        for (i, (a, f)) in g.iter().zip(day_off.iter()).enumerate() {
            if !(!a | f) == 1 { eval_arr[i] = 1 }
        }
        self.penalty += eval_arr.iter().sum::<u8>() as usize;
    }
    fn eval_consecutive_dayoff(&mut self, days: usize, chunks: &Vec<Vec<u8>>) {
        /*
        [1, 1, 1] -> 2
        [1, 1, 0] -> 1
        [1, 0, 1] -> 2
        [1, 0, 0] -> 1
        [0, 1, 1] -> 1
        [0, 1, 0] -> 0 (飛び石連休)
        [0, 0, 1] -> 1
        [0, 0, 0] -> 0 (3連休)
        */
        let p = chunks.iter().map(|a| {
            (0..days-2).any(|i| a[i] + a[i+2] == 0)
        }).map(|b| if b {0} else {1} ).collect::<Vec<usize>>();

        self.penalty += p.iter().sum::<usize>();
    }
    fn eval_consecutive_5work(&mut self, days: usize, chunks: &Vec<Vec<u8>>) {
        /*
        [1, 1, 1, 1, 1] (5連勤)
        */
        let p = chunks.iter().map(|a| {
            (0..days-4).any(|i| a[i] + a[i+1] + a[i+2] + a[i+3] + a[i+4] == 5)
        }).map(|b| if b {1} else {0} ).collect::<Vec<usize>>();

        self.penalty += p.iter().sum::<usize>() * 2;
    }
}