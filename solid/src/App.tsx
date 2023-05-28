import { Match, Switch, createSignal } from "solid-js";
import Processes from "./components/Processes";
import Resources from "./components/Resources";
import Sidebar from "./components/Sidebar";

export default function App() {
  const [active, setActive] = createSignal("processes");

  return (
    <main class="container mx-auto">
      <Sidebar onActive={setActive} />
      <div class="ml-20 p-4">
        <Switch>
          <Match when={active() == "processes"}>
            <Processes />
          </Match>
          <Match when={active() == "resources"}>
            <Resources />
          </Match>
        </Switch>
      </div>
    </main>
  );
}
