# box model 구성

![img.png](box-model-img-1.png)

margin > border > padding > content
- margin: 요소의 바깥 여백
- border: 요소의 테두리
- padding: 요소의 내부 여백
- content: 요소의 콘텐츠

ex) padding, margin  변경

```html
<head>
  <meta charset="utf-8">
  <title></title>
  <style>
    h1{
      border:5px solid red;
      padding:20px;
      margin:20px;
    }
  </style>
</head>
<body>
  <h1>CSS</h1>
  <h1>CSS</h1>
</body>
```

# 태그 display 속성

두가지 유형의 태그가 존재한다
- block level element
  - 화면 전체를 쓰는 태그
  - ex) h1, div,  ... 
  - 
- inline element
  - 자신의 컨텐츠 크기만큼 갖는 태그
  - ex) a

이 유형은 display 속성으로 변경할 수 있다

```css
block level elemnt를 inline level 변경

h1{
    display:inline; 
}
```

display
- block : 높이는 부모에게, 넓이는 자식에게서 받는다
- inline-block : 높이와 너비를 자식에게 받는다.
- grid : 그리드 레이아웃 사용. 
- none : 화면에 표시되지 않는다.