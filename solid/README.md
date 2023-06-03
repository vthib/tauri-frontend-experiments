# Tauri + Solid

Simply run `npm install .` then `cargo tauri dev` or `npm run dev` to run it.

# Lessons

I quite like it. The explicitness of having to call a signal to get its value
makes it very clear what is reactive and what isn't. There isn't much magic
and much to learn, everything is built on signals. Integration with Typescript
is top notch as well, with the strict flag enabled by default.

A few pain points that I stumbled upon:

- I often ended up writing stuff like `signal1 == signal2`, which type checks,
  but always returns false. Instead, `signal1() == signal2()` must be used.
  This is a bit annoying as it's not always easy to spot this issue. I wonder
  if a linter already exists to catch it.

- I got a big stack overflow on infinite reactive loop when i was not careful
  enough on how I wrote some reactivity inside a createEffect: the same signal
  was read and wrote to, causing the infinite loop. Extra care is probably
  required when using createEffect.