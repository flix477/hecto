{{#*inline "content"}}
    <style>
        .posts {
            margin-top: 16px;
            display: grid;
            grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr;
            grid-auto-rows: 100px;
            grid-gap: 32px 16px;
        }

        .posts a:first-child {
            grid-column: 1 / -1;
            grid-row: 1 / 4;
            font-size: 36px;
        }

        .posts a:nth-child(2), .posts a:nth-child(3) {
            grid-column: auto / span 3;
            grid-row: auto / span 2;
            font-size: 32px;
        }

        .posts a {
            grid-column: auto / span 2;
        }

        .post {
            border: 1px solid rgba(0,0,0,0.6);
            height: 100%;
        }

        .post h3 {
            font-family: 'Playfair Display', serif;
            padding: 24px;
            line-height: 2.5rem;
            margin: 0;
            font-weight: 800;
            background: rgb(255, 230, 0);
            display: inline-block;
            position: relative;
            top: -16px;
            left: -8px;
        }

        .post p {
            margin: 0;
            padding: 16px;
            padding-top: 0;
            font-weight: 400;
            font-size: 16px;
        }
    </style>

    {{#if data.posts}}
        <section class="posts">
            {{#each data.posts as |post|}}
                <a href={{post.link}}>
                    <div class="post" style="{{#if post.image}}background: url({{post.image}}){{/if}}">
                        <h3>{{post.title}}</h3>
                        <p>{{post.preview}}</p>
                    </div>
                </a>
            {{/each}}
        </section>
    {{else}}
        <h1>There does not seem to be anything here.</h1>
    {{/if}}
    {{#if data.folders}}
        <section>
            <div>
                {{#each data.folders as |folder|}}
                    <a href={{folder.link}}><h2>{{folder.title}}</h2></a>
                {{/each}}
            </div>
        </section>
    {{/if}}
{{/inline}}
{{~> page this}}