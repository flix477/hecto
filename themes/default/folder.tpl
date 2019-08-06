{{#*inline "content"}}
    <style>
        .posts {
            margin-top: 16px;
        }

        .post {
            height: 100%;
            background: white;
            margin-bottom: 64px;
        }

        .post img {
            display: block;
            width: 100%;
            height: 300px;
            object-fit: cover;
        }

        .post .post-content {
            position: relative;
            border: 1px solid rgba(0,0,0,0.6);
            text-align: center;
        }

        .post-title-container {
            transform: translateY(-24px);
        }

        .post-title-container h3 {
            display: inline-block;
            font-family: 'Playfair Display', serif;
            padding: 16px;
            margin: 0;
            font-size: 32px;
            font-weight: 800;
            background: rgb(255, 230, 0);
        }

        .post-title-container .post-info {
            display: flex;
            justify-content: center;
            margin-top: 8px;
            font-size: 14px;
            font-style: italic;
            opacity: 0.35;
        }
        
        .post-info p {
            margin: 0 16px;
        }

        .post .post-preview {
            text-align: left;
            margin: 0;
            padding: 32px 42px;
            padding-top: 0;
            font-size: 20px;
        }
    </style>

    {{#if posts}}
        <section class="posts">
            {{#each posts as |post|}}
                <div class="post">
                    {{#if post.image}}
                        <img src={{post.image}} />
                    {{/if}}
                    <div class="post-content">
                        <div class="post-title-container">
                            <a href={{post.link}}>
                                <h3>{{post.title}}</h3>
                            </a>
                            <div class="post-info">
                                <p>{{post.creation_date}}</p>
                                <p>{{post.reading_time}} minutes read</p>
                            </div>
                        </div>
                        <p class="post-preview">{{post.preview}}</p>
                    </div>
                </div>
            {{/each}}
        </section>
    {{else}}
        <h1>There does not seem to be anything here.</h1>
    {{/if}}
    {{#if folders}}
        <section>
            <div>
                {{#each folders as |folder|}}
                    <a href={{folder.link}}><h2>{{folder.title}}</h2></a>
                {{/each}}
            </div>
        </section>
    {{/if}}
{{/inline}}
{{~> page this}}