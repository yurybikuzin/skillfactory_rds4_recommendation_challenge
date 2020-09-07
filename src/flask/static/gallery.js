// elem.onload = 
(function() {
'use strict'

for (const elem of document.querySelectorAll('.gallery')) {
    const dataset = elem.dataset
    const listUrl = JSON.parse(dataset.listUrl)
    const len = listUrl.length
    if (len) {
        const round = Math.round
        const eventNames = ['start', 'move', 'cancel', 'end'].map(s => 'touch' + s).concat('click')
        const de = document.documentElement
        const dim = Math.max(de.clientWidth, de.clientHeight)
        const size = round((dim - 640) / (1024 - 640)) ? '1024x768' : '640x480' 
        elem.style.setProperty('--image-width', (round(100000 / len) / 1000) + '%')
        elem.style.setProperty('--wrapper-main-width', len + '00%')
        elem.innerHTML =
            '<div class="main">' + 
                listUrl.map(url => '<img src="' + url + '"/>').join('') +
            '</div>' + 
            '<div class="left arrow"><div><div></div></div></div>' + 
            '<div class="right arrow"><div><div></div></div></div>' + 
            '<div class="zoom"></div>' + 
            '<div class="counter"><div>1/' + len + '</div></div>' + 
            ''
        const main = elem.firstChild
        let counter = elem.lastChild
        const zoom = counter.previousElementSibling
        counter = counter.children[0]
        zoom.addEventListener(eventNames[4], event => {
            if (dataset.z != null) {
                delete dataset.z
            } else {
                dataset.z = ""
            }
            event.stopPropagation()
        })
        const rightArrow = zoom.previousElementSibling
        const leftArrow = rightArrow.previousElementSibling
        const updateArrows = () => {
            leftArrow.style.display = i ? 'block' : 'none'
            rightArrow.style.display = i < len - 1 ? 'block' : 'none'
        }
        let i = 0, touchPrevX, touchLastX, touchStartX, startLeft
        updateArrows()
        const mainStyle = main.style
        for (let j = 0; j < eventNames.length; j++) {
            elem.addEventListener(eventNames[j], event => {
                if (j < 2) {// touchmove | touchstart
                    j && (touchPrevX = touchLastX)// touchmove
                    const touch = event.touches[0]
                    touchLastX = touch.clientX
                    j || (touchPrevX = touchStartX = touchLastX) // touchstart
                }
                const delta = round(touchLastX - touchStartX)
                const clientWidth = elem.clientWidth
                if (j > 2) { // touchend | click
                    const direction = j > 3 ?  // click
                        Math.floor(3 * (event.clientX - elem.getBoundingClientRect().left) / clientWidth) - 1 : 
                        // touchend
                        delta * (touchLastX - touchPrevX) < 0 ? 0 : -delta  
                    i += 
                        direction < 0 && i > 0 ? -1 : 
                        direction > 0 && i < len - 1 ? 1 : 
                        0
                    counter.innerText = (i + 1) + '/' + len
                    updateArrows()
                }
                let cssText = j ? '' : mainStyle.cssText
                j == 1 && (cssText += "left:" + (-i * clientWidth + delta) + "px")// touchmove
                mainStyle.cssText = cssText + (j < 2 ? ";transition:left 0s;" : "left:-" + i + "00%;")
            })
        }
    }
}

})()

