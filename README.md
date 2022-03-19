#Aliexpress card converter

###Get Started
```py
git clone https://github.com/chifmaster113/Aliexpress-Card-Converter.git
cargo build 
cargo run
```

###Examples:

I have [**mifare**](https://en.wikipedia.org/wiki/MIFARE) ``13.56 MHz`` aliexpress card,
turnstiles from [**enkora**](https://enkora.fi/fi/) and mifare [**card reader**](https://www.aliexpress.com/item/4001044670681.html?spm=a2g0o.productlist.0.0.424150f3BgINVg&algo_pvid=474affd6-17f1-42bf-bdaf-a5873d85c17c&algo_exp_id=474affd6-17f1-42bf-bdaf-a5873d85c17c-2&pdp_ext_f=%7B%22sku_id%22%3A%2210000013722961158%22%7D&pdp_pi=-1%3B10.2%3B-1%3B-1%40salePrice%3BEUR%3Bsearch-mainSearch)


``
cardID = 3500693297
``

``
Enkora response = 827828432
``

So, we have response that we can manually convert to [**hexadecimal**](https://www.rapidtables.com/convert/number/decimal-to-hex.html)
and we will get this hex ``3157A8D0``, this hex is reversed version that card id has,
and the card reader accept only **decimal digits** like **cardID**.
This is why we need convert response to hex that card reader will be understood.




