<!DOCTYPE html>
<html>
    <head>
        <meta charset="UTF-8">
        <meta
            name="viewport"
            content="width=device-width, initial-scale=1, shrink-to-fit=no"
        />
        <title>{{data.title}}</title>
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.10.2/dist/katex.min.css" integrity="sha384-yFRtMMDnQtDRO8rLpMIKrtPCD5jdktao2TV19YiZYWMDkUR5GQZR/NOVTdquEx1j" crossorigin="anonymous">
        <script defer src="https://cdn.jsdelivr.net/npm/katex@0.10.2/dist/katex.min.js" integrity="sha384-9Nhn55MVVN0/4OFx7EE5kpFBPsEMZxKTCnA+4fqDmg12eCTqGi6+BB2LjY8brQxJ" crossorigin="anonymous"></script>
        <script defer src="https://cdn.jsdelivr.net/npm/katex@0.10.2/dist/contrib/auto-render.min.js" integrity="sha384-kWPLUVMOks5AQFrykwIup5lo0m3iMkkHrD0uJ4H5cjeGihAutqP0yW0J6dpFiVkI" crossorigin="anonymous"
                onload="renderMathInElement(document.body);"></script>
        <style>
            @import url('https://fonts.googleapis.com/css?family=PT+Serif|Playfair+Display:400,500,900&display=swap');

            body {
                font-family: 'PT Serif', serif;
                color: #232323;
                margin: 0;
                line-height: 1.8rem;
                overflow-x: hidden;
                font-size: 18px;
            }

            header {
                display: grid;
                grid-auto-flow: column;
                align-items: center;
                height: 64px;
            }

            header h1 {
                display: inline;
                padding: 0 8px;
                margin: 0;
                font-weight: 900;
                font-family: 'Playfair Display', sans-serif;
                font-style: italic;
                color: black;
                font-size: 32px;
            }

            a {
                text-decoration: none;
                color: inherit;
            }

            .container {
                display: grid;
                justify-content: stretch;
                max-width: 700px;
                margin: auto;
            }

            .content {
                padding: 16px 0;
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
                    <h1>{{site_name}}</h1>
                </a>
            </header>
            <div class="content">
                {{> content}}
            </div>
        </div>
    </body>
</html>