/*
banker algorithm에 사용되는 은행원의 제약은 아래와 같다
- 기업은 자금을 대출하는 즉시 사용한다
- 기업은 필요한 금액을 대출할 수 있게 되면 반드시 전액 상환한다
- 기업은 전액을 대출할 때까지 사업을 완수하지 못하고 상환하지 못한다
- 대출 이자는 고려하지 않는다
- 신용 창조는 하지 않는다(즉, 은행은 보유 자금 이상을 대출할 수 없다)

위 조건에서 데드락을 방지하는 알고리즘이 banker algorithm이다
*/
// number resource, number thread
// 제네릭 const값으로 객체를 만들면 컴파일 타임에 크기를 확정지어 heap이 아닌 stack에 할당할 수 있다
pub struct Resource<const NRES: usize, const NTH: usize> {
    // number resource 현재 사용가능한 스레드 자원
    available: [usize; NRES],
    // 각 스레드가 필요로 하는 최대 자원량
    max: [[usize; NRES]; NTH],
    // 각 스레드가 점유중인 자원
    allocation: [[usize; NRES]; NTH],
    // need = max - allocation
    // 검사는 req =< need , req =< available 를 통과해야함
    // need와 available은 다른 성질을 가진다
    // Need: 고객이 앞으로 더 필요로 하는 금액 (예: 앞으로 3천만 원 더 필요)
    // Available: 은행이 지금 실제로 줄 수 있는 돈 (예: 현금 1천만 원만 있음)
}

impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
    pub fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Resource {
            available,
            // [[x]; y] 구조의 문법을 따름
            allocation: [[0; NRES]; NTH],
            max,
        }
    }

    // 상태가 안전한 경우 true, 위험한 경우 false를 반환한다
    pub fn is_safe(&self) -> bool {
        // i = 스레드가 확보중인 리소스, j = 스레드
        // finish[i] == false && work[j] >= (self.max[i][j] - self.allocation[i][j])를 만족하는 스레드를 찾는다
        let mut finish = [false; NTH];
        let mut work = self.available.clone();

        loop {
            let mut found = false;
            let mut num_true = 0;
            for (i, alc) in self.allocation.iter().enumerate() {
                if finish[i] {
                    num_true += 1;
                    continue;
                }

                // need[j] = self.max[i][j] - self.allocation[i][j]를 계산하고 모든 리소스 j에 대해 work[j] >= need[j]인지 판정한다
                let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);
                let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
                if is_avail {
                    // 스레드 i가 리소스 확보 가능
                    found = true;
                    finish[i] = true;
                    for (w, a) in work.iter_mut().zip(alc) {
                        *w += *a // 스레드 i가 확보하는 리소스 반환
                    }
                    break;
                }
            }

            if num_true == NTH {
                // 모든 스레드가 리소스 확보 가능하다면 안전
                return true;
            }

            if !found {
                // 스레드가 리소스를 확보할 수 없음
                break;
            }
        }

        false
    }

    // id번째 스레드가 resource를 하나 얻음
    pub fn take(&mut self, id: usize, resource: usize) -> bool {
        if id >= NTH || resource >= NRES || self.available[resource] == 0 {
            return false;
        }

        self.allocation[id][resource] += 1;
        self.available[resource] -= 1;

        if self.is_safe() {
            true
        } else {
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
            false
        }
    }

    fn release(&mut self, id: usize, resource: usize) {
        if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
            return;
        }

        self.allocation[id][resource] -= 1;
        self.available[resource] += 1;
    }
}
