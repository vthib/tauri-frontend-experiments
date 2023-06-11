# Tauri + Svelte

Simply run `npm install .` then `cargo tauri dev` or `npm run dev` to run it.

# Lessons

Vue exposes a more Typescript friendly interface with the composition API,
and the use of a few "type designed" helpers.
See <https://vuejs.org/guide/typescript/composition-api.html>.

This is still a bit painful to use though, as the template part is really not
designed for type-checking.

Some of my pain points:

- Vue magic in the template means some distinct behavior between code in the script
  setup and code in the template. For example, this code will work in the script
  setup: `(newValue) => ref.value = newValue`, but it will need to be written as
  `@click="(newValue) = ref = newValue` if in the template... I'm not a fan
  of this.

- The template is missing some glaring type checks. If I fail to import a Component,
  but still use `<Component ...>` in the template... nothing is reported. Only at
  runtime will there be warnings that a component is unknown. I haven't found how
  to actually have a type error on this! I suppose it works like this to allow
  for using global components that are registered, but to me, this is just
  unacceptable behavior. Miss an import and your app is broken without any
  type errors.

- Emits can be typed in the emitter component, but are completely untyped in the
  parent, not great.

- The CpuChart wrapper actually does not work with `ref`, and must use `shallowRef`,
  otherwise it completely breaks the chart.js dependency. Makes sense in retrospect,
  but lost a lot of time before finding the solution, and I can see how this can be
  tricky to think about. I feel like always using `shallowRef` as the default, and
  using `ref` (or I would call it `deepRef`) only when needed, would be much a
  much better default.