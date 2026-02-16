# 순환 알고리즘

함수가 그 수행이 완료 되기 전에 자기 자신을 다시 호출 한다.

Ackermann 함수나 Factorial 연산 같은 특수한 부류의 문제에만 사용할 수 있다 생각하는데 이는 옳지 않다. 

Fibonacci
```
n! / m!(n-m)!
```

이항 계수
```
        |n - 1|     |n - 1|
n / m =         +  
        |m   |      |m - 1|
```

if else, while 문을 지원하는 어떤 언어 순환 알고리즘을 작성할 수 있기다.

순환 방식의 알고리즘을 작성 시 중요 사항이 있다.

1. 경계 조건 : 순환 호출이 무한히 계속되지 않도록 하는 조건이 필요하다.
2. 해답을 향한 다음 단계 : 매 호출 마다 해답을 향해 진행되도록 구현해야 한다.

이진 검색

```cpp
#define COMPARE(x,y) (((x) < (y)) ? -1 : ((x) == (y)) ? 0 : 1)
int binsearch(int list[], int searchnum, int left, int right) {	
	if (left > right) return -1;
	
	int middle = (left + right) / 2;
	switch (COMPARE(list[middle], searchnum)) {
		case -1: return binsearch(list, searchnum, middle + 1, right);
		case 0: return middle;
		case 1: return binsearch(list, searchnum, left, middle - 1);
	}	
}
```

- 경계 조건 : left, right 의 상호 관계이다.
- 해답을 향한 다음 단계 : middle 기준으로 탐색 범위를 2분 한다.



순열

```cpp
#define SWAP(x,y) ({char temp = x;x = y;	y = temp;})
void perm(char* list, int begin_i, int n) {	
	if (begin_i == n) {		
		for (int i = 0; i < n; i++) {
			std::cout << list[i];
		}
		std::cout << std::endl;
		return;
	}
	
	for (int i = begin_i; i < n; i++) {			
		SWAP(list[begin_i], list[i]);
		perm(list, begin_i + 1, n);
		SWAP(list[begin_i], list[i]);			
	}	
}
```

순열은 n 개의 주어진 원소에 대해 n! 개의 상이한 순서로 배열하는 것이다.

n! 에서 순환의 실마리를 얻을 수 있다. 이것은 n - 1 개의 원소에 동작하는 알고리즘이 있다면 n 개의 원소에도 동작함을 의미한다.