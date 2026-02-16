

```cpp
#include <iostream>
#include <stdio.h>
#include <string.h>
#include <stack>


typedef enum {
	lparen, rparen, plus, minus, times, divide, mod, eos, operand
} precedence;


precedence get_token(const char* expr, char* symbol, int* n) {
	*symbol = expr[(*n)++];
	switch(*symbol) {
		case '(': return lparen;
		case ')': return rparen;		
		case '+': return plus;
		case '-': return minus;
		case '/': return divide;
		case '*': return times;
		case '%': return mod;
		case ' ': return eos;
		default: return operand;
	}
}

void print_token(const precedence token) {
	switch(token) {
		case lparen : std::cout << '(';return;
		case rparen : std::cout << ')';return;
		case plus : std::cout << '+';return;
		case minus : std::cout << '-';return;
		case divide : std::cout << '/';return;
		case times : std::cout << '*';return;
		case mod : std::cout << '%';return;
	}	
}

int eval() {		
	const char* expr = "234*+5% ";
	
	std::stack<int> st;
	int n = 0;
	while(true) {
		char symbol;
		precedence token = get_token(expr, &symbol, &n);
		if (token == eos) {
			break;
		}
		
		if (token == operand) {			
			st.push(symbol - '0');
		} else {
			int op2 = st.top();st.pop();
			int op1 = st.top();st.pop();
			
			switch(token) {
				case plus: st.push(op1 + op2);break;
				case minus: st.push(op1 - op2);break;
				case times: st.push(op1 * op2);break;
				case divide: st.push(op1 / op2);break;
				case mod: st.push(op1 % op2);break;
			}
		}		
	}
	
	int answer = st.top();
	st.pop();
	return answer;
}

int isp[] = { 0, 19, 12, 12, 13, 13, 13, 0}; //in stack precedence priority
int icp[] = {20, 19, 12, 12, 13, 13, 13, 0}; //incoming precedence priority
void postfix() {
	
	const char* expr = "2*3+4 ";
	
	char symbol;
	precedence token;
	int n = 0;
	int top = 0;
	
	std::stack<precedence> st;
	st.push(eos);	
	while (true) {
		token = get_token(expr, &symbol, &n);
		if (token == eos) {
			break;
		}
		if (token == operand){ // 피연산자 토큰은 연산자가 아니기에 스택에 저장될 일 없음. 바로 출력. 
			std::cout << symbol;
		} else if (token == rparen) { // ')' // (, ) 조합 토큰이 나오면 우선 순위 높기에 바로 출력
			while (st.top() != lparen) {
				print_token(st.top());
				st.pop();
			}
			st.pop();// '('
		} else {
			while (isp[st.top()] >= icp[token]) { // 스택에 저장된 토큰들이 우선순위 더 높다면 출력
				print_token(st.top());
				st.pop();
			}				
			st.push(token);
		}
	}
	
	while((token = st.top()) != eos) {
		print_token(token);
		st.pop();
	}
		
	std::cout << std::endl;
}

int main() {	
	postfix();	
	std::cout << "eval = " << eval() << std::endl;		
}
```

후위 표기식으로 변환

a + b * c

| token | 연산자 stack | output |
|-------|-----------|--------|
| a     |           | a      |
| +     | +         | a      |
| b     | +         | ab     |
| *     | +*        | ab     |
| c     | +*        | abc    |
| eos   |           | abc*+  |

a * b + c

| token | 연산자 stack | output |
|-------|-----------|--------|
| a     |           | a      |
| *     | *         | a      |
| b     | *         | ab     |
| +     | +         | ab*    |
| c     | +         | ab*c   |
| eos   |           | ab*c+  |

a*(b+c)/d

| token | 연산자 stack | output |
|-------|-----------|--------|
| a     |           | a      |
| *     | *         | a      |
| (     | *(        | a      |
| b     | *(        | ab     |
| +     | *(+        | ab     |
| c     | *(+        | abc    |
| )     | *         | abc+  |
| /     | /         | abc+*  |
| d     | /         | abc+*d |
| eos   |           | abc+*d/  |


- 연산자 stack 에는 오직 연산자들만 저장될 수 있다. 1, 2 같은 피연산자는 저장되지 않는다. 피연산자는 바로 출력한다.
- 새로운 연산자가 stack에 저장될 때, stack에 저장된 연산자들 중 우선순위가 높은 연산자들을 모두 꺼내어 출력한다.
- ')'가 나오면 '('가 나올 때까지 stack에 저장된 모든 연산자들을 출력한다.
  - '(' 연산자의 우선위가가 가장 높고 그다음 ')' 연산자가 두번째로 높음. 따라서 '(' 가 저장될때는 스택에 '('보다 높은 연산자 없기에 스택에서 연산자를 꺼내지 않고 ')'가 나오면 가장 높은 '(' 연산자 나올때까지 출력.
- eos가 나오면 stack에 저장된 모든 연산자들을 출력한다.