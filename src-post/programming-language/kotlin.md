# 코틀린 코딩 스타일
- 세미콜론 필요 없음
- 클래스 이름 : 파스칼 표기법(ClassName)
- 함수, 변수 이름 : 카멜 표기법(functionName)

# var, val
- var : 수정 가능한 변수.
- val : 선언시에만 초기화 가능. 중간에 값을 변경할 수 없음.
- 코틀린은 변수가 초기화 되어있지 않을 경우 컴파일 에러 발생.

# nullable
- ? 이용하여 nullable 변수 선언 가능.

# 프리미티브 타입

```kotlin
fun main() {
    var intValue:Int = 1234
    var longValue:Long = 1234L
    var intValueByHex:Int = 0x1af
    var intValueByBin:Int = 0b10110110
    
    var doubleValue:Double = 123.5
    var doubleValueWithExp:Double = 123.5e10
    var floatValue:Float = 123.5f
}
```

- 코틀린은 8진수 표기 지원하지 않음.
- 프리미티브 타입은 클래스로 구현되어 있음.

# 코틀린의 문자 유니코드
- UTF-16BE.
- 문자 하나 표현시 2byte 사용. 

# 파라미터

코틀린도 다른 언어들 처럼 기본 인자, named 인자, 가변 인자 지원 

```kotlin
pay(amount = 200)
```

```kotlin
fun printlnAll(vararg numbers: Int) {
    for (n in numbers) {
        println(n)
    }
}

fun main() {
    printlnAll(1, 2, 3, 4, 5)
}
```

# 함수 기본 반환 타입(Unit)

함수 반환 타입이 지정되지 않을 경우 Unit 를 반환한다. 

```kotlin
fun a(str: String) { 
  println("function $str")  
  //return 0    <-- 없는 것 같지만 기본적으로 0 반환
}
```

# 단일 표현식

단일 표현식 사용하면 리턴 생략 가능.

```kotlin
fun add(a: Int, b: Int) = a + b
```

**함수형 언어인 코틀린에서 함수는 자료형이 결정된 변수라는 개념으로 접근하는 것이 좋다**

# Any, when

어떤 자료형과도 호환되는 자료형 타입. 코틀린의 최상위 자료형.
- C# 의 object 와 같은 개념

```kotlin
fun main() {
    doWhen(1)
    doWhen("Dimo")
}
fun doWhen(a: Any) {
    when (a) {
        1 -> println("정수 1입니다")
        "Dimo" -> println("Dimo입니다")
        is Long -> println("Long 타입입니다")
        !is String -> println("String 타입이 아닙니다")
        else -> println("어떤 조건도 만족하지 않습니다")
    }   
}
```
```kotlin
var res = when (a) {
    1 -> "정수 1입니다"
    "Dimo" -> "Dimo입니다"
    is Long -> "Long 타입입니다"
    !is String -> "String 타입이 아닙니다"
    else -> "어떤 조건도 만족하지 않습니다"
}
println(res)
```

# 레이블 break, continue

```kotlin
loop@ for (i in 1..10) {
    for (j in 1..10) {
        if (i == 1 && j == 2) break@loop
        println("i: $i, j: $j")
    }
}
```

# 기본 생성자

멤버 변수 초기화.

```kotlin
class Person(name: String, age: Int) {
    val name: String
    val age: Int
    init {
        this.name = name
        this.age = age
        println("name: ${this.name}, age: ${this.age}")
    }
}
fun main() {
    val p = Person("Nick", 10)
}
```

# 보조 생성자 

기본 생성자를 호출하여 변수를 초기화 필요.

```kotlin
class Person(val name: String, val age: Int) {
    constructor(name: String) : this(name, 10) {
        println("name: $name")
    }
}

fun main() {
    val p = Person("Nick")     
}
```

# 클래스 상속

- 클래스는 기본적으로 final(상속 불가) 이다. 
- 클래스 상속/멤버 함수 재정의(override) 위해서 개방(open) 필요.
- 서브 클래스는 베이스 클래스의 생성자를 호출해야 한다.
 
```kotlin
open class Person(val name: String, val age: Int) {
    open fun print() {
        println("name: $name, age: $age")
    }
}
class Student(name: String, age: Int, val id: Int) : Person(name, age) {
    fun printId() {
        println("id: $id")
    }
    override fun print() {
        println("name: $name, age: $age, id: $id")
    }
}

fun main() {
    val p = Student("Nick", 10, 123)
    p.print()
}
```


# 추상 클래스

추상 클래스는 추상 함수를 가지고 있는 클래스. 추상 클래스는 생성자를 가질 수 없다.

```kotlin
abstract class Person(val name: String, val age: Int) {
    abstract fun print()
}

class Student(name: String, age: Int, private val id: Int) : Person(name, age) {
    override fun print() {
        println("name: ${name}, age: ${age}, id: $id")
    }
}
```

# 인터페이스

추상 클래스와 같지만 단 하나가 다르다. 인터페이스는 생성자를 가질 수 없다.

인터페이스 함수 처리
- 구현부가 있는 함수 : open 함수로 간주.
- 구현부가 없는 함수 : abstract 함수로 간주.
- 인터페이스는 별도의 open, abstract 키워드가 없어도 모든 함수를 재정의 가능.

한번에 여러 인터페이스 상속 가능.

```kotlin
interface Runner {
    fun run()
}
interface Eater {
    fun eat() { println("eat food") }
}
class Person(val name: String, val age: Int) : Runner, Eater {
    override fun run() { println("run!!") }
    override fun eat() { println("eat many!!") }
}

fun main() {
    val s = Person("nick", 10)
    s.run()
    s.eat()
}
```


# 프로젝트

프로젝트
- 개발에 필요한 파일들의 물리적인 단위.
- 프로젝트는 모듈로 구성되어 있다.
- 모듈은 여러 소스 파일, 설정 파일, 리소스, 디렉터리 ... 등으로 이루어져 있다.

# 패키지

소스 코드의 '소속'을 지정하기 위한 논리적인 단위.

일반적인 패키지 작명

1. 서비스 도메인을 거꾸로(youtube.com -> com.youtube)
2. 뒤에 프로젝트 명 추가(com.youtube.dimo)
3. 기능별로 세분화
- com.youtube.dimo.base
- com.youtube.dimo.android
- com.youtube.dimo.talk
- ...

package, import

```kotlin
package com.example

import org.springframework.boot.autoconfigure.SpringBootApplication
```

- 코틀린은 자바와 달리 폴더 구조를 패키지 명과 일치 시키지 않아도 된다.
- 코틀린은 파일명과 클래스 이름이 일치하지 않아도 되며 한 파일에 여러개의 클래스 생성 가능.
  - 코틀린은 파일이 아닌 패키지 명으로 구조를 나눈다.
- 패키지를 지정하지 않으면 'default' 패키지에 포함된다.

# 스코프

코틀린에서는 함수 뿐만 아니라 클래스, 패키지도 스코프의 일종으로 취급한다.

# 접근 제한자

패키지 스코프
- public(기본) : 어떤 패키지에서도 접근 가능.
- internal : 같은 모듈 내에서만 접근 가능.
- private : 같은 파일 내에서만 접근 가능.

클래스 스코프
- public(기본) : 클래스 외부에서 접근 가능.
- private : 클래스 내부에서만 접근 가능.
- protected : 클래스 자신과 상속받은 클래스에서 접근 가능.

# 변수 사용시 주의

val 는 할당된 객체를 바꿀 수 없을 뿐이지 객체 내부 속성을 바꾸지 못하는 것은 아니다