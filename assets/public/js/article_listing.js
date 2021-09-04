window.addEventListener("load", () => {
    var draft_articles_explorer = new ArticlesExplorer("draft", "#draft-articles-explorer");
    var published_articles_explorer = new ArticlesExplorer("published", "#published-articles-explorer");
})

class ArticlesExplorer {
    constructor (type, container_selector) {
        this.type = type;

        this.paginator = {
            page: 0,
            count: 30,
            total: 0,
            max_buttons: 9
        }

        // Articles Explorer root element
        this.container = document.querySelector(container_selector);

        // Paginator search box input
        this.search_box = this.container.querySelector(".article-search-box");
        
        // Paginator search results box
        this.results_box = this.container.querySelector(".article-search-results");
        
        // Paginator info elements
        this.paginator_box = this.container.querySelector(".article-search-paginator");
        this.paginator_current_articles_count = this.container.querySelector(".current-articles-count");
        this.paginator_total_articles_count = this.container.querySelector(".total-articles-count");
        this.paginator_current_page = this.container.querySelector(".current-articles-page");
        this.paginator_total_pages = this.container.querySelector(".total-articles-pages");

        // Paginator buttons
        this.paginator_prev_button = this.container.querySelector(".paginator-prev-button");
        this.paginator_next_button = this.container.querySelector(".paginator-next-button");

        // Paginator page buttons container
        this.paginator_pages_box = this.container.querySelector(".paginator-list");

        // Setup events
        this.paginator_next_button.addEventListener("click", () => {
            this.next_page();
        })

        this.paginator_prev_button.addEventListener("click", () => {
            this.prev_page();
        })

        // Load the first page
        this.load_page ();
    }

    next_page () {
        if (this.paginator.page + 1 < this.paginator.total) {
            this.paginator.page += 1;
            this.load_page();
        }
    }
    prev_page () {
        if (this.paginator.page - 1 >= 0) {
            this.paginator.page -= 1;
            this.load_page();
        }
    }

    load_page () {
        this.get_article_list(this.type, this.paginator.count * this.paginator.page, this.paginator.count).then((response) => {
            // Calc the total pages
            var total_pages = Math.ceil(response.total / this.paginator.count);

            // Store the total number of pages
            this.paginator.total = total_pages;

            // Fill info in to show the user
            this.paginator_current_articles_count.innerText = Object.keys(response.articles).length;
            this.paginator_total_articles_count.innerText = response.total;
            this.paginator_current_page.innerText = this.paginator.page + 1;
            this.paginator_total_pages.innerText = total_pages;

            // Clear the pages box the dirty and easy way
            this.paginator_pages_box.innerHTML = "";
            
            // Calc the number of buttons to add to the paginator
            var page_range = this.page_range(this.paginator.page, total_pages, this.paginator.max_buttons);
            if(page_range.start == 0 && page_range.end == 0) {
                page_range.end = 1;
            }
            for (let i = page_range.start; i < page_range.end; i++) {
                var page_selector = document.createElement("button");
                page_selector.innerText = i + 1;
                if(i == this.paginator.page) {
                    page_selector.setAttribute("current", "");
                }
                this.paginator_pages_box.appendChild(page_selector);
            }

            // Render articles list
            this.render_articles(response.articles);
        }).catch((e) => {
            console.log(e);
        })
    }

    render_articles (articles) {
        // Clear the results box the dirty and easy way
        this.results_box.innerHTML = "";
        for (const article_id in articles) {
            const article = articles[article_id];
            // Create article element
            var article_link = document.createElement("a");
            var article_box = document.createElement("div");
            var article_title = document.createElement("div");

            article_link.href = `${location.protocol}//${location.host}/dashboard/article/edit/${article_id}`
            article_title.innerText = article.title;

            article_link.classList.add("article-link");
            article_box.classList.add("article-link-box");

            article_box.appendChild(article_title);
            article_link.appendChild(article_box);
            // Append the article
            this.results_box.appendChild(article_link);
        }
        
        if (Object.keys(articles).length == 0)
        {
            this.results_box.innerHTML = `<div class="no-articles-found">No articles found</div>`;
        }
    }

    page_range(page, total_pages, items) {

        var start = page - Math.floor(items / 2);
        var end = page + Math.ceil(items / 2);
    
        if (end > total_pages) {
            start -= (end - total_pages);
            end = total_pages;
        }
        
        if (start <= 0) {
            end += ((start) * -1);
            start = 0;
        }
    
        end = end > total_pages ? total_pages : end;
    
        return {
            start: start,
            end: end
        };
    }

    get_article_list (type, start, count) {
        return new Promise((resolve, reject) => {
            fetch(`${location.protocol}//${location.host}/dashboard/api/articles/${type}/${start}/${count}`, {
                method: 'GET',
                mode: 'cors',
                cache: 'no-cache',
                credentials: 'same-origin',
                headers: {
                    'Content-Type': 'application/json'
                },
                redirect: 'follow',
                referrerPolicy: 'no-referrer'
            }).then((response) => {
                if (response.status == 200) {
                    var data = response.json();
                    resolve(data);
                } else {
                    reject();
                }
            }).catch((err) => {
                console.log(err)
                reject(err)
            })
        })    
    }
}