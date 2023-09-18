Rust in the Frontend
====================

Why would you do this
---------------------

- Rust is really fast
- Rust WASM tooling is mature, perhaps the most mature of any ecosystem
- WASM is really fast
- Rust can prove "correctness" at runtime

What are my options
-------------------

Actually, there are a lot of options, I'm going to run quickly through a few of my favourites

### Yew

- Yew is a lot like React
- Components are essentially html
- Uses a Virtual DOM

### Perseus (Sycamore)

### Dioxus

### Comparison

- Yew is slower than React
- Sycamore is faster
- Dioxus is faster still
- Speed isn't everything
- No HTML copy-paste in Perseus and Dioxus

Wait, should I even do this
----------------------

- Probably not
- Speed isn't everything
- Speed differences are small
- Nothing is stable (yet)
- But, it depends:
  - These speeds are for rendering
  - big calculations in the FE that don't need to manipulate the dom are where WASM and Rust excel
- Rust, like JavaScript, is isomorphic
- Rust is more predictable that TypeScript

How do I learn more
-------------------

- This presentation was written in Yew
- This presentation was written in Perseus*
- This presentation was written in Dioxus
