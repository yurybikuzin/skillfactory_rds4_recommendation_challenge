const filters = {
}

function prepareFilterSections() {
    const list_ul = document.querySelectorAll("#filter > main > section > ul[data-data]")
    list_ul.forEach(ul => {
        const list_item = JSON.parse(ul.dataset.data)
        const height_client = document.documentElement.clientHeight
        const height_max = height_client * parseFloat(ul.dataset.heightMax)
        let is_truncated = false
        for (let item of list_item) {
            const el = document.createElement("li")
            el.dataset.id = item.id
            el.innerText = item.name
            ul.appendChild(el)
            if (ul.getBoundingClientRect().height > height_max) {
                ul.removeChild(ul.lastChild)
                is_truncated = true
                break
            }
        }
        if (is_truncated) {
            const header = ul.previousElementSibling
            const el = document.createElement('label')
            el.innerText = 'show more'
            el.onclick = showMoreClick
            header.appendChild(el)
        }
    })
}

function showMoreClick(event) {
    const label = event.target
    const ul = label.parentElement.nextElementSibling
    const list_item = JSON.parse(ul.dataset.data)
    const to_skip_count = ul.children.length
    let i = 0
    for (let item of list_item) {
        if (i < to_skip_count) {
            i++
        } else {
            const el = document.createElement("li")
            el.dataset.id = item.id
            el.innerText = item.name
            ul.appendChild(el)
        }
    }
    label.innerText = 'show less'
}

function ulClick(event, kind) {
    const target = event.target
    if (target && target.tagName == "LI") {
        const id_selected = target.dataset.id
        const id_current = filters[kind]
        if (id_selected != id_current) {
            filters[kind] = id_selected
            Array.prototype.forEach.call(target.parentElement.children, li => {
                if (li.dataset.id == id_selected) {
                    li.dataset.selected = "true"
                } else {
                    li.dataset.selected = "false"
                }
            })
        } else {
            delete filters[kind]
            target.dataset.selected = "false"
        }
    }
}

function filterClick() {
    const header = document.querySelector('body > header')
    const footer = document.querySelector('body > footer')
    const h1 = document.querySelector('h1')
    const filter = document.getElementById('filter')
    if (filter.dataset.state == "closed") {
        const rect = filter.getBoundingClientRect()
        let cssText = 'top: -' + Math.round(rect.top) + 'px'
        header.style.cssText = cssText
        footer.style.cssText = cssText
        h1.style.cssText = cssText
        filter.style.cssText = cssText + '; max-height: 100vh; height: 100vh'
        setTimeout(function(){
            cssText = cssText + '; display: none'
            header.style.cssText = cssText
            footer.style.cssText = cssText
            h1.style.cssText = cssText
            filter.style.cssText = 'position: static'
            filter.dataset.state = "opened"
        }, 500)
        fetch('/filter-main')
            .then((response) => response.text())
            .then((text) => {
                const main = document.querySelector('#filter > main')
                main.innerHTML = text
                prepareFilterSections()
            })
    } else {
        const cssTop = header.style.cssText.split(';')[0]
        const cssText = cssTop + '; display: block'
        header.style.cssText = cssText
        footer.style.cssText = cssText
        h1.style.cssText = cssText
        filter.style.cssText = cssTop + '; transition: top 0s; position: relative'
        setTimeout(function() {
            header.style.cssText = ''
            footer.style.cssText = ''
            h1.style.cssText = ''
            filter.style.cssText = ''
            filter.dataset.state = "closed"
        }, 1)
    }
}
