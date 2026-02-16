웹페이지 1억개가 있는데 이 모든 웹페이지의 CSS 디자인을 바꿔야 할 때 모든 웹페이지를 수정 해야 할까 ? 

=> 해답 : css 코드만 따로 빼서 .css 파일에 저장해서 사용.

# .css 파일

css 를 정의한 파일. html 파일은 link 태그를 사용하여 css 파일을 다운받고 스타일을 적용할 수 있다.

.css 파일을 사용하면 스타일을 여러 html 파일이 공유할 수 있다.

ex) style.css
```css
body{
    margin:0;
}
a {
    color:black;
}
h1 {
    font-size:45px;

    ...
```

ex) index.html
```html
<!doctype html>
<html>
<head>
  <title>WEB - CSS</title>
  <meta charset="utf-8">
  <link rel="stylesheet" href="style.css">   <-- css 파일과 링크
</head>
<body>
  <h1><a href="index.html">WEB</a></h1>
  <div id="grid">
    <ol>
```

모든 웹 페이지마다 index.html 처럼 `link` 로 걸어주면 style.css 파일이 변경 될때마다 일괄적으로 UI가 적용 될 것이다

# .css 파일 캐시

css 를 html 에 내장하지 않고 별도의 .css 파일로 분리하여 적용하면 브라우저는 .css 파일을 다운로드 받은 후 캐싱하여 사용할 수 있다. 매번 css 코드를 받는 내장 방식보다 성능 면에서 유리하다.

단 캐시 처리의 영향으로 css 파일을 변경하고 새로고침 하여도 디자인 업데이트가 안되는 경우가 있다. 이때는 캐싱된 .css 파일 제거 후 새로고침 해야 한다.