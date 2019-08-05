{{#*inline "content"}}
    <style>
        h1, h2, h3, h4, h5 {
            font-family: 'Playfair Display', serif;
        }

        h2 {
            opacity: 0.9;
        }

        h3 {
            font-weight: 500;
        }

        .content {
            text-align: center;
        }

        .content * {
            text-align: left;
        }

        .content h1:nth-child(3) {
            margin-top: 0;
            font-size: 42px;
            font-weight: 900;
            text-align: center;
            margin-bottom: 48px;
            line-height: 1.3;
            padding: 16px;
            display: inline-block;
            background: rgb(255, 230, 0);
        }

        pre {
            overflow: auto;
        }

        .date {
            font-family: 'Playfair Display', serif;
            margin-top: 16px;
            margin-bottom: 4px;
            opacity: 0.5;
            font-weight: 400;
            font-style: italic;
            font-size: 14px;
            text-align: center;
        }

        @media only screen and (max-width: 750px) {
            .date {
                margin-top: 8px;
            }
            
            .content h1:nth-child(3) {
                font-size: 32px;
                margin-bottom: 28px;
            }
        }
    </style>
    <p class="date">{{creation_date}}</p>
    {{{contents}}}
{{/inline}}
{{~> page this}}
