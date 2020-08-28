let filter = { }

let filter_original = { }

function prepareFilterSections() {
    const list_ul = document.querySelectorAll("#filter > main > section > ul[data-data]")
    list_ul.forEach(ul => {
        const list_item = JSON.parse(ul.dataset.data)
        const height_client = document.documentElement.clientHeight
        const height_max = height_client * parseFloat(ul.dataset.heightMax)

        if (ul.dataset.selected) {
            let json = JSON.parse(list_item.dataset.selected)
            const name = list_item.dataset.name
            if (Array.isArray(json)) {
                filter[name] = new Set(json)
                filter_original[name] = new Set(json)
            } else {
                filter[name] = json
                filter_original[name] = json
            }
        }

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
            el.dataset.state = 'less'
            el.innerText = 'show more'
            el.onclick = showMoreClick
            header.appendChild(el)
        }
    })
}

function showMoreClick(event) {
    const label = event.target
    const ul = label.parentElement.nextElementSibling
    if (label.dataset.state == 'less') {
        const list_item = JSON.parse(ul.dataset.data)
        const skip_count = ul.children.length
        let i = 0
        for (let item of list_item) {
            if (i < skip_count) {
                i++
            } else {
                const el = document.createElement("li")
                el.dataset.id = item.id
                el.innerText = item.name
                ul.appendChild(el)
            }
        }
        label.dataset.lessCount = skip_count
        label.dataset.state = 'more'
        label.innerText = 'show less'
        const section = ul.parentElement
        const main = section.parentElement
        const footer = main.nextElementSibling
        const height_max = footer.offsetTop - main.offsetTop - 48
        if (ul.offsetHeight > height_max) {
            ul.style.cssText = 'height: ' + height_max + 'px; overflow-y: scroll'
        }
        main.scrollTo({left: 0, top: section.offsetTop - main.offsetTop - 4, behavior: 'smooth'})
    } else {
        let less_count = label.dataset.lessCount
        while (ul.children.length > less_count) {
            ul.removeChild(ul.lastChild)
        }
        label.dataset.state = 'less'
        label.innerText = 'show more'
        if (ul.style.cssText != '') {
            ul.style.cssText = ''
        }
    }
}

function ulClick(event, name) {
    if (event.target.tagName == "LI") {
        const li = event.target
        const ul = li.parentElement
        const id_selected = li.dataset.id
        if (ul.dataset.type == 'multi') {
            const id_current = filter[name]
            if (id_current && id_current.has(id_selected)) {
                li.dataset.selected = "false"
                filter[name].delete(id_selected)
            } else {
                li.dataset.selected = "true"
                if (id_current) {
                    filter[name].add(id_selected)
                } else {
                    filter[name] = new Set([id_selected])
                }
            }
        } else {
            const id_current = filter[name]
            if (id_selected != id_current) {
                filter[name] = id_selected
                Array.prototype.forEach.call(ul.children, li => {
                    if (li.dataset.id == id_selected) {
                        li.dataset.selected = "true"
                    } else {
                        li.dataset.selected = "false"
                    }
                })
            } else {
                li.dataset.selected = "false"
                delete filter[name]
            }
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
            console.log('send request', filter)
        }, 1)
    }
}
