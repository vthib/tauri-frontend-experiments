# Tauri + Leptos

Simply run `cargo tauri dev` to test.

If updating the code, there are two commands that can be needed, that are outside of tauri's
handling:

- If css classes are added in the code, the css file must be regenerated, with:
  `cd js; npm ci .; npm run css`.
- To modify the chart.js integration, `js/chart.js` can be modified, and then
  `cd js; npm ci .; npm run chart` to regen the generated js file used by leptos.

# Current status

This implementation is still buggy:

- There is no proper cleaning of resources, so the task retrieving data from tauri
  never stop running when the view is changed. This can accumulate when playing around
  different views. Cleaning code is properly used in `leptos::on_cleanup` calls, but
  this is apparently not enough.
- The chart displays strangely when clicking on a process. Not sure why...

# Lessons

This implementation was the most time-consuming and much more painful than the others,
for several reasons:

- Integration with JS code is, obviously, a bit painful, due to the wasm bridge. Integration
  with chart.js is relatively easy and clean (but require the use of an external bundler,
  see the `chart` script in `js/package.json`). But communication with tauri also uses
  a JS API. The `listen` API for example needs a lot of boilerplate code, and is very brittle.

- To make props reactive, they need to be a signal. This is a bit cumbersome for callers,
  as the typing of the signal may not be exactly what he has. The solution to this is to use
  derived signals, which are just functions. So you end up with all props of a component
  by a `Fn() -> T` to keep flexibility for the caller, and the component signature is
  getting quite messy.

- I several times got bugs because I did not wrap code in closures to make it reactive.
  It probably gets better over time however.

- And as detailed above, closures passed to intervals or given to JS-land are never cleaned,
  and I don't understand why. Using `leptos::on_cleanup` on the scope of the component should
  be enough, but it does not work.