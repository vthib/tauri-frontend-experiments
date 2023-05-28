/* @refresh reload */
import { render } from "solid-js/web";

import "./styles.postcss";
import App from "./App";

render(() => <App />, document.getElementById("root") as HTMLElement);
