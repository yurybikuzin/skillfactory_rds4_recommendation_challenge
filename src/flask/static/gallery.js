elem.onload = function() {
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
        const miniImgWidth = parseInt(getComputedStyle(elem).getPropertyValue('--mini-width'))
        const miniTotalWidth = len * miniImgWidth
        elem.style.setProperty('--wrapper-mini-width', miniTotalWidth + 'px')
        elem.innerHTML =
            '<div class="main">' + 
                listUrl.map(url => '<img src="' + url + '"/>').join('') +
            '</div>' + 
            '<div class="mini">' + 
                listUrl.map((url, i) => '<img data-i="' + i + '" src="' + url + '_' + '78x52' + '"/>').join('') +
            '</div>' + 
            '<div class="left arrow"><div><div></div></div></div>' + 
            '<div class="right arrow"><div><div></div></div></div>' + 
            '<div class="zoom"></div>' + 
            '<div class="counter"><div>1/' + len + '</div></div>' + 
            ''
        const main = elem.firstChild
        const mini = main.nextElementSibling
        for (const i of mini.children) {
            i.addEventListener(eventNames[4], event => {
                mainStyle.cssText = "left:-" + event.target.dataset.i + "00%;"
            })
        }
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
        const miniStyle = mini.style
        let target 
        const targetMain = 1
        const targetMini = 2
        const getTarget = clientY => 
            clientY <= elem.firstChild.getBoundingClientRect().bottom ?  
                targetMain : // main
                targetMini   // mini
        for (let j = 0; j < eventNames.length; j++) {
            elem.addEventListener(eventNames[j], event => {
                if (j < 2) {// touchmove | touchstart
                    j && (touchPrevX = touchLastX)// touchmove
                    const touch = event.touches[0]
                    touchLastX = touch.clientX
                    if (!j) { // touchstart
                        target = getTarget(touch.clientY)
                        if (target == targetMini) {
                            startLeft = parseInt(miniStyle.left) || 0
                            // miniStyle.transition = 'left: 0s'
                        }
                        console.log({target, startLeft})
                    }
                    j || (touchPrevX = touchStartX = touchLastX) // touchstart
                }
                const delta = round(touchLastX - touchStartX)
                const clientWidth = elem.clientWidth
                if (j > 2) { // touchend | click
                    if (j == 4) target = getTarget(event.clientY)
                    const direction = j > 3 ?  // click
                        Math.floor(3 * (event.clientX - elem.getBoundingClientRect().left) / clientWidth) - 1 : 
                        // touchend
                        !target || delta * (touchLastX - touchPrevX) < 0 ? 0 : -delta  
                    if (j > 3 && target == targetMini) return
                    i += 
                        direction < 0 && i > 0 ? -1 : 
                        direction > 0 && i < len - 1 ? 1 : 
                        0
                    counter.innerText = (i + 1) + '/' + len
                    updateArrows()
                }
                if (target == targetMain) {
                    let cssText = j ? '' : mainStyle.cssText
                    j == 1 && (cssText += "left:" + (-i * clientWidth + delta) + "px")// touchmove
                    mainStyle.cssText = cssText + (j < 2 ? ";transition:left 0s;" : "left:-" + i + "00%;")
                } else {
                    let cssText = j ? '' : miniStyle.cssText
                    j == 1 && (cssText += "left:" + (startLeft + delta) + "px")// touchmove
                    console.log({j, startLeft, delta, miniTotalWidth, clientWidth}, miniTotalWidth - clientWidth, cssText + (j < 2 ? ";transition:left 0s;" : "left:" + (startLeft + delta) + "px;"))
                    miniStyle.cssText = cssText + (j < 2 ? ";transition:left 0s;" : "left:" + Math.min(0, Math.max(clientWidth - miniTotalWidth, startLeft + delta)) + "px;")
                }
            })
        }
    }
}

};document.head.prepend(elem)

