# Tauri + Svelte

Simply run `npm install .` then `cargo tauri dev` or `npm run dev` to run it.

# Lessons

Svelte is pretty straightforward and simple to use. However, since svelte is
compiled magic on custom syntax, anything outside of simple cases can be a bit
more annoying to write, and non-simple stuff can feel scary as one wonder whether
it will be properly handled by the magic. This probably gets much better when
used to it, but it is a design choice that is probably hit or miss (and more on
the side of miss on my side for the moment).

Some of my pain points:

- Async helpers are hit or miss. There are the #await, Suspense, ... helpers, but any
  other case is much more painful, as for example "onMount" cannot be used after an
  await, and other cases like this.

- As noted above, magic is easy in many cases, but start to get scary or in the way
  on more complex cases. When getting used to it it's probably quite comfortable, but
  i'm not sure i really like it (the $: syntax for example, i quite dislike).

- Typescript integration works, but it's not that great. It depends on a svelte plugin,
  and I for example tried to set "strict: true" in tsconfig, which completely broke
  this svelte plugin. Not everything is magically typed as well, events are not, and
  more complex props needs even more custom syntax/svelte magic.