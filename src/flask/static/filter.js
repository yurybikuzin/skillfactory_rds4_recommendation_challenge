function filterClick(event) {
    const filter = document.getElementById('filter')
    if (filter.dataset.state == "closed") {
        const footer = document.querySelector('body > footer')
        const header = document.querySelector('body > header')
        const h1 = document.querySelector('h1')
        const rect = filter.getBoundingClientRect()
        {
            header.style.cssText = 'top: -' + Math.round(rect.top) + 'px'
            h1.style.cssText = 'top: -' + Math.round(rect.top) + 'px'
            footer.style.cssText = 'top: -' + Math.round(rect.top) + 'px'
            filter.style.cssText = 'top: -' + Math.round(rect.top) + 'px; max-height: 100vh; height: 100vh'
        }
        let timeoutElapsed = false
        let filterHtml
        setTimeout(function(){
            h1.style.cssText = 'display: none'
            header.style.cssText = 'display: none'
            footer.style.cssText = 'display: none'
            filter.dataset.state = "opened"
            filter.style.cssText = 'position: static'
            timeoutElapsed = true
        }, 500)
        fetch('/filter-content').then((response) => response.text()).then((text) => console.log(text))
    }
}
