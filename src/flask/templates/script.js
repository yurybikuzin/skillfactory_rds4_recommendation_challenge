
function labelClick(event, kind) {
    if (event.target.tagName == "LABEL") {
        if (event.target.dataset.selected == "true") {
            event.target.dataset.selected = "false"
            el = document.getElementById(kind + '-' + event.target.dataset.id); 
            el.checked = false;
            if (kind == 'brand') {
                selected_list_brand.remove(parseInt(event.target.dataset.id))
            }
        } else {
            event.target.dataset.selected = "true"
            el = document.getElementById(kind + '-' + event.target.dataset.id); 
            el.checked = true;
            if (kind == 'brand') {
                selected_list_brand.add(parseInt(event.target.dataset.id))
            }
        }
        console.log(event.target.dataset.selected)
    }
}
const selected_list_brand = new Set([
    {% for item in filter.selected_list_brand %}
        {{- item -}},
    {% endfor %}
])
let list_brand_initial_length = 0
const list_brand = [
    {% for item in filter.list_brand() %}
        {%- if item.count < 150 -%}
        { 
            "id": {{ item.id | tojson }},
            "name": {{ item.name | tojson }},
        },
        {%- endif -%}
    {% endfor %}
]
function showMoreClick(event, kind) {
    const target = event.target.tagName == "LABEL" ? event.target : event.target.parentElement
    const ul = document.querySelector('.filter-section.brand ul')
    if (target.dataset.selected == "show-less") {
        list_brand_initial_length = ul.childElementCount
        for (let item of list_brand) {
            const li = document.createElement("li");
            li.innerHTML = "<label data-id=" + item.id + (selected_list_brand.has(item.id) ? ' data-selected="true"' : "") + ">" + item.name + "</label>"
            ul.appendChild(li);
        }
        target.dataset.selected = "show-more"
    } else {
        while (ul.childElementCount > list_brand_initial_length) {
            ul.removeChild(ul.lastChild)
        }
        target.dataset.selected = "show-less"
    }
}
