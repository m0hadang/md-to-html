# strong

```html
strong tag는 Text에 <strong>강조</strong> 를 해준다.
```

# h

head

HTML 문단 제목을 만들 수 있다.

```html
<h1>This is heading 1</h1>
<h2>This is heading 2</h2>
<h3>This is heading 3</h3>
<h4>This is heading 4</h4>
<h5>This is heading 5</h5>
<h6>This is heading 6</h6>
```

# br

break line

줄 바꿈. `</br>` 로 태그를 닫지 않아도 된다.

```html
First Line<br><br>Second Line
```

# p

paragraph

단락을 나누기 위해 줄바꿈(br)을 사용하는 것을 고려할 수 있다. 하지만 HTML에는 단락 표현을 위한 전용 태그(paragraph tag)가 별도로 존재한다.

```html
<p>
    Hypertext Markup Language (HTML) is the standard markup language for...      
</p>
```

단락을 표현하기 위해 br 대신 p 를 사용해야 하는 이유
- 단락에는 단락 태그를 사용하는 것이 웹페이지를 정보로서 보다 가치있게 해준다.
  - ex1) 다른 프로그램 역시 p 태그를 단락으로 인식하여 처리하기에 호환성 향상
  - ex2) p 태그를 통해서 단락의 경계를 분명히 하면서 CSS를 통해서 p 태그의 디자인을 자유롭게 변경 
- br 태그는 단순히 줄 바꿈을 의미 할 뿐이다. br 태그는 쓰는만큼 줄바꿈이 되기 때문에 원하는 만큼 간격을 줄 수 있는 장점이 있지만 CSS라는 기술이 있다면 p태그의 한계를 극복 할 수 있다.

# img

image

이미지 삽입. src attribute 에 이미지 경로 설정.

```html
<img src="https://s3-ap-northeast-2.amazonaws.com/opentutorials-user-file/module/3135/7648.png">

로컬 파일 경로도 사용 가능
<img src="C:\Users\daewo\Desktop\3391805_1496218621340.jpeg">
```

# li

list

목차.

```html
<li>1. HTML</li>
<li>2. CSS</li>
<li>3. JavaScript</li>
```

# ul

unordered list

목록끼리 경계 구분. 

```html
<ul>
  <li>1. HTML</li>
  <li>2. CSS</li>
  <li>3. JavaScript</li>
</ul>
<ul>
  <li>egoing</li>
  <li>k8805</li>
  <li>sorialgi</li>
</ul>
```

# ol

ordered list

목록끼리 경계 구분. 자동으로 번호를 붙여 순서를 매김. 

```html
<ol>
  <li>HTML</li>
  <li>CSS</li>
  <li>JavaScript</li>
</ol>
```

# table

테이블

**기본 사용**
- tr : table row
- td : table data

```html
<table>
  <tr>
    <td>이름</td>     <td>성별</td>   <td>주소</td>
  </tr>
  <tr>
    <td>최진혁</td>  <td>남</td>      <td >청주</td>
  </tr>
  <tr>
    <td>최유빈</td>  <td>여</td>      <td>청주</td>
  </tr>
</table>
```
<table>
  <tr>
    <td>이름</td>     <td>성별</td>   <td>주소</td>
  </tr>
  <tr>
    <td>최진혁</td>  <td>남</td>      <td >청주</td>
  </tr>
  <tr>
    <td>최유빈</td>  <td>여</td>      <td>청주</td>
  </tr>
</table>

**테이블 구조화**
- thead(table head) : 컬럼 제목들이다. 해당 row를 가장 위에 둔다.
- tbody(table b~ody) : 해당 row를 중간에 둔다.
- tfoot(table foot) : 해당 row를 가장 아래로 둔다.
- th(table head) : 제목으로서 표현할 때 사용

```html
<table>
  <thead>
    <tr>
      <th>이름</th>     <th>성별</th>   <th>주소</th> <th>회비</th>
    </tr>
  </thead>
  <tbody>
  <tr>
    <td>최진혁</td>  <td>남</td>      <td >청주</td> <td>1000</td>
  </tr>
  <tr>
    <td>최유빈</td>  <td>여</td>      <td>청주</td> <td>500</td>
  </tr>
  </tbody>
  <tfoot>
    <tr>
      <td>합계</td>  <td></td>      <td></td> <td>1500</td>
    </tr>
  </tfoot>
</table>
```

<table>
  <thead>
    <tr>
      <th>이름</th>     <th>성별</th>   <th>주소</th> <th>회비</th>
    </tr>
  </thead>
  <tbody>
  <tr>
    <td>최진혁</td>  <td>남</td>      <td >청주</td> <td>1000</td>
  </tr>
  <tr>
    <td>최유빈</td>  <td>여</td>      <td>청주</td> <td>500</td>
  </tr>
  </tbody>
  <tfoot>
    <tr>
      <td>합계</td>  <td></td>      <td></td> <td>1500</td>
    </tr>
  </tfoot>
</table>

**병합**
- colspan, rowspan

```html
<table>
  <thead>
    <tr>
      <th>이름</th>     <th>성별</th>   <th>주소</th> <th>회비</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>최진혁</td>  <td>남</td>      <td rowspan="2">청주</td> <td>1000</td>
    </tr>
    <tr>
      <td>최유빈</td>  <td>여</td>      <td>500</td>
    </tr>
  </tbody>
  <tfoot>
    <tr>
      <td colspan="3">합계</td>      <td>1500</td>
    </tr>
  </tfoot>
</table>
```

<table>
  <thead>
    <tr>
      <th>이름</th>     <th>성별</th>   <th>주소</th> <th>회비</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>최진혁</td>  <td>남</td>      <td rowspan="2">청주</td> <td>1000</td>
    </tr>
    <tr>
      <td>최유빈</td>  <td>여</td>      <td>500</td>
    </tr>
  </tbody>
  <tfoot>
    <tr>
      <td colspan="3">합계</td>      <td>1500</td>
    </tr>
  </tfoot>
</table>

# title

웹 페이지 타이틀 설정. 타이틀 태그를 이용해 웹페이지 제목을 지정하면 더 이상 웹브라우저의 탭에 파일 이름이 안 뜨고 title이 뜨게 된다.

> 검색엔진이 웹페이지를 분석할 때 가장 중요하게 생각하는 태그이기 때문에 title 태그를 쓰지 않으면 정말 큰 손해이다

```html
<title>WEB1 - html</title>
```

# body

웹페이지에서 본문 지정.

```html
<head>
  <title>WEB1 - html</title>
  <meta charset="utf-8">
</head>
<body>
  <h1>HTML</h1>
  <p>
    Hypertext Markup Language (HTML) is the standard 
  </p>
  <p style="margin-top:45px;">
    HTML elements are the building blocks of HTML pages. With HTML constructs, images and 
  </p>
</body>
```

# html, DOCTYPE

html : body 태그와 head 태그를 감싸는 tag.

doctype : HTML 코드를 작성할 때 항상 `<!DOCTYPE html>` 을 먼저 적고 시작한다. 웹브라우저가 이 HTML 문서가 어떤 표준을 따르고 있는 tag 들인지 알려 준다.
- HTML5가 들어오면서 `<!DOCTYPE html>`로 통일 되었기에 그냥 넣어주면 된다.

> html 과 DOCTYPE 은 HTML 문서를 작성 한다면 관용적으로 추가해야 한다 

```html
<!doctype html>    이 웹페이지가 HTML로 만들어졌다는 것을 표시.
<html>
<head>
    ...
</head>
<body>
    ...
</body>
</html>
```

# a

anchor

링크 설정. href(HyperText Reference) attribute 에 링크 주소 설정.

```html
<a href="https://www.w3.org/TR/html5/" target="_blank">go to HTML5</a>
```

# meta

메타 정보. 웹페이저 전체를 설명하는 정보. 이런 정보들은 글의 본문에 포함되지는 않지만 웹 페이지를 검색, 분류하기 윈한 중요한 정보이다

ex) 웹페이지를 검색 키워드, 웹페이지 제목, 웹 페이지 저자, 웹 페이지 내용...

```html
웹 브라우저가 웹을 특정 인코딩으로 읽어 들이게끔 알려주기 위해서 meta 태그의 charset이라는 attribute를 이용해서 지정
<meta charset="utf-8">

검색 엔진에서 name을 보고 검색 할 확률이 높다.
<meta name="description" content="생활코딩의 소개 자료" >
<meta name="keywords" content="HTML, CSS, Coding, programming" >
<meta name="author" content="egoing" >

30초 간격으로 리프레시.
<meta http-equiv="refresh" content="30">
```

# 스타일을 위한 태그 div, span

div, span 태그는 다른 태그들과 달리 정보로서 의미를 가지지 않는다. 단지 스타일을 적용하기 위한 용도로 사용된다.

div
- 무색무취와 같은 태그, 어떤 의미도 없음.
- block level tag.

```html
<body>
  <div>NAVIGATION</div>
  <div>ARTICLE</div>
</body>
```

span
- div 와 비슷하며 기본적으로 inline tag.


ex) div, span 태그를 이용하여 여러 태그를 그룹핑하여 스타일 적용

```html
<style>
    div{
        border:5px solid gray;
    }
</style>
<body>
    <div>                  <-- div라는 의미없는 태그로 2개의 태그를 묶음.
        <div>NAVIGATION</div>  
        <div>ARTICLE</div>
    </div>
</body>
```

# link

파일에서 코드를 다운로드 받아 붙여넣음.

```html
style.css 파일에 있는 코드를 다운로드 받고 붙여 넣기

<link rel="stylesheet" href="style.css">
```

# TIP

태그 끼리는 부모 자식 관계를 가지며 필요에 따라서는 자식, 부모 태그 관계과 역전될 수 있다.

```html
<parent>
  <child></child>
</parent>
<p>
  <a href="https://www.w3.org/TR/html5/" target="_blank">go to HTML5</a>
</p>
```

클로즈가 없는 태그
- img, input, br, hr, meta
- HTML의 여러 태그 중에 무엇인가를 설명하지 않는 태그들은 감싸야하는 컨텐츠가 없기 때문에 태그를 닫지 않는다는 규칙이 있다.

web page에서 tag 사용 빈도수를 확인 할 수 있는 사이트
- https://www.advancedwebranking.com/html/

HTML5 부터 추가된 semantic
```
article	: 본문
aside	: 광고와 같이 페이지의 내용과는 관계가 적은 내용들
details	: 기본적으로 표시되지 화면에 표시되지 않는 정보들을 정의
figure	: 삽화나 다이어그램과 같은 부가적인 요소를 정의
footer	: 화면의 하단에 위치하는 사이트나 문서의 전체적인 정보를 정의
header	: 화면의 상단에 위치하는 사이트나 문서의 전체적인 정보를 정의
main	: 문서에서 가장 중심이 되는 컨텐츠를 정의
mark	: 참조나 하이라이트 표시를 필요로 하는 문자를 정의
nav     : 문서의 네비게이션 항목을 정의
section	: 문서의 구획들을 정의 (참고)
```

**검색 엔진 최적화**

사용자가 검색 했을때 상위에 노출할 수 있도록 하기 위해서는 HTML 코드를 의미론적으로 잘 구현해야 한다.
- **HTML 개발 시 UI 모습은 신경쓰지 말고 의미론적(semantic)으로 잘 구조화 될 수 있도록 개발해야 한다.** UI 모습은 CSS 로 처리해야 한다.

ex)
- title 태그를 이용하여 페이지의 제목 나타내기
-  description 메타 태그 사용하기
- URL 주소를 알아보기 쉽도록 바꾸기,
```
www.wwf.com/abxd23as <- 안좋은 예
www.wwf.com/animal <- 좋은 예
```