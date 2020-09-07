# Страницы веб-сервиса и маршруты переходов

<!-- vim-markdown-toc Redcarpet -->

* [root](#root)
    * [root to cart](#root-to-cart)
    * [root to filters](#root-to-filters)
    * [root to item](#root-to-item)
    * [root to root-next](#root-to-root-next)
* [cart](#cart)
    * [Пустая корзина](#пустая-корзина)
    * [cart to root](#cart-to-root)
    * [Непустая корзина](#непустая-корзина)
    * [cart to root](#cart-to-root)
    * [cart to item](#cart-to-item)
* [filters](#filters)
    * [filters to root](#filters-to-root)
* [item](#item)
    * [item to root](#item-to-root)
    * [Если товара еще нет в корзине](#если-товара-еще-нет-в-корзине)
        * [non-cart-item to root](#non-cart-item-to-root)
    * [Если товар уже есть в корзине](#если-товар-уже-есть-в-корзине)
        * [cart-item to cart](#cart-item-to-cart)
    * [item to cart](#item-to-cart)
    * [item to reviews](#item-to-reviews)
    * [item to item](#item-to-item)
* [reviews](#reviews)
    * [reviews to item](#reviews-to-item)
    * [reviews to reviews-next](#reviews-to-reviews-next)

<!-- vim-markdown-toc -->

## root

root - Главная страница - список всех товаров для выбора

<kbd><img width="250px" src="/assets/root.png"></kbd>

### root to cart

to [cart](#cart) - в корзину выбранных товаров. При нажатии на иконку корзины в правом верхнем углу

### root to filters

to [filters](#filters) - на страницу параметров сортировки/фильрации. При нажатии на кнопку `Filters`

### root to item

to [item](#item) - на страницу карточки товара. При нажатии на любую точку в прямоугольнике карточки товара

### root to root-next

to [root](#root) - на следующую страницу списка товаров для выбора. При нажатии на кнопку `Next` на самом последнем скролле страницы

<kbd><img width="250px" src="/assets/root-next.png"></kbd>



## cart

cart - Корзина

### Пустая корзина

<kbd><img width="250px" src="/assets/cart.png"></kbd>

### cart to root

to [root](#root) - на страницу списка товаров для выбора. При нажатии иконку веб-сервиса в верхнем левом углу 

### Непустая корзина

<kbd><img width="250px" src="/assets/cart-non-empty.png"></kbd>

### cart to root

to [root](#root) - на страницу списка товаров для выбора. При нажатии иконку веб-сервиса в верхнем левом углу 

### cart to item

to [item](#item) - на страницу карточки товара. При нажатии на любую точку в прямоугольнике карточки товара


## filters

filters - Страница параметров сортировки/фильтрации списка товаров для выбора

<kbd><img width="250px" src="/assets/filters.png"></kbd>

### filters to root

- [root](#root) - Главная страница списка товаров, удовлетворяющих установленным параметрам сортировки/фильтрации. При нажатии на кнопку `Filters` или кнопку `Done`



## item

item - Страница карточки товара

<kbd><img width="250px" src="/assets/item.png"></kbd>

### item to root

to [root](#root) - на страницу списка товаров для выбора. При нажатии иконку веб-сервиса в верхнем левом углу 


### Если товара еще нет в корзине

#### non-cart-item to root

to [root](#root) - на страницу списка товаров для выбора. При нажатии на кнопку `Add to Cart`

<kbd><img width="250px" src="/assets/add-to-cart.png"></kbd>

### Если товар уже есть в корзине

#### cart-item to cart

to [cart](#cart) - на страницу списка товаров для выбора. При нажатии на кнопку `Remove from Cart`

<kbd><img width="250px" src="/assets/remove-from-cart.png"></kbd>

### item to cart

to [cart](#cart-empty) - в корзину выбранных товаров. При нажатии на иконку корзины в правом верхнем углу

### item to reviews

to [reviews](#reviews) - на страницу списка отзывов о товаре. При нажатии на строку с оценкой товара и количеством отзов ("21 reviews")

### item to item

to [item](#item) - на страницу карточки рекомендованного товара. При нажатии на любую точку в прямоугольнике карточки товара в списке рекомендаций

<kbd><img width="250px" src="/assets/we-recommend.png"></kbd>


## reviews

reviews - Страницы отзывов на товарa

<kbd><img width="250px" src="/assets/reviews.png"></kbd>

### reviews to item

to [item](#item) - на страницу карточки товара. При нажатии на серую плашку под черным заголовком страницы


### reviews to reviews-next

to [reviews](#reviews) - на следующую страницу списка товаров для выбора. При нажатии на кнопку `Next` на самом последнем скролле страницы

<kbd><img width="250px" src="/assets/reviews-next.png"></kbd>




