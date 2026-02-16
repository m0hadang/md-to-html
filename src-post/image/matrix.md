# 행렬

### 전치
- 행과 열을 바꾼다. 행렬 원소 a[i][j] 를 가져와 a[j][i] 로 저장하면 전치 행렬을 만들 수 있다.

### 행렬의 곱

정의
- 행렬 A : m x n 차원
- 행렬 B : n x p 차원
- 행렬 D : A X B = m x p 차원
- 0 <= i < m, 0 <= j < p 일때 각 원소
  - ![matrix_element.png](matrix_element.png)
  - \\(d_{ij} = \sum_{k=0}^{n-1}a_{ik}b_{kj}\\)

This sentence uses `$` delimiters to show math inline:  $\sqrt{3x-1}+(1+x)^2$


When \\(a \ne 0\\\), there are two solutions to \\(ax^2 + bx + c = 0\\) and they are
\[x = {-b \pm \sqrt{b^2-4ac} \over 2a}.\]
 
도식 
- ![matrix_mux.png](matrix_mux.png)

코드
```
int rowA = m;
int colB = p;
for (int i = 0; i < rowA; i++) {
  for (int j = 0; j < colB; j++) {  
    int sum = 0;
    for (int k = 0; k < n; k++) {
      sum += (a[i][k] * b[k][j]);
    }      
    d[i][j] = sum;
  }
}  
```