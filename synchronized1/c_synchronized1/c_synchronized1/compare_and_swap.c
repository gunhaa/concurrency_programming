#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

// 논리적으로는 아토믹하지만 CPU관점에서 전혀 아토믹하지 않다
// 레이스 컨디션이 발생할수있다
bool compare_and_swap(uint64_t* p, uint64_t val, uint64_t newval) {
	if (*p != val) {
		return false;
	}
	*p = newval;
	return true;
}

// gcc/ clang에서 cpu atomic한 연산을 지원한다
bool compare_and_swap_atomic(uint64_t* p, uint64_t val, uint64_t newval) {
    return __sync_bool_compare_and_swap(p, &val, newval);
}

int main() {
    uint64_t x = 100;
    uint64_t old_val = 100;
    uint64_t new_val = 200;

    if (compare_and_swap(&x, old_val, new_val)) {
        printf("CAS succeeded, x = %llu\n", x);
    }
    else {
        printf("CAS failed, x = %llu\n", x);
    }

    return 0;
}