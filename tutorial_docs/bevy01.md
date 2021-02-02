
Making Shooting Game With Bevy(1)

Rust is becoming a imortant programming language recently. Many big companies, including Microsoft/Google, is switching their using language to Rust because of its safe-memory-management. 

But for me, it was hard to find where to use Rust. I'm a mobile developer, but Rust is not suitable for Android/iOS/Web programming (Wasm is a only place to use Rust). This fact discouraged me from learning the language.

After while, I found some game engines written in Rust. Yeah, I like video games(I suppose many programmers like it), and I decided to make a simple game.

In this series of articles, I will use "Bevy", which is one of popular game engines in Rust, and make a `Geometry Wars` like shooting game, Step by Step.

<!-- more -->

## Today's Goal

* Create a window
* Show one texture
* Move it!

## Bevy

> Notice: This article uses **Bevy 0.4**. There is a possibility that following code won't work in the future.

Bevy is 2D and 3D game engine (Like Unity). Before I choose Bevy, I compared some game engines.

* [Amethyst](https://amethyst.rs/)
  * Great documents (compared to bevy)
  * But too large for me and it causes greater build time.
  * 6.7k stars in Github
* [Ggez](https://ggez.rs/)
  * Very simple but need to draw objects your own.
  * 2.7k stars
* [Bevy]
  * Similar to Amethyst, but lighter.
  * Newer than others
  * Less documents
  * 6.5k stars

The reason I choose Bevy was bevy's ECS-structure and fast build time. Many Github-Stars was also the reason.


## Setup bevy

To use bevy is simple and same as other rust framework.  Only to add dependencies in `Cargo.toml`.

```toml
[dependencies]
bevy = "0.4"
```

If you use windows 10, you need to install [VS2019 build tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)  (I forgot this process at first!).

Please check [the official setup guide](https://bevyengine.org/learn/book/getting-started/setup/) for more details.


## Show window

The learning materials that was created by bevy is not enough. Not only the official documents is not enough, but also some of their samples in the github won'to work.

This causes me to introduce bevy setup step-by-step.

First, you need to create a window to draw your game objects. It's easy, only a few lines.

```rust
using bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .run();
}
```











