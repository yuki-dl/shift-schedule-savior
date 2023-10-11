use super::timeframe::TimeFrame;
use crate::ga::solve::solve;

#[derive(Debug, Clone, PartialEq)]
pub struct Employee {
    pub id: usize,
    pub timeframe: TimeFrame,
    pub day_off: Vec<u8>
}
impl Employee {
    pub fn new(id: usize, timeframe: TimeFrame, day_off: Vec<u8>) -> Self {
        Self { id, timeframe, day_off }
    }
    pub fn convert(&self) -> Vec<u8> {
        let signal = self.timeframe.to_signal();
        let mut day_off = self.day_off.clone();
        day_off.insert(0, signal);
        day_off
    }
}

pub type DayOff = Vec<u8>;
pub type EmployeeDayoffTable = Vec<DayOff>;

#[derive(Debug, Clone, PartialEq)]
pub struct Roster {
    employee_num: usize,
    pub employees: Vec<Employee>,
    required_people: Vec<usize>,
    pub sum: Vec<Vec<usize>>,
    signal: u8,
}
impl Roster {
    pub fn new(
        employee_num: &String,
        employees: Vec<Employee>,
        required_people: &Vec<String>,
        signal: u8
    ) -> Self {
        let employee_num = employee_num.parse::<usize>().unwrap();
        let required_people = required_people
            .iter()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self { employee_num, employees, required_people, sum: vec![], signal }
    }
    pub fn create(&mut self) -> bool {
        if !self.tests(groupby(&*self.employees)) {
            return false;
        }
        let chunk_size = self.employees[0].day_off.len();
        let day_off = self.employees
            .iter()
            .map(|e| e.convert())
            .collect::<EmployeeDayoffTable>();

        let individual = solve(&day_off, &self.required_people);
        let results = individual.chunks(chunk_size).map(|d| d.to_vec()).collect::<Vec<_>>();
        self.employees
            .iter_mut()
            .zip(results.iter())
            .for_each(|(e, r)| e.day_off = r.to_vec());
        self.calc_sum(results, &day_off);
        true
    }
    fn tests(&self, people: Vec<usize>) -> bool {
        // test1
        let t1 = self.required_people.iter().all(|r| *r == 0);
        if t1 { return false; }

        // test2
        let t2 = self.required_people.iter()
            .map(|r| (0..self.employee_num).contains(&r))
            .all(|b| b == true);
        if !t2 { return false; }
        
        // test3
        let t3 = self.required_people.iter()
            .zip(people.iter())
            .all(|(r, s)| *r <= *s);
        if !t3 { return false; }

        true
    }
    pub fn calc_sum(&mut self, chunks: Vec<Vec<u8>>, day_off: &EmployeeDayoffTable) {
        let days = day_off[0][1..].len();
        for d in 0..days {
            let mut assign = vec![0; 3];
            for i in 0..chunks.len() {
                match day_off[i][0] {
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
            self.sum.push(assign);
        }
    }
}
pub fn groupby<'a>(employees: &'a [Employee]) -> Vec<usize> {
    let mut results = vec![0; 3];
    for emp in employees {
        match emp.timeframe {
            TimeFrame::Morning => results[0] += 1,
            TimeFrame::Afternoon => results[1] += 1,
            TimeFrame::Evening => results[2] += 1,
            TimeFrame::Full1 => {
                results[0] += 1;
                results[1] += 1;
            },
            TimeFrame::Full2 => {
                results[1] += 1;
                results[2] += 1;
            }
        }
    }
    results
}

pub fn create_default_table(employee_num: &String, days: usize) -> Vec<Employee> {
    // 人数を入力すると、作成
    let employee_num = employee_num.parse::<usize>().unwrap();
    (0..employee_num).map(|i| {
        Employee::new(i, TimeFrame::Full1, vec![1u8; days])
    }).collect::<Vec<_>>()
}