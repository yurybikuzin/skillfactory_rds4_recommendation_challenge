function filterClick() {
    const header = document.querySelector('body > header')
    const footer = document.querySelector('body > footer')
    const h1 = document.querySelector('h1')
    const filter = document.getElementById('filter')
    if (filter.dataset.state == "closed") {
        const rect = filter.getBoundingClientRect()
        let cssText = 'top: -' + Math.round(rect.top) + 'px'
        console.log(cssText)
        header.style.cssText = cssText
        footer.style.cssText = cssText
        h1.style.cssText = cssText
        filter.style.cssText = cssText + '; max-height: 100vh; height: 100vh'
        let timeoutElapsed = false
        let filterHtml
        setTimeout(function(){
            cssText = cssText + '; display: none'
            header.style.cssText = cssText
            footer.style.cssText = cssText
            h1.style.cssText = cssText
            filter.style.cssText = 'position: static'
            filter.dataset.state = "opened"
            timeoutElapsed = true
        }, 500)
        fetch('/filter-content').then((response) => response.text()).then((text) => console.log(text))
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
