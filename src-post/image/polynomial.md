# 다항식

정의 : 각 항이 ax^e 형태로 이루어진 항들이다.
- a : 계수
- x : 변수
- e : 지수

A(x) = 3x^20 + 2x + 1
- 차수(degree) : 가장 큰 지수
    - 위 항은 20차 다항식이다.
- 지수가 0인 항은 1이다.

다항식 연산
A(x) = 3x^2 + 2x + 1
B(x) = 2x^2 + 3x + 1

덧셈 : A(x) + B(x)
- ![polynomial_sum.png](polynomial_sum.png)
- (3 + 2)x^2 + (2 + 3)x + (1 + 1)
- 5x^2 + 5x + 2


곱셈 : A(x) * B(x)
- ![polynomial_mux.png](polynomial_mux.png)
- (3x^2 + 2x + 1)(2x^2 + 3x + 1)
- (3x^2 + 2x + 1)(2x^2) + (3x^2 + 2x + 1)(3x) + (3x^2 + 2x + 1)(1)
- 6x^4 + 13x^3 + 11x^2 + 5x + 1