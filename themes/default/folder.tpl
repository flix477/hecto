{{#*inline "content"}}
    <style>
        .posts {
            display: grid;
            grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr;
            grid-auto-rows: 100px;
            grid-gap: 24px 16px;
        }

        .posts a:first-child {
            grid-column: 1 / -1;
            grid-row: 1 / 4;
            font-size: 48px;
        }

        .posts a:nth-child(2), .posts a:nth-child(3) {
            grid-column: auto / span 3;
            grid-row: auto / span 2;
            font-size: 36px;
        }

        .posts a {
            grid-column: auto / span 2;
        }

        .post {
            font-family: 'Playfair Display', serif;
            background: rgba(0, 0, 0, 0.05);
            height: 100%;
        }

        .post span {
            padding: 16px;
            font-weight: 800;
            background: rgb(255, 230, 0);
            display: inline-block;
            position: relative;
            top: -16px;
            left: -8px;
        }
    </style>

    {{#if posts}}
        <section class="posts">
            {{#each posts as |post|}}
                <a href={{post.link}}>
                    <div class="post" style="{{#if post.image}}background: url({{post.image}}){{/if}}">
                        <span>{{post.title}}</span>
                    </div>
                </a>
            {{/each}}
        </section>
    {{else}}
        <h1>There does not seem to be anything here.</h1>
    {{/if}}
    {{#if folders}}
        <section>
            <h1>Folders in {{title}}</h1>
            <div>
                {{#each folders as |folder|}}
                    <a href={{folder.link}}><h2>{{folder.title}}</h2></a>
                {{/each}}
            </div>
        </section>
    {{/if}}
{{/inline}}
{{~> page this}}