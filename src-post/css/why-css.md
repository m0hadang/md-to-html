HTML은 문서 구조(이름, 단원, 내용, 목차 등등)를 표현하기 적합하며 검색 엔진이 찾기 편리한 구조이다. 데이터가 잘 가공되어 있어서 정보로서 가치가 있다. 그러나 HTML에 디자인 관련 코드가 섞이면서 검색 엔진이 검색하기 불편하고 정보로서 가치가 떨어진다.

```html
HTML은 디자인 적합한 언어가 아니다
만약 색깔을 blue로 바꿔야 할 때 모든 font 태그를 찾아서 일일이 수정 해 주어야 함

<h1><a href="index.html"><font color="red">WEB</font></a></h1>
    <ol>
        <li><a href="1.html"><font color="red">HTML</font></a></li>
        <li><a href="2.html"><font color="red">CSS</font></a></li>
        <li><a href="3.html"><font color="red">JavaScript</font></a></li>
    </ol>
<h2>CSS</h2>
```

기존에 HTML에서 디자인 관련 태그(font...)를 추가해서 사용 하려고 하였지만 한계에 부딛힌다. 

CSS는 HTML의 디자인적 한계를 극복할 수 있는 새로운 언어이다.