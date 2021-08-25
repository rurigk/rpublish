// EditorJS script is already loaded

window.addEventListener("load", () => {
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
                        byFile: `${location.protocol}://${location.hostname}/dashboard/endpoint/uploadFile`, // Your backend file uploader endpoint
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
                    endpoint: `${location.protocol}://${location.hostname}/dashboard/endpoint/fileUpload`
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
    });
})