# What is Axum?

[`axum`] is a **modular** web framework built with [`tokio`], [`tower`] and [`hyper`] written in [rust] by the [tokio team].

Some history: This framework is named after a [weird obelisk](https://en.wikipedia.org/wiki/Obelisk_of_Axum) built in ancient times in Ethiopia. [davidpdrsn] decided to name it like this because "axum" sounds cool and that obelisk looks like a [`tower`].

# Why use Axum?

You might think, oh this is one of those regular rust frameworks with some improvements. **Nope** it is not just another framework. [`axum`] might not be the most convenient of all frameworks out there, but it is super modular(you can use it in many places however you want) and built upon shoulders of *giants*.

With [`axum`] you get awesome async utilities of [`tokio`], you get [performance](https://github.com/programatik29/rust-web-benchmarks) of [`hyper`] and you get web service utilities of [`tower`]. All those *giant* libraries are used in production, well tested and popular.

To help you understand why those will be important in future, let me explain like this. Think of *some* easy frameworks out there like sugar, they will give you energy temporarily and make you do awesome stuff in short time. In long term you will be fat and have a hard time doing unusual stuff. [`axum`] is like the real meat. You might have a harder time doing awesome stuff in short time but as you learn, understand and do stuff, you will end up being a stronger and more capable web developer. 

More advanced explanation would be:
When you do common stuff, *any* framework would do. As your needs start to get complicated then you might be trapped inside that framework's ecosystem. The worst case would be requiring modification of source code which is really labor intensive and you might end up with nothing without dedicating weeks. [`axum`] has a developer friendly open ecosystem meaning random dudes can create random crates that solve random problems, spirit of open source yay! But that is a promise of future, currently [`axum`] can use features of [`hyper`], [`tower`] and [`tokio`].

# Lets Start

Lets start learning with basic stuff.

*Hello World Example Link*

[`axum`]: https://github.com/tokio-rs/axum
[`tokio`]: https://github.com/tokio-rs/tokio
[`tower`]: https://github.com/tower-rs/tower
[`hyper`]: https://github.com/hyperium/hyper
[rust]: https://www.rust-lang.org/
[tokio team]: https://github.com/tokio-rs
[davidpdrsn]: https://github.com/davidpdrsn
