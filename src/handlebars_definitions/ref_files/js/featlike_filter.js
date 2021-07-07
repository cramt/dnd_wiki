(() => {
    const flip = (element) => {
        if (element.style.display === "none") {
            element.style.display = ""
        } else {
            element.style.display = "none"
        }
    }

    Array.from(document.getElementsByClassName("feat-title")).forEach(element => {
        element.onclick = (event) => {
            flip(element.parentElement.getElementsByClassName("feat-contents")[0])
        }
    })

    document.getElementById("open-all").onclick = (e) => {
        Array.from(document.getElementsByClassName("feat-contents")).forEach(x => x.style.display = "")
    }

    document.getElementById("close-all").onclick = (e) => {
        Array.from(document.getElementsByClassName("feat-contents")).forEach(x => x.style.display = "none")
    }
})()