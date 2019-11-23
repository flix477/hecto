{{#*inline "content"}}
    <style>
        blockquote {
            background: rgba(0, 0, 0, 0.05);
            margin: 0;
            padding: 1.25rem;
        }

        h2 {
            opacity: 0.9;
        }

        h3 {
            font-weight: 500;
        }

        .content {
            text-align: center;
            background: rgba(0, 0, 0, 0.4);
            margin-bottom: 82px;
            margin-top: 100px;
        }

        .content * {
            text-align: left;
        }

        .content > * {
            margin-left: 48px;
            margin-right: 48px;
        }

        .content h1:nth-child(2) {
            margin-top: 0;
            margin-bottom: 0;
            font-size: 28px;
            text-align: center;
            padding: 16px;
            display: inline-block;
            color: #ededed;
            background: rgb(50, 50, 50);
            transform: translateY(-24px);
        }

        .content *:nth-child(3) {
            margin-top: 0;
        }

        pre {
            overflow: auto;
        }

        .date {
            margin-top: 16px;
            margin-bottom: 4px;
            opacity: 0.5;
            font-weight: 400;
            font-style: italic;
            font-size: 12px;
            text-align: center;
        }

        p {
            opacity: 0.7;
            font-size: 16px;
            margin-bottom: 48px;
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
    {{{contents}}}
{{/inline}}
{{~> page this}}
