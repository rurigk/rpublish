// EditorJS script is already loaded
var article_editor;

window.addEventListener("load", () => {
    article_editor = new ArticleEditor;
    article_editor.init();
})

class ArticleEditor {
    constructor() {
        this.first_edit_flag = false;
        // Get editor elements
        this.title_input = document.querySelector(".article-title");

        // Article management
        this.publish_button = document.querySelector("#publish-article");

        // Article status
        this.article_status = document.querySelector("#article-status");
        this.article_status_last_update = document.querySelector("#article-status-last-update");

        // Editor status
        this.editor_status = document.querySelector("#editor-status");
        this.editor_status_last_update = document.querySelector("#editor-status-last-update");
    }

    init () {
        var path_segments = location.pathname.split("/");
        var article_id = path_segments[path_segments.length - 1];

        this.load_article (article_id).then((response) => {
            var article = response.article;

            // Set article title
            this.title_input.value = article.title;

            // Set the article status
            this.update_article_status (response.published ? "published" : "unpublished", moment(response.published_date).fromNow());
            this.update_editor_status (response.status, moment(article.update_date).fromNow());
    
            // Load the editor
            const editor = new EditorJS({
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
                        editor.save().then((outputData) => {
                            this.save_draft(article_id, this.title_input.value, outputData).then(() => {
                                this.update_editor_status ("edited", moment().fromNow());
                            }).catch(() => {})
                        }).catch((error) => {
                            alert("Saving failed");
                            console.log('Saving failed: ', error)
                        });
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
                editor.save().then((outputData) => {
                    this.save_draft(article_id, this.title_input.value, outputData).then(() => {
                        this.update_editor_status ("edited", moment().fromNow());
                    }).catch(() => {})
                }).catch((error) => {
                    alert("Saving failed");
                    console.log('Saving failed: ', error)
                });
            })
    
            this.publish_button.addEventListener("click", () => {
                // Publish the article
                this.publish_article(article_id).then(() => {
                    this.update_article_status("published", moment().fromNow());
                    this.update_editor_status ("published", moment().fromNow());
                }).catch(() => {})
            })
        }).catch(() => {
            console.log("Not found")
        })
    }

    update_article_status (status, time_str) {
        switch (status) {
            case "published":
                this.article_status.innerText = "Published";
                this.article_status.setAttribute("status", "ok");
                this.article_status_last_update.innerText = "Last published " + time_str;
                break;
            case "unpublished":
                this.article_status.innerText = "Not Published";
                this.article_status.setAttribute("status", "alert");
                this.article_status_last_update.innerText = "Never published";
                break;
        }
    }

    update_editor_status (status, time_str) {
        switch (status.toLowerCase()) {
            case "published":
                this.editor_status.innerText = "Not Modified";
                this.editor_status.setAttribute("status", "ok");
                break;
            case "draft":
            case "edited":
                this.editor_status.innerText = "Modified";
                this.editor_status.setAttribute("status", "info");
                break;
        }
        this.editor_status_last_update.innerText = "Last saved " + time_str;
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
        return new Promise((resolve, reject) => {
            fetch(`${location.protocol}//${location.host}/dashboard/api/article/${article_id}/publish`, {
                method: 'POST',
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
                    reject("Error publishing the article");
                }
            }).catch((err) => {
                reject("Error publishing the article");
            })
        })
    }
}