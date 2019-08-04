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
            body {
                font-family: "Segoe UI", arial, sans-serif;
                color: #232323;
                margin: 0;
            }

            header {
                display: grid;
                grid-auto-flow: column;
                align-items: center;
                height: 64px;
            }

            header h1 {
                margin: 0;
                color: #7835a9;
            }

            header h1 span {
                color: cornflowerblue;
            }

            a {
                text-decoration: none;
            }

            .container {
                display: grid;
                justify-content: stretch;
                max-width: 700px;
                margin: auto;
            }

            .content {
                margin: 16px 0;
                overflow: hidden;
            }

            .content > *:first-child {
                margin-top: 0;
            }

            pre {
                overflow: auto;
            }

            @media only screen and (max-width: 750px) {
                .container {
                    margin: 0 16px;
                }
            }
        </style>
    </head>
    <body>
        <div class="container">
            <header>
                <a href="/">
                    <h1>mah<span>blog</span></h1>
                </a>
            </header>
            <div class="content">
                {{~> content}}
            </div>
        </div>
    </body>
</html>