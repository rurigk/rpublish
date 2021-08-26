// EditorJS script is already loaded

// Abort signals
const controller = new AbortController();
const { signal } = controller;

window.addEventListener("load", () => {
    var path_segments = location.pathname.split("/");
    var article_id = path_segments[path_segments.length - 1];
    load_article (article_id).then((article) => {
        // Get elements
        var title_input = document.querySelector(".article-title");
        var publish_button = document.querySelector(".publish-button");

        // Set article title
        title_input.value = article.title;

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
                editor.save().then((outputData) => {
                    save_draft(article_id, title_input.value, outputData);
                }).catch((error) => {
                    alert("Saving failed");
                    console.log('Saving failed: ', error)
                });
            }
        });

        // Setup events
        title_input.addEventListener("input", () => {
            editor.save().then((outputData) => {
                save_draft(article_id, title_input.value, outputData);
            }).catch((error) => {
                alert("Saving failed");
                console.log('Saving failed: ', error)
            });
        })

        publish_button.addEventListener("click", () => {
            // Publish the article
        })
    }).catch(() => {
        console.log("Not found")
    })
})

function load_article (article_id) {
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

function save_draft(article_id, title, data) {
    //controller.abort();

    var draft = {
        title,
        data: JSON.stringify(data)
    };

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
        body: JSON.stringify(draft),
        signal
    }).then((response) => {
        if (response.status == 200) {
            console.log("Draft saved");
        } else {
            console.log("Draft not saved saved: ", response.text());
        }
    }).catch((err) => {
        console.log("Draft not saved saved: Network error", err);
    })
}