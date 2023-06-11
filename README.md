# Experiments with Tauri

This repository contains experiments with Tauri and different frontend frameworks:

- Tauri + Svelte + Typescript + Tailwindcss
- Tauri + Solidjs + Typescript + Tailwindcss
- Tauri + Vue (composition) + Typescript + Tailwindcss
- Tauri + Leptos + Tailwindcss

# Description

This contains multiple different implementations of the exact same UI, a very
barebone sysinfo GUI.

The backend is always the same, with some simple APIs provided by tauri to
retrieve information on the system (using the `sysinfo` crate).
The frontend however is written in different frameworks, so that they can be compared.

There are also some other design decisions that are shared by all implementations.

- All implementations must have good type checking. For JS frameworks this means
  `Typescript`, and a good implementation of it. Notably, this excludes `Vue.js`
  with the option API, and if possible, it uses the strictest configuration possible.

- All implementations use `tailwind` for CSS. No particular reason here, just
  wanted to try it, and using the same one for all also allows some comparison.

There are four implementations for the moment, each one lives in the corresponding
folder, with a README file containing some observations.

- `Svelte`
- `Solidjs`
- `Vue`
- `Leptos`

# Design caveats

Please note that the code is not particularly well designed, in fact it's even
intentionally weirdly designed in some places. This is to force some situations
that could occur on a more complex application, and see how to handle them with
each framework.

A few things that are tested:

- A component displaying some "complex" data, ie mostly a derived value from a
  reactive store that is initialized asynchronously. See the `component/Processes`
  file in each framework.
- Modal display, with a child component as a slot of the Modal, see the
  `component/Modal` file.
- Props that are not trivial values, and whether they are properly typed, see `the
  `component/ProcessDetails` file.
- Child that "emits" events that the parent reacts to for its display, see the
  `App` and `components/Sidebar` files.
- Using a dependency, `chart.js`, directly (i.e. not with an already existing wrapper,
  like `svelte-chartjs` or something). See the `components/CpuChart` component.

# Status

This is quite barebone for the moment but functional. The main thing still missing
is testing, so that component testing can be compared.