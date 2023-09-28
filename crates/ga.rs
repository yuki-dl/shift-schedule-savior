use rand::{seq::SliceRandom, Rng};

// パラメータ
const TOURNAMENT_SIZE: usize = 8; // トーナメントサイズ
const CROSS_PROB: f64 = 0.5; // 交叉確率
const MUTATION_PROB: f64 = 0.01; // 突然変異確率

#[derive(Debug, Clone)]
pub struct Individual {
    vec: Vec<u8>,
}

impl Individual {
    // 適応度を計算 (値が小さいほど優れている)
    pub fn fitness(&self, day_off: &Vec<Vec<u8>>, required_people: usize) -> usize {
        let days = day_off[0].len();
        let arr = self.vec.chunks(days).collect::<Vec<_>>();
        let day_off = day_off.iter().flat_map(|x| x.clone()).collect::<Vec<_>>();

        let penalty = Evaluation::check(&self.vec, &arr, required_people, day_off);
        penalty
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

// 進化(選択)
pub fn tournament_selection<'a, R: Rng>(
    population: &'a [Individual], rng: &mut R, day_off: Vec<Vec<u8>>, required_people: usize
) -> (Vec<&'a Individual>, Vec<&'a Individual>) {
    let n = population.len();
    let mut a = (0..n).map(|_| {
        population
            .choose_multiple(rng, TOURNAMENT_SIZE)
            .min_by_key(|ind| ind.fitness(&day_off, required_people))
            .unwrap()
    }).collect::<Vec<_>>();
    let b = a.split_off(n/2);
    (a,b)
}

// 進化(交叉)
pub fn crossover<R: Rng>(
    a: Vec<&Individual>, b: Vec<&Individual>, rng: &mut R
) -> Vec<Individual> {
    let l = a[0].vec.len();
    
    let mut next = Vec::new();
    for (a,b) in a.into_iter().zip(b.into_iter()) {
        if rng.gen::<f64>() <= CROSS_PROB {
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

// 進化(突然変異)
pub fn mutation<R, M>(pop: &mut [Individual], rng: &mut R, mutate: M) where
    R: Rng, M: Fn(&mut u8)
{
    for ind in pop.iter_mut() {
        for g in &mut ind.iter_mut() {
            if rng.gen::<f64>() < MUTATION_PROB {
                mutate(g);
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

pub struct Evaluation<'a> {
    gene: &'a Vec<u8>,
    penalty: usize
}
impl<'a> Evaluation<'a> {
    pub fn check(gene: &'a Vec<u8>, arr: &Vec<&[u8]>, required_people: usize, day_off: Vec<u8>) -> usize {
        let mut ev = Self { gene, penalty: 0 };
        
        ev.check_assign(&arr, required_people); // 1. 
        ev.check_dayoff(day_off); // 2.
        ev.check_consecutive_dayoff(&arr); // 3.
        ev.check_consecutive_5work(&arr); // 4.
            
        ev.penalty
    }
    fn check_assign(&mut self, arr: &Vec<&[u8]>, required_people: usize) {
        let days = arr[0].len();

        for i in 0..days {
            let mut assign_num: u8 = 0;
            arr.iter().for_each(|&g| assign_num += g[i]);
            if assign_num != required_people as u8 {
                self.penalty += 1
            }
        }
    }
    fn check_dayoff(&mut self, day_off: Vec<u8>) {
        let l = day_off.len();
        let mut eval_arr = vec![0u8; l];

        // !(!arr || DAYOFF)を行い、trueになった数だけpenaltyに加算
        for (i, (a, f)) in self.gene.iter().zip(day_off.iter()).enumerate() {
            if !(!a | f) == 1 { eval_arr[i] = 1 }
        }
        self.penalty += eval_arr.iter().sum::<u8>() as usize;
    }
    fn check_consecutive_dayoff(&mut self, arr: &Vec<&[u8]>) {
        /*
        [1，1，1] -> 2
        [1，1，0] -> 1
        [1，0，1] -> 2
        [1，0，0] -> 1
        [0，1，1] -> 1
        [0，1，0] -> 0 (飛び石連休)
        [0，0，1] -> 1
        [0，0，0] -> 0 (※3連休)
        */
        let days = arr[0].len();

        let penalty_flag = arr.iter().map(|a| {
            (0..days-2).any(|i| a[i] + a[i+2] == 0)
        }).any(|b| b == true);

        if penalty_flag {
            self.penalty *= 2
        }
    }
    fn check_consecutive_5work(&mut self, arr: &Vec<&[u8]>) {
        let days = arr[0].len();

        let penalty_flag = arr.iter().map(|a| {
            (0..days-4).any(|i| a[i] + a[i+1] + a[i+2] + a[i+3] + a[i+4] == 5)
        }).any(|b| b == true);

        if penalty_flag {
            self.penalty *= 3
        }
    }
}