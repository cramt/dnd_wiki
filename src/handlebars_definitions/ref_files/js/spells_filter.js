(() => {
    const head = document.getElementsByTagName("head")[0];
    const style = head.appendChild(document.createElement("style"));

    const editStyle = (classFilter, schoolFilter) => {
        if (classFilter === "all" && schoolFilter === "all") {
            style.innerHTML = `
                .spell {
                    display: list-item;
                }
            `
        }
        else if (classFilter === "all") {
            style.innerHTML = `
                .spell {
                    display: none;
                }
                .spell.school-${schoolFilter} {
                    display: list-item;
                }
            `
        }
        else if (schoolFilter === "all") {
            style.innerHTML = `
                .spell {
                    display: none;
                }
                .spell.class-${classFilter} {
                    display: list-item
                }
            `
        }
        else {
            style.innerHTML = `
                .spell {
                    display: none;
                }
                .spell.class-${classFilter}.school-${schoolFilter} {
                    display: list-item
                }
            `
        }
    }

    editStyle("all", "all")

    document.getElementById("filter").addEventListener("change", (event) => {
        let form = event.target.closest("form")
        let data = new FormData(form)
        let classFilter = data.get("class-filter")
        let schoolFilter = data.get("school-filter")
        editStyle(classFilter, schoolFilter)
    })
})()