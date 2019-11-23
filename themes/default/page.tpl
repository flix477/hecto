<!DOCTYPE html>
<html>
    <head>
        <meta charset="UTF-8">
        <meta
            name="viewport"
            content="width=device-width, initial-scale=1, shrink-to-fit=no"
        />
        <title>{{title}}</title>
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.10.2/dist/katex.min.css" integrity="sha384-yFRtMMDnQtDRO8rLpMIKrtPCD5jdktao2TV19YiZYWMDkUR5GQZR/NOVTdquEx1j" crossorigin="anonymous">
        <script defer src="https://cdn.jsdelivr.net/npm/katex@0.10.2/dist/katex.min.js" integrity="sha384-9Nhn55MVVN0/4OFx7EE5kpFBPsEMZxKTCnA+4fqDmg12eCTqGi6+BB2LjY8brQxJ" crossorigin="anonymous"></script>
        <script defer src="https://cdn.jsdelivr.net/npm/katex@0.10.2/dist/contrib/auto-render.min.js" integrity="sha384-kWPLUVMOks5AQFrykwIup5lo0m3iMkkHrD0uJ4H5cjeGihAutqP0yW0J6dpFiVkI" crossorigin="anonymous"
                onload="renderMathInElement(document.body);"></script>
        <style>
            @import url('https://fonts.googleapis.com/css?family=Alata&display=swap');

            body {
                color: #ffffff;
                background-color: #222222;
                margin: 0;
                line-height: 1.6;
                overflow-x: hidden;
                font-size: 18px;
                font-family: 'Roboto', sans-serif;
            }

            h1, h2, h3, h4, h5, #menu {
                letter-spacing: -1px;
                font-family: 'Alata', sans-serif;
            }

            header {
                position: fixed;
                padding: 16px 0;
            }

            #menu {
                display: flex;
                justify-content: center;
                align-items: center;
                font-size: 32px;
                text-decoration: none;
                color: inherit;
                width: 56px;
                height: 56px;
                border: 1px solid rgba(255, 255, 255, 0.2);
                transition: background-color 0.3s, color 0.3s;
            }

            #menu:hover {
                color: #222222;
                background-color: #ffffff;
            }

            .container {
                display: grid;
                justify-content: stretch;
                max-width: 900px;
                margin: auto;
            }

            .content {
                margin-top: 82px;
                max-width: 750px;
                width: 100%;
                margin-left: auto;
                margin-right: auto;
            }

            a {
                color: inherit;
            }

            @media only screen and (max-width: 932px) {
                .container {
                    margin: 0 16px;
                }
            }
        </style>
    </head>
    <body>
        <div class="container">
            <header>
                <a id="menu" href="/">hc</a>
            </header>
            <div class="content">
                {{> content}}
            </div>
        </div>
    </body>
</html>