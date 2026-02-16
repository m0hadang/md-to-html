# 반응형 웹 디자인

웹 페이지는 다양한 형태의 시스템(PC, Mobile, Tablet...) 환경에 맞게 최적화된 화면을 제공해야 한다. 반응형 웹 디자인은 이러한 다양한 환경에 맞게 웹 페이지를 최적화하는 기술이다.

 반응형 웹 디자인이 적용되지 않을 경우 PC 에서는 정상적으로 출력되지만 Mobile 에서는 글자가 짤리는 듯한 현상이 발생할 수 있다.

반응형 웹 디자인의 핵심은 변환하는 화면에 대응하여 적절한 웹 페이지 디자인을 제공하는 것이다.

# 미디어 쿼리

미디어 쿼리를 이용하면 화면의 크기에 따라 다른 스타일을 적용할 수 있다.

ex) 화면 크기가 800px 미만일 때 div 태그를 숨긴다.

```html
<head>
    <meta charset="utf-8">
    <title></title>
    <style>
        div{
            border:10px solid green;
            font-size:60px
        }
        @media(min-width:800px){      <-- 미디어 쿼리
            div{
                display:none;
            }
        }
    </style>
</head>
<body>
    <div>
        Responsive
    </div>
</body>
```