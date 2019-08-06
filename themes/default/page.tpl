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
            @import url('https://fonts.googleapis.com/css?family=PT+Serif:400|Playfair+Display:400,500,900&display=swap');

            body {
                font-family: 'PT Serif', serif;
                color: #232323;
                margin: 0;
                line-height: 1.5;
                overflow-x: hidden;
                font-size: 18px;
                background: rgb(250, 250, 250);
            }

            header {
                text-align: center;
            }

            header a {
                display: inline-flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                margin-top: 56px;
                margin-bottom: 36px;
            }

            header img {
                width: 128px;
                height: 128px;
                margin-bottom: 8px;
                background: rgba(0, 0, 0, 0.1);
                border-radius: 50%;
            }

            header h1 {
                display: inline;
                margin: 0;
                font-weight: 900;
                font-family: 'Playfair Display', sans-serif;
                font-style: italic;
                color: black;
                font-size: 32px;
            }

            header p {
                margin: 0;
                opacity: 0.5;
                font-size: 16px;
            }

            a {
                text-decoration: none;
                color: inherit;
            }

            .container {
                display: grid;
                justify-content: stretch;
                max-width: 800px;
                margin: auto;
            }

            @media only screen and (max-width: 850px) {
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
                    <img src="https://avatars0.githubusercontent.com/u/6579510?s=460&v=4" />
                    <h1>{{site_name}}</h1>
                    <p>Software Engineer</p>
                </a>
            </header>
            <div class="content">
                {{> content}}
            </div>
        </div>
    </body>
</html>