// EditorJS script is already loaded
var article_editor;

window.addEventListener("load", () => {
    article_editor = new ArticleEditor;
    article_editor.init();
})

class ArticleEditor {
    constructor() {
        this.editor = null;
        this.article_id = null;
        this.first_edit_flag = false;

        //Article states
        this.is_published = false;
        this.status = "draft";
        this.published_date = moment().fromNow();
        this.update_date = moment().fromNow();

        // Get editor elements
        let title_input_timeout = null;
        this.title_input = document.querySelector(".article-title");

        // Article management
        this.publish_button = document.querySelector("#publish-article");
        this.unpublish_button = document.querySelector("#unpublish-article");
        this.discard_changes_button = document.querySelector("#discard-article-changes");
        this.delete_article_button = document.querySelector("#delete-article");

        // Article status
        this.article_status = document.querySelector("#article-status");
        this.article_status_last_update = document.querySelector("#article-status-last-update");

        // Editor status
        this.editor_status = document.querySelector("#editor-status");
        this.editor_status_last_update = document.querySelector("#editor-status-last-update");

        setInterval(() => {
            this.update_article_status();
            this.update_editor_status();
        }, 1000);
    }

    init () {
        var path_segments = location.pathname.split("/");
        var article_id = path_segments[path_segments.length - 1];
        this.article_id = article_id;

        this.load_article (this.article_id).then((response) => {
            var article = response.article;

            // Set article title
            this.title_input.value = article.title;
            this.status = response.status;
            this.is_published = response.published;
            this.published_date = response.published_date;
            this.update_date = article.update_date;

            // Set the article status
            this.update_article_status();
            this.update_editor_status();
    
            // Load the editor
            this.editor = new EditorJS({
                holder: "rpusblish-editor",
                placeholder: 'Let`s write something awesome!',
                autofocus: true,
                minHeight : 0,
                tools: {
                    header: Header,
                    list: {
                        class: List,
                        inlineToolbar: true,
                    },
                    nested_list: {
                        class: NestedList,
                        inlineToolbar: true,
                    },
                    checklist: {
                        class: Checklist,
                        inlineToolbar: true,
                    },                 
                    image: {
                        class: ImageTool,
                        config: {
                            endpoints: {
                                byFile: `${location.protocol}://${location.host}/dashboard/endpoint/uploadFile`, // Your backend file uploader endpoint
                            },
                            uploader: {
                                uploadByUrl(url){
                                    return new Promise((resolve) => {
                                        console.log(url)
                                        resolve({
                                            success: 1,
                                            file: {
                                                url: url
                                            }
                                        })
                                    })
                                }
                            }
                        }
                    },
                    attaches: {
                        class: AttachesTool,
                        config: {
                            endpoint: `${location.protocol}://${location.host}/dashboard/endpoint/fileUpload`
                        }
                    },          
                    code: CodeTool,
                    quote: Quote,
                    marker: Marker,
                    inlineCode: InlineCode,
                    raw: RawTool,
                    warning: Warning,
                    table: Table,
                    paragraph: {
                        class: Paragraph,
                        inlineToolbar: true,
                    },
                    footnotes: FootnotesTune,
                    alert: Alert,
                    paragraph: {
                        class: Paragraph,
                        inlineToolbar: true,
                    }          
                },
                data: article.data == ""? {} : JSON.parse(article.data),
                onChange: () => {
                    if (this.first_edit_flag) {
                        this.save_article();
                    } else {
                        this.first_edit_flag = true;
                    }
                },
                onReady: () => {
                    setTimeout(() => {
                        this.first_edit_flag = true;
                    }, 1000);
                }          
            });
    
            // Setup events
            this.title_input.addEventListener("input", () => {
                clearTimeout(this.title_input_timeout);
                this.title_input_timeout = setTimeout(() => {
                    this.save_article();
                }, 500);
            })
    
            this.publish_button.addEventListener("click", () => {
                // Publish the article
                this.publish_article(this.article_id).then(() => {
                    this.published_date = moment();
                    this.update_date = moment();
                    this.is_published = true;
                    this.status = "published";
                    this.update_article_status();
                    this.update_editor_status();
                }).catch(() => {})
            })

            this.unpublish_button.addEventListener("click", () => {
                // Unpublish the article
                this.unpublish_article(this.article_id).then(() => {
                    this.published_date = moment();
                    this.is_published = false;
                    this.status = "draft";
                    this.update_article_status();
                    this.update_editor_status();
                }).catch(() => {})
            })

            this.discard_changes_button.addEventListener("click", () => {
                //
            })

            this.delete_article_button.addEventListener("click", () => {
                // Delete the article
                this.delete_article(this.article_id).then(() => {
                    window.location = `${location.protocol}//${location.host}/dashboard/articles`
                }).catch(() => {})
            })
        }).catch(() => {
            console.log("Not found")
        })
    }

    save_article () {
        this.editor.save().then((outputData) => {
            this.save_draft(this.article_id, this.title_input.value, outputData).then(() => {
                this.update_date = moment();
                this.status = "edited";
                this.update_editor_status();
            }).catch(() => {})
        }).catch((error) => {
            alert("Saving failed");
            console.log('Saving failed: ', error)
        });
    }

    update_article_status () {
        switch (this.is_published) {
            case "published":
            case true:
                this.article_status.innerText = "Published";
                this.article_status.setAttribute("status", "ok");
                this.article_status_last_update.innerText = "Last published " + moment(this.published_date).fromNow();
                this.unpublish_button.removeAttribute("hide");
                break;
            case "unpublished":
            case false:
                this.article_status.innerText = "Not Published";
                this.article_status.setAttribute("status", "alert");
                this.article_status_last_update.innerText = "Never published";
                this.unpublish_button.setAttribute("hide", "");
                break;
        }
    }

    update_editor_status () {
        switch (this.status.toLowerCase()) {
            case "published":
                this.editor_status.innerText = "Not Modified";
                this.editor_status.setAttribute("status", "ok");
                this.discard_changes_button.setAttribute("hide", "");
                break;
            case "draft":
            case "edited":
                this.editor_status.innerText = "Modified";
                this.editor_status.setAttribute("status", "info");
                if (this.is_published) {
                    this.discard_changes_button.removeAttribute("hide");
                } else {
                    this.discard_changes_button.setAttribute("hide", "");
                }
                break;
        }
        this.editor_status_last_update.innerText = "Last saved " + moment(this.update_date).fromNow();
    }

    load_article (article_id) {
        return new Promise((resolve, reject) => {
            fetch(`${location.protocol}//${location.host}/dashboard/api/article/${article_id}`, {
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

    save_draft (article_id, title, data) {

        var draft = {
            title,
            data: JSON.stringify(data)
        };
    
        return new Promise((resolve, reject) => {
            fetch(`${location.protocol}//${location.host}/dashboard/api/article/${article_id}`, {
                method: 'PUT',
                mode: 'cors',
                cache: 'no-cache',
                credentials: 'same-origin',
                headers: {
                'Content-Type': 'application/json'
                },
                redirect: 'follow',
                referrerPolicy: 'no-referrer',
                body: JSON.stringify(draft)
            }).then((response) => {
                if (response.status == 200) {
                    console.log("Draft saved");
                    resolve();
                } else {
                    console.log("Draft not saved saved: ", response.text());
                    reject();
                }
            }).catch((err) => {
                console.log("Draft not saved saved: Network error", err);
                reject();
            })
        })
    }

    publish_article (article_id) {
        return this.article_action (article_id, "POST", "publish");
    }

    unpublish_article (article_id) {
        return this.article_action (article_id, "POST", "unpublish");
    }

    discard_article_changes (article_id) {
        return this.article_action (article_id, "POST", "discard");
    }

    delete_article (article_id) {
        return this.article_action (article_id, "POST", "delete");
    }

    article_action (article_id, method, action) {
        return new Promise((resolve, reject) => {
            fetch(`${location.protocol}//${location.host}/dashboard/api/article/${article_id}/${action}`, {
                method: method,
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
                    resolve("Published");
                } else {
                    reject(`Error on ${action} the article`);
                }
            }).catch((err) => {
                reject(`Error ${action} the article`);
            })
        })
    }
}